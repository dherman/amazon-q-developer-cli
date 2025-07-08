use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a module in an MCP server.
/// A module is a logical grouping of tools, prompts, or resources that are related
/// and should be selected together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Module {
    /// The name of the module
    pub name: String,
    
    /// A description of the module's functionality, used for dynamic selection
    pub description: String,
    
    /// List of tool names that belong to this module
    #[serde(default)]
    pub tools: Vec<String>,
    
    /// List of prompt names that belong to this module
    #[serde(default)]
    pub prompts: Vec<String>,
    
    /// List of resource names that belong to this module
    #[serde(default)]
    pub resources: Vec<String>,
}

/// Represents metadata for an MCP server, including module definitions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ServerMetadata {
    /// List of modules defined for this server
    #[serde(default)]
    pub modules: Vec<Module>,
    
    /// Additional metadata fields that might be added in the future
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Error types for module validation
#[derive(Debug, thiserror::Error)]
pub enum ModuleValidationError {
    #[error("Module name cannot be empty")]
    EmptyName,
    
    #[error("Module description cannot be empty")]
    EmptyDescription,
    
    #[error("Module must contain at least one tool, prompt, or resource")]
    EmptyModule,
    
    #[error("Invalid module format: {0}")]
    InvalidFormat(String),
}

impl Module {
    /// Validates that the module has the required fields and meets basic requirements
    pub fn validate(&self) -> Result<(), ModuleValidationError> {
        if self.name.is_empty() {
            return Err(ModuleValidationError::EmptyName);
        }
        
        if self.description.is_empty() {
            return Err(ModuleValidationError::EmptyDescription);
        }
        
        if self.tools.is_empty() && self.prompts.is_empty() && self.resources.is_empty() {
            return Err(ModuleValidationError::EmptyModule);
        }
        
        Ok(())
    }
}

impl ServerMetadata {
    /// Validates all modules in the metadata
    pub fn validate(&self) -> Result<(), Vec<(String, ModuleValidationError)>> {
        let mut errors = Vec::new();
        
        for module in &self.modules {
            if let Err(err) = module.validate() {
                errors.push((module.name.clone(), err));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_validation_valid() {
        let module = Module {
            name: "aws-ec2".to_string(),
            description: "Tools for managing EC2 instances".to_string(),
            tools: vec!["ec2_create_instance".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        assert!(module.validate().is_ok());
    }
    
    #[test]
    fn test_module_validation_empty_name() {
        let module = Module {
            name: "".to_string(),
            description: "Tools for managing EC2 instances".to_string(),
            tools: vec!["ec2_create_instance".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        let result = module.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ModuleValidationError::EmptyName));
    }
    
    #[test]
    fn test_module_validation_empty_description() {
        let module = Module {
            name: "aws-ec2".to_string(),
            description: "".to_string(),
            tools: vec!["ec2_create_instance".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        let result = module.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ModuleValidationError::EmptyDescription));
    }
    
    #[test]
    fn test_module_validation_empty_module() {
        let module = Module {
            name: "aws-ec2".to_string(),
            description: "Tools for managing EC2 instances".to_string(),
            tools: vec![],
            prompts: vec![],
            resources: vec![],
        };
        
        let result = module.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ModuleValidationError::EmptyModule));
    }
    
    #[test]
    fn test_server_metadata_validation() {
        let metadata = ServerMetadata {
            modules: vec![
                Module {
                    name: "aws-ec2".to_string(),
                    description: "Tools for managing EC2 instances".to_string(),
                    tools: vec!["ec2_create_instance".to_string()],
                    prompts: vec![],
                    resources: vec![],
                },
                Module {
                    name: "aws-s3".to_string(),
                    description: "Tools for managing S3 buckets".to_string(),
                    tools: vec!["s3_create_bucket".to_string()],
                    prompts: vec![],
                    resources: vec![],
                },
            ],
            additional_fields: HashMap::new(),
        };
        
        assert!(metadata.validate().is_ok());
    }
    
    #[test]
    fn test_server_metadata_validation_with_errors() {
        let metadata = ServerMetadata {
            modules: vec![
                Module {
                    name: "aws-ec2".to_string(),
                    description: "Tools for managing EC2 instances".to_string(),
                    tools: vec!["ec2_create_instance".to_string()],
                    prompts: vec![],
                    resources: vec![],
                },
                Module {
                    name: "".to_string(),
                    description: "Tools for managing S3 buckets".to_string(),
                    tools: vec!["s3_create_bucket".to_string()],
                    prompts: vec![],
                    resources: vec![],
                },
            ],
            additional_fields: HashMap::new(),
        };
        
        let result = metadata.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 1);
    }
    
    #[test]
    fn test_module_serialization() {
        let module = Module {
            name: "aws-ec2".to_string(),
            description: "Tools for managing EC2 instances".to_string(),
            tools: vec!["ec2_create_instance".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        let json = serde_json::to_string(&module).unwrap();
        let deserialized: Module = serde_json::from_str(&json).unwrap();
        
        assert_eq!(module, deserialized);
    }
    
    #[test]
    fn test_server_metadata_serialization() {
        let metadata = ServerMetadata {
            modules: vec![
                Module {
                    name: "aws-ec2".to_string(),
                    description: "Tools for managing EC2 instances".to_string(),
                    tools: vec!["ec2_create_instance".to_string()],
                    prompts: vec![],
                    resources: vec![],
                },
            ],
            additional_fields: {
                let mut map = HashMap::new();
                map.insert("version".to_string(), serde_json::json!("1.0.0"));
                map
            },
        };
        
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ServerMetadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(metadata, deserialized);
    }
}
