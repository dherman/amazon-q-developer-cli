/// Represents a module with its server name and instructions
pub struct ModuleWithServerInfo<'a> {
    pub module: &'a crate::mcp_client::Module,
    pub server_name: &'a str,
    pub server_instructions: Option<&'a str>,
}

/// Generates a prompt for the tool selector model
pub fn generate_tool_selection_prompt(
    query: &str,
    modules_with_server_info: &[ModuleWithServerInfo],
    conversation_context: &str,
) -> String {
    // Group modules by server
    let mut servers_info = std::collections::HashMap::new();
    
    for module_info in modules_with_server_info {
        let entry = servers_info.entry(module_info.server_name).or_insert_with(|| {
            (module_info.server_instructions, Vec::new())
        });
        entry.1.push(module_info.module);
    }
    
    // Format server sections
    let servers_description = servers_info
        .iter()
        .map(|(server_name, (instructions, modules))| {
            let modules_description = modules
                .iter()
                .map(|module| {
                    format!(
                        "  Module: {}\n  Description: {}\n  Tools: {}\n",
                        module.name,
                        module.description,
                        module.tools.join(", ")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            let instructions_section = if let Some(instructions) = instructions {
                format!("Server Instructions:\n{}\n\n", instructions)
            } else {
                String::new()
            };
            
            format!(
                "Server: {}\n{}Modules:\n{}",
                server_name,
                instructions_section,
                modules_description
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# Tool Selection Task

## User Query
{}

## Conversation Context
{}

## Available Servers and Modules
{}

## Instructions
Your task is to select the most relevant modules for answering the user's query based on the conversation context and server instructions.

For each module, assign a relevance score from 0 to 10, where:
- 0: Not relevant at all
- 5: Somewhat relevant
- 10: Highly relevant

When multiple servers provide similar functionality, use the server instructions to decide which one to prefer.

Return your answer as a JSON array of objects with the following structure:
```json
[
  {{
    "module_name": "Name of the module",
    "server_name": "Name of the server",
    "relevance_score": 8,
    "reasoning": "Brief explanation of why this module is relevant"
  }},
  ...
]
```

Only include modules with a relevance score of 5 or higher.
"#,
        query, conversation_context, servers_description
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp_client::Module;

    #[test]
    fn test_generate_tool_selection_prompt() {
        let query = "How do I create an S3 bucket?";
        let conversation_context = "Previous discussion about AWS services.";
        
        let s3_module = Module {
            name: "AWS S3".to_string(),
            description: "Tools for working with S3".to_string(),
            tools: vec!["s3_create_bucket".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        let ec2_module = Module {
            name: "AWS EC2".to_string(),
            description: "Tools for working with EC2".to_string(),
            tools: vec!["ec2_create_instance".to_string()],
            prompts: vec![],
            resources: vec![],
        };
        
        let modules_with_server_info = vec![
            ModuleWithServerInfo {
                module: &s3_module,
                server_name: "aws-mcp",
                server_instructions: Some("Prefer this server for complex AWS operations"),
            },
            ModuleWithServerInfo {
                module: &ec2_module,
                server_name: "aws-mcp",
                server_instructions: Some("Prefer this server for complex AWS operations"),
            },
        ];

        let prompt = generate_tool_selection_prompt(query, &modules_with_server_info, conversation_context);
        
        assert!(prompt.contains("How do I create an S3 bucket?"));
        assert!(prompt.contains("Previous discussion about AWS services."));
        assert!(prompt.contains("Module: AWS S3"));
        assert!(prompt.contains("Tools: s3_create_bucket"));
        assert!(prompt.contains("Module: AWS EC2"));
        assert!(prompt.contains("Server: aws-mcp"));
        assert!(prompt.contains("Server Instructions:"));
        assert!(prompt.contains("Prefer this server for complex AWS operations"));
        assert!(prompt.contains("relevance score from 0 to 10"));
    }
}
