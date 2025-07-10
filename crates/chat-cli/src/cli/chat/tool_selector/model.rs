/// Represents a model that can be used for tool selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    /// Claude 3.5 Sonnet
    Claude35Sonnet,
    /// Claude 3 Haiku
    Claude3Haiku,
}

impl Model {
    /// Returns the model ID as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Claude35Sonnet => "claude-3-5-sonnet-20240620",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }
    
    /// Returns the AWS Bedrock model ID for API calls
    pub fn to_model_id(&self) -> String {
        match self {
            Self::Claude35Sonnet => "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string(),
            Self::Claude3Haiku => "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
        }
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_as_str() {
        assert_eq!(Model::Claude35Sonnet.as_str(), "claude-3-5-sonnet-20240620");
        assert_eq!(Model::Claude3Haiku.as_str(), "claude-3-haiku-20240307");
    }
    
    #[test]
    fn test_model_display() {
        assert_eq!(Model::Claude35Sonnet.to_string(), "claude-3-5-sonnet-20240620");
        assert_eq!(Model::Claude3Haiku.to_string(), "claude-3-haiku-20240307");
    }
}
