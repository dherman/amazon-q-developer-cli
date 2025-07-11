use eyre::Result;
use tracing::{debug, error};

use crate::api_client::ApiClient;
use crate::api_client::model::{ConversationState, UserInputMessage, ChatResponseStream};

use super::Model;
use super::response_parser::{parse_tool_selection_response, ModuleSelection};

/// A simple client for making tool selection requests
pub struct ToolSelectionClient<'a> {
    api_client: &'a ApiClient,
    model: Model,
}

impl<'a> ToolSelectionClient<'a> {
    /// Creates a new tool selection client
    pub fn new(api_client: &'a ApiClient, model: Model) -> Self {
        Self { api_client, model }
    }
    
    /// Sends a tool selection request to the LLM
    pub async fn select_tools(&self, prompt: String) -> Result<Vec<ModuleSelection>> {
        debug!("Sending tool selection request with model: {}", self.model);
        
        // Create a conversation state for the tool selection request
        let conversation_state = ConversationState {
            conversation_id: None,
            user_input_message: UserInputMessage {
                content: prompt,
                images: None,
                user_input_message_context: None,
                user_intent: None,
                model_id: Some(self.model.to_model_id()),
            },
            history: None,
        };
        
        // Send the message using the existing API client
        let mut output = self.api_client.send_message(conversation_state).await?;
        
        // Collect the response
        let mut response_text = String::new();
        while let Some(event) = output.recv().await? {
            match event {
                ChatResponseStream::AssistantResponseEvent { content } => {
                    response_text.push_str(&content);
                }
                ChatResponseStream::ToolUseEvent { .. } => {
                    // Tool selection shouldn't use tools
                    debug!("Unexpected tool use event in tool selection response");
                }
                _ => {
                    // Ignore other events
                }
            }
        }
        
        debug!("Tool selection response: {}", response_text);
        
        // Parse the response
        match parse_tool_selection_response(&response_text) {
            Ok(selections) => Ok(selections),
            Err(e) => {
                error!("Failed to parse tool selection response: {}", e);
                // Fall back to empty selection rather than failing
                Ok(vec![])
            }
        }
    }
}