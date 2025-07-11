mod prompt_template;
mod response_parser;
mod model;
mod client;

use std::collections::HashMap;
use tokio::sync::RwLock;
use eyre::Result;
use tracing::debug;

pub use model::Model;
use client::ToolSelectionClient;
use crate::mcp_client::Module;
use crate::api_client::ApiClient;

/// Information about a module and its server
pub struct ModuleServerInfo {
    /// The module
    pub module: Module,
    /// The server name
    pub server_name: String,
    /// Optional server instructions
    pub server_instructions: Option<String>,
}

/// Represents a tool selector that dynamically selects tools based on user queries
pub struct ToolSelector<'a> {
    /// The model to use for tool selection
    model: Model,
    /// Cache of selection results
    cache: RwLock<HashMap<String, Vec<String>>>,
    /// Reference to the API client for making LLM calls
    api_client: &'a ApiClient,
}

impl<'a> ToolSelector<'a> {
    /// Creates a new tool selector with the specified model and API client
    pub fn new(model: Model, api_client: &'a ApiClient) -> Self {
        Self {
            model,
            cache: RwLock::new(HashMap::new()),
            api_client,
        }
    }
    
    /// Creates a new tool selector with the default model (Claude 3.5 Sonnet)
    pub fn with_default_model(api_client: &'a ApiClient) -> Self {
        Self::new(Model::Claude35Sonnet, api_client)
    }

    /// Selects tools based on the user query and available modules with server info
    pub async fn select_tools(
        &self,
        query: &str,
        modules_info: &[ModuleServerInfo],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        // Check cache for similar queries
        if let Some(cached_tools) = self.check_cache(query).await {
            debug!("Using cached tool selection for query: {}", query);
            return Ok(cached_tools);
        }

        // Use the model to select tools
        let selected_tools = self.select_tools_with_model(query, modules_info, conversation_context).await?;
        
        // Cache the result
        self.update_cache(query.to_string(), selected_tools.clone()).await;
        
        
        Ok(selected_tools)
    }

    /// Checks the cache for similar queries
    async fn check_cache(&self, query: &str) -> Option<Vec<String>> {
        let cache = self.cache.read().await;
        
        // For now, we only do exact matches
        // In the future, we could implement fuzzy matching or semantic similarity
        cache.get(query).cloned()
    }

    /// Updates the cache with a new query and selected tools
    async fn update_cache(&self, query: String, selected_tools: Vec<String>) {
        let mut cache = self.cache.write().await;
        cache.insert(query, selected_tools);
        
        // Limit cache size to prevent memory issues
        if cache.len() > 100 {
            // Remove oldest entries (simple implementation)
            // In the future, we could use LRU cache or other strategies
            let keys_to_remove: Vec<String> = cache.keys()
                .take(cache.len() - 100)
                .cloned()
                .collect();
            
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
    }

    /// Selects tools using the model
    async fn select_tools_with_model(
        &self,
        query: &str,
        modules_info: &[ModuleServerInfo],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        // Generate the prompt for tool selection
        let modules_with_server_info: Vec<prompt_template::ModuleWithServerInfo> = modules_info
            .iter()
            .map(|info| prompt_template::ModuleWithServerInfo {
                module: &info.module,
                server_name: &info.server_name,
                server_instructions: info.server_instructions.as_deref(),
            })
            .collect();
            
        let prompt = prompt_template::generate_tool_selection_prompt(
            query, 
            &modules_with_server_info, 
            conversation_context
        );
        
        debug!("Selecting tools for query: {}", query);
        debug!("Available modules: {}", modules_info.len());
        
        // Try to use the LLM for tool selection
        let tool_selection_client = ToolSelectionClient::new(self.api_client, self.model);
        match tool_selection_client.select_tools(prompt).await {
            Ok(selections) => {
                debug!("LLM tool selection completed successfully");
                
                let mut selected_tools = Vec::new();
                for selection in selections {
                    if selection.relevance_score >= 5 {
                        // Find the module and add its tools
                        if let Some(info) = modules_info.iter().find(|info| 
                            info.module.name == selection.module_name && 
                            info.server_name == selection.server_name
                        ) {
                            debug!(
                                "Selected module {} from server {} with score {} (reasoning: {})", 
                                selection.module_name, 
                                selection.server_name, 
                                selection.relevance_score,
                                selection.reasoning
                            );
                            selected_tools.extend(info.module.tools.clone());
                        }
                    }
                }
                
                if selected_tools.is_empty() {
                    debug!("No tools selected by LLM based on relevance scoring");
                } else {
                    debug!("LLM selected {} tools from relevant modules", selected_tools.len());
                }
                
                Ok(selected_tools)
            }
            Err(e) => {
                // Fall back to heuristic implementation
                debug!("LLM tool selection failed, falling back to heuristic: {}", e);
                
                // Enhanced heuristic implementation
                let query_lower = query.to_lowercase();
                let mut module_scores: Vec<(String, Vec<String>, f32, bool)> = Vec::new();
                
                for info in modules_info {
                    let module_name_lower = info.module.name.to_lowercase();
                    let module_desc_lower = info.module.description.to_lowercase();
                    let mut score = 0.0;
                    let has_priority_instruction = info.server_instructions
                        .as_ref()
                        .map(|inst| inst.to_lowercase().contains("prefer this") || 
                                   inst.to_lowercase().contains("priority"))
                        .unwrap_or(false);
                    
                    // Check for direct keyword matches in module name or description
                    for word in query_lower.split_whitespace() {
                        if module_name_lower.contains(word) {
                            score += 3.0; // High score for name match
                        }
                        if module_desc_lower.contains(word) {
                            score += 2.0; // Medium score for description match
                        }
                        
                        // Check tool names
                        for tool in &info.module.tools {
                            if tool.to_lowercase().contains(word) {
                                score += 2.5; // High score for tool name match
                            }
                        }
                    }
                    
                    // Special handling for AWS services
                    if query_lower.contains("s3") || query_lower.contains("bucket") {
                        if module_name_lower.contains("s3") || module_desc_lower.contains("s3") || 
                           module_desc_lower.contains("storage") || module_desc_lower.contains("bucket") {
                            score += 5.0; // Very high score for S3-related modules
                        }
                    }
                    
                    if query_lower.contains("ec2") || query_lower.contains("instance") {
                        if module_name_lower.contains("ec2") || module_desc_lower.contains("ec2") || 
                           module_desc_lower.contains("instance") || module_desc_lower.contains("virtual machine") {
                            score += 5.0; // Very high score for EC2-related modules
                        }
                    }
                    
                    if query_lower.contains("lambda") {
                        if module_name_lower.contains("lambda") || module_desc_lower.contains("lambda") || 
                           module_desc_lower.contains("serverless") {
                            score += 5.0; // Very high score for Lambda-related modules
                        }
                    }
                    
                    // Apply server instruction bonus
                    if has_priority_instruction && score > 0.0 {
                        score *= 1.5; // 50% bonus for servers with priority instructions
                        debug!("Applied priority bonus to {} (server has priority instructions)", info.module.name);
                    }
                    
                    if score > 0.0 {
                        module_scores.push((info.module.name.clone(), info.module.tools.clone(), score, has_priority_instruction));
                    }
                }
                
                // Sort by score (highest first) and select modules with score >= 5.0 (threshold)
                module_scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
                
                let mut selected_tools = Vec::new();
                for (module_name, tools, score, has_priority) in module_scores {
                    if score >= 5.0 { // Relevance threshold
                        debug!("Selected module {} with score {} (priority: {})", module_name, score, has_priority);
                        selected_tools.extend(tools);
                    } else {
                        debug!("Skipped module {} with score {} (below threshold)", module_name, score);
                    }
                }
                
                if selected_tools.is_empty() {
                    debug!("No tools selected based on heuristic relevance scoring");
                } else {
                    debug!("Heuristic selected {} tools from relevant modules", selected_tools.len());
                }
                
                Ok(selected_tools)
            }
        }
    }

    /// Forces re-selection of tools, bypassing the cache
    pub async fn force_select_tools(
        &self,
        query: &str,
        modules_info: &[ModuleServerInfo],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        let selected_tools = self.select_tools_with_model(query, modules_info, conversation_context).await?;
        self.update_cache(query.to_string(), selected_tools.clone()).await;
        Ok(selected_tools)
    }

    /// Clears the tool selection cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

// Tests are disabled until we can provide a mock API client
// #[cfg(test)]
// mod tests {
//     use super::*;
//     
//     #[tokio::test]
//     async fn test_tool_selector_caching() {
//         // TODO: Implement with mock API client
//     }
//     
//     #[tokio::test]
//     async fn test_tool_selection_relevance() {
//         // TODO: Implement with mock API client
//     }
// }
