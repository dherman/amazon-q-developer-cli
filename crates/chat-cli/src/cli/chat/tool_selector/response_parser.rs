use serde::{Deserialize, Serialize};
use eyre::Result;

/// Represents a module selection with relevance score and reasoning
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSelection {
    pub module_name: String,
    #[serde(default)]
    pub server_name: String,
    pub relevance_score: u8,
    pub reasoning: String,
}

/// Parses the response from the tool selector model
pub fn parse_tool_selection_response(response: &str) -> Result<Vec<ModuleSelection>> {
    // Extract JSON from the response
    let json_str = extract_json(response)?;
    
    // Parse the JSON into a vector of ModuleSelection
    let selections: Vec<ModuleSelection> = serde_json::from_str(json_str)?;
    
    Ok(selections)
}

/// Extracts JSON from a string that might contain other text
fn extract_json(text: &str) -> Result<&str> {
    // Look for JSON array pattern
    if let Some(start) = text.find('[') {
        if let Some(end) = text.rfind(']') {
            if end > start {
                return Ok(&text[start..=end]);
            }
        }
    }
    
    // If we can't find JSON array pattern, return an error
    Err(eyre::eyre!("Could not extract JSON from response"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_json() {
        let text = r#"Here's my selection:
[
  {
    "module_name": "AWS S3",
    "server_name": "aws-mcp",
    "relevance_score": 9,
    "reasoning": "The query is directly about creating S3 buckets"
  },
  {
    "module_name": "AWS IAM",
    "server_name": "aws-mcp",
    "relevance_score": 6,
    "reasoning": "IAM permissions are needed for S3 operations"
  }
]
Hope this helps!"#;
        
        let json = extract_json(text).unwrap();
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("AWS S3"));
    }
    
    #[test]
    fn test_parse_tool_selection_response() {
        let response = r#"[
  {
    "module_name": "AWS S3",
    "server_name": "aws-mcp",
    "relevance_score": 9,
    "reasoning": "The query is directly about creating S3 buckets"
  },
  {
    "module_name": "AWS IAM",
    "server_name": "aws-mcp",
    "relevance_score": 6,
    "reasoning": "IAM permissions are needed for S3 operations"
  }
]"#;
        
        let selections = parse_tool_selection_response(response).unwrap();
        assert_eq!(selections.len(), 2);
        assert_eq!(selections[0].module_name, "AWS S3");
        assert_eq!(selections[0].server_name, "aws-mcp");
        assert_eq!(selections[0].relevance_score, 9);
        assert_eq!(selections[1].module_name, "AWS IAM");
    }
    
    #[test]
    fn test_parse_tool_selection_response_without_server_name() {
        // For backward compatibility, we should handle responses without server_name
        let response = r#"[
  {
    "module_name": "AWS S3",
    "relevance_score": 9,
    "reasoning": "The query is directly about creating S3 buckets"
  },
  {
    "module_name": "AWS IAM",
    "relevance_score": 6,
    "reasoning": "IAM permissions are needed for S3 operations"
  }
]"#;
        
        let selections = parse_tool_selection_response(response).unwrap();
        assert_eq!(selections.len(), 2);
        assert_eq!(selections[0].module_name, "AWS S3");
        assert_eq!(selections[0].server_name, ""); // Default empty string
        assert_eq!(selections[0].relevance_score, 9);
        assert_eq!(selections[1].module_name, "AWS IAM");
    }
}
