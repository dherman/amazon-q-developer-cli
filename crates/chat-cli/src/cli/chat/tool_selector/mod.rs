mod prompt_template;
mod response_parser;
mod model;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use eyre::Result;
use tracing::{debug, warn};

pub use model::Model;
use crate::mcp_client::Module;
use crate::cli::chat::tools::ToolSpec;

/// Represents a tool selector that dynamically selects tools based on user queries
pub struct ToolSelector {
    /// The model to use for tool selection
    model: Model,
    /// Cache of selection results
    cache: RwLock<HashMap<String, Vec<String>>>,
}

impl ToolSelector {
    /// Creates a new tool selector with the specified model
    pub fn new(model: Model) -> Self {
        Self {
            model,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Selects tools based on the user query and available modules
    pub async fn select_tools(
        &self,
        query: &str,
        modules: &[Module],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        // Check cache for similar queries
        if let Some(cached_tools) = self.check_cache(query).await {
            debug!("Using cached tool selection for query: {}", query);
            return Ok(cached_tools);
        }

        // Use the model to select tools
        let selected_tools = self.select_tools_with_model(query, modules, conversation_context).await?;
        
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
        modules: &[Module],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        // Generate the prompt for tool selection
        let _prompt = prompt_template::generate_tool_selection_prompt(query, modules, conversation_context);
        
        // TODO: Implement actual model-based selection using the prompt
        // For now, return a placeholder implementation that simulates the model's response
        
        debug!("Selecting tools for query: {}", query);
        debug!("Available modules: {}", modules.len());
        
        // In a real implementation, we would:
        // 1. Send the prompt to the model
        // 2. Parse the response using response_parser
        // 3. Filter modules based on relevance score
        // 4. Return tools from selected modules
        
        // For now, return all tools from all modules
        // This ensures that tools are visible in the UI
        let mut selected_tools = Vec::new();
        for module in modules {
            debug!("Including all tools from module: {}", module.name);
            for tool in &module.tools {
                debug!("Adding tool: {}", tool);
                selected_tools.push(tool.clone());
            }
        }
        
        debug!("Selected {} tools", selected_tools.len());
        
        Ok(selected_tools)
    }

    /// Forces re-selection of tools, bypassing the cache
    pub async fn force_select_tools(
        &self,
        query: &str,
        modules: &[Module],
        conversation_context: &str,
    ) -> Result<Vec<String>> {
        let selected_tools = self.select_tools_with_model(query, modules, conversation_context).await?;
        self.update_cache(query.to_string(), selected_tools.clone()).await;
        Ok(selected_tools)
    }

    /// Clears the tool selection cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tool_selector_caching() {
        let selector = ToolSelector::new(Model::Claude35Sonnet);
        let query = "How do I create an S3 bucket?";
        let modules = vec![
            Module {
                name: "AWS S3".to_string(),
                description: "Tools for working with S3".to_string(),
                tools: vec!["s3_create_bucket".to_string()],
                prompts: vec![],
                resources: vec![],
            }
        ];
        
        // First call should not use cache
        let tools1 = selector.select_tools(query, &modules, "").await.unwrap();
        assert!(!tools1.is_empty());
        
        // Second call should use cache
        let tools2 = selector.select_tools(query, &modules, "").await.unwrap();
        assert_eq!(tools1, tools2);
        
        // Force select should bypass cache
        let tools3 = selector.force_select_tools(query, &modules, "").await.unwrap();
        assert_eq!(tools1, tools3); // In our placeholder implementation, they're the same
    }
    
    #[tokio::test]
    async fn test_tool_selection_relevance() {
        let selector = ToolSelector::new(Model::Claude35Sonnet);
        let query = "How do I create an S3 bucket?";
        let modules = vec![
            Module {
                name: "AWS S3".to_string(),
                description: "Tools for working with S3".to_string(),
                tools: vec!["s3_create_bucket".to_string()],
                prompts: vec![],
                resources: vec![],
            },
            Module {
                name: "AWS EC2".to_string(),
                description: "Tools for working with EC2".to_string(),
                tools: vec!["ec2_create_instance".to_string()],
                prompts: vec![],
                resources: vec![],
            },
        ];
        
        let tools = selector.select_tools(query, &modules, "").await.unwrap();
        
        // Our simple keyword matching should select S3 tools but not EC2 tools
        assert!(tools.contains(&"s3_create_bucket".to_string()));
        assert!(!tools.contains(&"ec2_create_instance".to_string()));
    }
}
