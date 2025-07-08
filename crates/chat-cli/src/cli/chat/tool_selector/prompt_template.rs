/// Generates a prompt for the tool selector model
pub fn generate_tool_selection_prompt(
    query: &str,
    modules: &[crate::mcp_client::Module],
    conversation_context: &str,
) -> String {
    let modules_description = modules
        .iter()
        .map(|module| {
            format!(
                "Module: {}\nDescription: {}\nTools: {}\n",
                module.name,
                module.description,
                module.tools.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"# Tool Selection Task

## User Query
{}

## Conversation Context
{}

## Available Modules
{}

## Instructions
Your task is to select the most relevant modules for answering the user's query based on the conversation context.

For each module, assign a relevance score from 0 to 10, where:
- 0: Not relevant at all
- 5: Somewhat relevant
- 10: Highly relevant

Return your answer as a JSON array of objects with the following structure:
```json
[
  {{
    "module_name": "Name of the module",
    "relevance_score": 8,
    "reasoning": "Brief explanation of why this module is relevant"
  }},
  ...
]
```

Only include modules with a relevance score of 5 or higher.
"#,
        query, conversation_context, modules_description
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

        let prompt = generate_tool_selection_prompt(query, &modules, conversation_context);
        
        assert!(prompt.contains("How do I create an S3 bucket?"));
        assert!(prompt.contains("Previous discussion about AWS services."));
        assert!(prompt.contains("Module: AWS S3"));
        assert!(prompt.contains("Tools: s3_create_bucket"));
        assert!(prompt.contains("Module: AWS EC2"));
        assert!(prompt.contains("relevance score from 0 to 10"));
    }
}
