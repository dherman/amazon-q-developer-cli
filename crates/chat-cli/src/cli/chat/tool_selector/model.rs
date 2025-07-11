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
            Self::Claude35Sonnet => "claude-3-5-sonnet-20241022",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }
    
    /// Returns the Q CLI model ID for API calls
    pub fn to_model_id(&self) -> String {
        match self {
            Self::Claude35Sonnet => "CLAUDE_3_5_SONNET_20241022_V2_0".to_string(),
            Self::Claude3Haiku => "CLAUDE_3_HAIKU_20240307_V1_0".to_string(),
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
        assert_eq!(Model::Claude35Sonnet.as_str(), "claude-3-5-sonnet-20241022");
        assert_eq!(Model::Claude3Haiku.as_str(), "claude-3-haiku-20240307");
    }
    
    #[test]
    fn test_model_display() {
        assert_eq!(Model::Claude35Sonnet.to_string(), "claude-3-5-sonnet-20241022");
        assert_eq!(Model::Claude3Haiku.to_string(), "claude-3-haiku-20240307");
    }
    
    #[test]
    fn test_model_to_model_id() {
        assert_eq!(Model::Claude35Sonnet.to_model_id(), "CLAUDE_3_5_SONNET_20241022_V2_0");
        assert_eq!(Model::Claude3Haiku.to_model_id(), "CLAUDE_3_HAIKU_20240307_V1_0");
    }
}
