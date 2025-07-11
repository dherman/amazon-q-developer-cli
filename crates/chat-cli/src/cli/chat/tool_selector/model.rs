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
    
    /// Creates a Model from a Q CLI model ID
    /// Falls back to Claude 3.5 Sonnet if the model ID is not recognized
    pub fn from_model_id(model_id: &str) -> Self {
        match model_id {
            "CLAUDE_3_5_SONNET_20241022_V2_0" => Self::Claude35Sonnet,
            "CLAUDE_3_HAIKU_20240307_V1_0" => Self::Claude3Haiku,
            // For newer models (4.0, 3.7) that we don't have specific variants for yet,
            // we'll use the Claude 3.5 Sonnet as it's the most capable supported model
            _ => Self::Claude35Sonnet,
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
    
    #[test]
    fn test_from_model_id() {
        assert_eq!(Model::from_model_id("CLAUDE_3_5_SONNET_20241022_V2_0"), Model::Claude35Sonnet);
        assert_eq!(Model::from_model_id("CLAUDE_3_HAIKU_20240307_V1_0"), Model::Claude3Haiku);
        // Test fallback for unknown models
        assert_eq!(Model::from_model_id("CLAUDE_SONNET_4_20250514_V1_0"), Model::Claude35Sonnet);
        assert_eq!(Model::from_model_id("CLAUDE_3_7_SONNET_20250219_V1_0"), Model::Claude35Sonnet);
        assert_eq!(Model::from_model_id("unknown_model"), Model::Claude35Sonnet);
    }
}
