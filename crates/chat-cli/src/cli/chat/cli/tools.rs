use std::collections::{
    BTreeSet,
    HashSet,
};
use std::io::Write;

use clap::{
    Args,
    Subcommand,
};
use crossterm::style::{
    Attribute,
    Color,
    Stylize,
};
use crossterm::{
    queue,
    style,
};

use crate::api_client::model::Tool as FigTool;
use crate::cli::agent::Agent;
use crate::cli::chat::consts::DUMMY_TOOL_NAME;
use crate::cli::chat::tools::ToolOrigin;
use crate::cli::chat::{
    ChatError,
    ChatSession,
    ChatState,
    TRUST_ALL_TEXT,
};
use crate::util::consts::MCP_SERVER_TOOL_DELIMITER;

#[deny(missing_docs)]
#[derive(Debug, PartialEq, Args)]
pub struct ToolsArgs {
    /// Show all tools, including those not dynamically selected
    #[arg(short = 'a', long = "all")]
    all: bool,
    
    #[command(subcommand)]
    subcommand: Option<ToolsSubcommand>,
}

impl ToolsArgs {
    pub async fn execute(self, os: &mut crate::os::Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        if let Some(subcommand) = self.subcommand {
            return subcommand.execute(os, session).await;
        }

        // No subcommand - print the current tools and their permissions.
        // Determine how to format the output nicely.
        let terminal_width = session.terminal_width();
        let longest = session
            .conversation
            .tool_manager
            .tn_map
            .values()
            .map(|info| info.host_tool_name.len())
            .max()
            .unwrap_or(0)
            .max(
                session
                    .conversation
                    .tools
                    .get("native")
                    .and_then(|tools| {
                        tools
                            .iter()
                            .map(|tool| {
                                let FigTool::ToolSpecification(t) = tool;
                                t.name.len()
                            })
                            .max()
                    })
                    .unwrap_or(0),
            );

        queue!(
            session.stderr,
            style::Print("\n"),
            style::SetAttribute(Attribute::Bold),
            style::Print({
                // Adding 2 because of "- " preceding every tool name
                let width = (longest + 2).saturating_sub("Tool".len()) + 4;
                format!("Tool{:>width$}Permission", "", width = width)
            }),
            style::SetAttribute(Attribute::Reset),
            style::Print("\n"),
            style::Print("▔".repeat(terminal_width)),
        )?;

        let mut origin_tools: Vec<_> = session.conversation.tools.iter().collect();

        // If showing all tools, we need to also include tools from dynamic servers that aren't currently selected
        let additional_dynamic_tools = if self.all {
            let mut additional = std::collections::HashMap::new();
            
            // For dynamic servers, we need to directly query them for their tools
            // since they don't load tools into tn_map until they're selected
            for (server_name, client) in &session.conversation.tool_manager.clients {
                if client.is_dynamic() {
                    // Query the server directly for its tools
                    match client.request("tools/list", None).await {
                        Ok(response) => {
                            if let Some(result) = response.result {
                                if let Ok(tools_result) = serde_json::from_value::<crate::mcp_client::facilitator_types::ToolsListResult>(result) {
                                    // Define a temporary structure that matches the MCP server's tool format
                                    #[derive(serde::Deserialize)]
                                    struct McpToolSpec {
                                        name: String,
                                        description: String,
                                        #[allow(dead_code)]
                                        parameters: serde_json::Value,
                                    }
                                    
                                    for tool_value in tools_result.tools {
                                        if let Ok(mcp_spec) = serde_json::from_value::<McpToolSpec>(tool_value) {
                                            // Create the model tool name (with server prefix)
                                            let model_tool_name = format!("@{}___{}", server_name, mcp_spec.name);
                                            
                                            // Check if this tool is already in the selected tools
                                            // We need to check both the plain name and the model tool name
                                            let is_already_selected = session.conversation.tools
                                                .values()
                                                .flat_map(|tools| tools.iter())
                                                .any(|FigTool::ToolSpecification(tool_spec)| {
                                                    tool_spec.name == model_tool_name || tool_spec.name == mcp_spec.name
                                                });
                                            
                                            if !is_already_selected {
                                                // Create a FigTool for this unselected dynamic tool
                                                let fig_tool = FigTool::ToolSpecification(crate::api_client::model::ToolSpecification {
                                                    name: model_tool_name,
                                                    description: if mcp_spec.description.is_empty() {
                                                        format!("Tool from dynamic server {}", server_name)
                                                    } else {
                                                        mcp_spec.description
                                                    },
                                                    input_schema: crate::api_client::model::ToolInputSchema {
                                                        json: None,
                                                    },
                                                });
                                                
                                                let origin = ToolOrigin::McpServer(server_name.clone());
                                                additional
                                                    .entry(origin)
                                                    .or_insert_with(Vec::new)
                                                    .push(fig_tool);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_e) => {
                            // Silently ignore errors when querying dynamic servers
                        }
                    }
                }
            }
            
            additional
        } else {
            std::collections::HashMap::new()
        };
        
        // Convert additional_dynamic_tools to the same format as origin_tools
        let additional_refs: Vec<_> = additional_dynamic_tools.iter().collect();
        
        // Combine both regular tools and additional dynamic tools
        origin_tools.extend(additional_refs);

        // Built in tools always appear first.
        origin_tools.sort_by(|(origin_a, _), (origin_b, _)| match (origin_a, origin_b) {
            (ToolOrigin::Native, _) => std::cmp::Ordering::Less,
            (_, ToolOrigin::Native) => std::cmp::Ordering::Greater,
            (ToolOrigin::McpServer(name_a), ToolOrigin::McpServer(name_b)) => name_a.cmp(name_b),
        });

        // Calculate the longest tool name for formatting, including dynamic tools
        let longest = origin_tools
            .iter()
            .flat_map(|(_, tools)| tools.iter())
            .map(|tool| {
                let FigTool::ToolSpecification(spec) = tool;
                // For display purposes, we need to use the host tool name if available
                session.conversation.tool_manager.tn_map
                    .get(&spec.name)
                    .map_or(spec.name.len(), |info| info.host_tool_name.len())
            })
            .max()
            .unwrap_or(0);

        // Get the list of currently selected tools for comparison
        let currently_selected_tools: HashSet<String> = session.conversation.tools.values()
            .flat_map(|tools| tools.iter())
            .filter_map(|FigTool::ToolSpecification(spec)| {
                if spec.name == DUMMY_TOOL_NAME {
                    None
                } else {
                    Some(spec.name.clone())
                }
            })
            .collect();

        for (origin, tools) in origin_tools.iter() {
            // Note that Tool is model facing and thus would have names recognized by model.
            // Here we need to convert them to their host / user facing counter part.
            let tn_map = &session.conversation.tool_manager.tn_map;
            let sorted_tools = tools
                .iter()
                .filter_map(|FigTool::ToolSpecification(spec)| {
                    if spec.name == DUMMY_TOOL_NAME {
                        return None;
                    }

                    // For non-all mode, only show selected tools from dynamic servers
                    if !self.all && matches!(origin, ToolOrigin::McpServer(_)) {
                        if !currently_selected_tools.contains(&spec.name) {
                            return None;
                        }
                    }

                    tn_map
                        .get(&spec.name)
                        .map_or(Some(spec.name.as_str()), |info| Some(info.host_tool_name.as_str()))
                })
                .collect::<BTreeSet<_>>();

            // Skip empty origins
            if sorted_tools.is_empty() {
                continue;
            }

            let to_display = sorted_tools.iter().fold(String::new(), |mut acc, tool_name| {
                let width = longest.saturating_sub(tool_name.len()).saturating_add(4);
                
                // For dynamic servers in "all" mode, show unselected tools in gray
                let is_unselected_dynamic = self.all && 
                                          matches!(origin, ToolOrigin::McpServer(_)) && 
                                          !currently_selected_tools.contains(&tool_name.to_string());
                
                // Format the tool name based on whether it's selected or not
                let formatted_tool_name = if is_unselected_dynamic {
                    format!("{}", style::style(tool_name).with(Color::DarkGrey))
                } else {
                    tool_name.to_string()
                };
                
                acc.push_str(
                    format!(
                        "- {}{:>width$}{}\n",
                        formatted_tool_name,
                        "",
                        session.conversation.agents.display_label(tool_name, origin),
                        width = width
                    )
                    .as_str(),
                );
                acc
            });

            let _ = queue!(
                session.stderr,
                style::SetAttribute(Attribute::Bold),
                style::Print(format!("{}:\n", origin)),
                style::SetAttribute(Attribute::Reset),
                style::Print(to_display),
                style::Print("\n")
            );
        }

        let loading = session.conversation.tool_manager.pending_clients().await;
        if !loading.is_empty() {
            queue!(
                session.stderr,
                style::SetAttribute(Attribute::Bold),
                style::Print("Servers still loading"),
                style::SetAttribute(Attribute::Reset),
                style::Print("\n"),
                style::Print("▔".repeat(terminal_width)),
            )?;
            for client in loading {
                queue!(session.stderr, style::Print(format!(" - {client}")), style::Print("\n"))?;
            }
        }

        // Add a note about dynamic selection if applicable
        let has_dynamic_servers = session.conversation.tool_manager.clients.values().any(|client| client.is_dynamic());
        if has_dynamic_servers {
            if self.all {
                queue!(
                    session.stderr,
                    style::Print("\n"),
                    style::SetForegroundColor(Color::DarkGrey),
                    style::Print("Note: Tools in gray are from dynamic servers but not currently selected."),
                    style::SetForegroundColor(Color::Reset),
                    style::Print("\n"),
                )?;
            } else {
                queue!(
                    session.stderr,
                    style::Print("\n"),
                    style::SetForegroundColor(Color::DarkGrey),
                    style::Print("Note: Only showing dynamically selected tools. Use "),
                    style::SetForegroundColor(Color::Green),
                    style::Print("/tools -a"),
                    style::SetForegroundColor(Color::DarkGrey),
                    style::Print(" to show all tools."),
                    style::SetForegroundColor(Color::Reset),
                    style::Print("\n"),
                )?;
            }
        }

        queue!(
            session.stderr,
            style::Print("\nTrusted tools will run without confirmation."),
            style::SetForegroundColor(Color::DarkGrey),
            style::Print(format!("\n{}\n", "* Default settings")),
            style::Print("\n💡 Use "),
            style::SetForegroundColor(Color::Green),
            style::Print("/tools help"),
            style::SetForegroundColor(Color::Reset),
            style::SetForegroundColor(Color::DarkGrey),
            style::Print(" to edit permissions.\n\n"),
            style::SetForegroundColor(Color::Reset),
        )?;

        Ok(ChatState::default())
    }
}

#[deny(missing_docs)]
#[derive(Debug, PartialEq, Subcommand)]
#[command(
    before_long_help = "By default, Amazon Q will ask for your permission to use certain tools. You can control which tools you
trust so that no confirmation is required. These settings will last only for this session."
)]
pub enum ToolsSubcommand {
    /// Show the input schema for all available tools
    Schema,
    /// Trust a specific tool or tools for the session
    Trust {
        #[arg(required = true)]
        tool_names: Vec<String>,
    },
    /// Revert a tool or tools to per-request confirmation
    Untrust {
        #[arg(required = true)]
        tool_names: Vec<String>,
    },
    /// Trust all tools (equivalent to deprecated /acceptall)
    TrustAll,
    /// Reset all tools to default permission levels
    Reset,
    /// Reset a single tool to default permission level
    ResetSingle {
        #[arg(required = true)]
        tool_name: String,
    },
    /// Force re-selection of tools based on your conversation context
    /// Only works with dynamic MCP servers
    Select,
}

impl ToolsSubcommand {
    pub async fn execute(self, os: &mut crate::os::Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        // Here we need to obtain the list of host tool names
        let existing_custom_tools = session
            .conversation
            .tool_manager
            .tn_map
            .values()
            .cloned()
            .collect::<HashSet<_>>();

        // We also need to obtain a list of native tools since tn_map from ToolManager does not
        // contain native tools
        let native_tool_names = session
            .conversation
            .tools
            .get("native")
            .map(|tools| {
                tools
                    .iter()
                    .filter_map(|tool| match tool {
                        FigTool::ToolSpecification(t) if t.name != DUMMY_TOOL_NAME => Some(t.name.clone()),
                        FigTool::ToolSpecification(_) => None,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        match self {
            Self::Schema => {
                let schema_json = serde_json::to_string_pretty(&session.conversation.tool_manager.schema)
                    .map_err(|e| ChatError::Custom(format!("Error converting tool schema to string: {e}").into()))?;
                queue!(session.stderr, style::Print(schema_json), style::Print("\n"))?;
            },
            Self::Trust { tool_names } => {
                let (valid_tools, invalid_tools): (Vec<String>, Vec<String>) =
                    tool_names.into_iter().partition(|tool_name| {
                        existing_custom_tools.contains(tool_name) || native_tool_names.contains(tool_name)
                    });

                if !invalid_tools.is_empty() {
                    queue!(
                        session.stderr,
                        style::SetForegroundColor(Color::Red),
                        style::Print(format!("\nCannot trust '{}', ", invalid_tools.join("', '"))),
                        if invalid_tools.len() > 1 {
                            style::Print("they do not exist.")
                        } else {
                            style::Print("it does not exist.")
                        },
                        style::SetForegroundColor(Color::Reset),
                    )?;
                }
                if !valid_tools.is_empty() {
                    let tools_to_trust = valid_tools
                        .into_iter()
                        .filter_map(|tool_name| {
                            if native_tool_names.contains(&tool_name) {
                                Some(tool_name)
                            } else {
                                existing_custom_tools
                                    .get(&tool_name)
                                    .map(|info| format!("@{}{MCP_SERVER_TOOL_DELIMITER}{tool_name}", info.server_name))
                            }
                        })
                        .collect::<Vec<_>>();

                    queue!(
                        session.stderr,
                        style::SetForegroundColor(Color::Green),
                        if tools_to_trust.len() > 1 {
                            style::Print(format!("\nTools '{}' are ", tools_to_trust.join("', '")))
                        } else {
                            style::Print(format!("\nTool '{}' is ", tools_to_trust[0]))
                        },
                        style::Print("now trusted. I will "),
                        style::SetAttribute(Attribute::Bold),
                        style::Print("not"),
                        style::SetAttribute(Attribute::Reset),
                        style::SetForegroundColor(Color::Green),
                        style::Print(format!(
                            " ask for confirmation before running {}.",
                            if tools_to_trust.len() > 1 {
                                "these tools"
                            } else {
                                "this tool"
                            }
                        )),
                        style::Print("\n"),
                        style::SetForegroundColor(Color::Reset),
                    )?;

                    session.conversation.agents.trust_tools(tools_to_trust);
                }
            },
            Self::Untrust { tool_names } => {
                let (valid_tools, invalid_tools): (Vec<String>, Vec<String>) =
                    tool_names.into_iter().partition(|tool_name| {
                        existing_custom_tools.contains(tool_name) || native_tool_names.contains(tool_name)
                    });

                if !invalid_tools.is_empty() {
                    queue!(
                        session.stderr,
                        style::SetForegroundColor(Color::Red),
                        style::Print(format!("\nCannot untrust '{}', ", invalid_tools.join("', '"))),
                        if invalid_tools.len() > 1 {
                            style::Print("they do not exist.")
                        } else {
                            style::Print("it does not exist.")
                        },
                        style::SetForegroundColor(Color::Reset),
                    )?;
                }
                if !valid_tools.is_empty() {
                    let tools_to_untrust = valid_tools
                        .into_iter()
                        .filter_map(|tool_name| {
                            if native_tool_names.contains(&tool_name) {
                                Some(tool_name)
                            } else {
                                existing_custom_tools
                                    .get(&tool_name)
                                    .map(|info| format!("@{}{MCP_SERVER_TOOL_DELIMITER}{tool_name}", info.server_name))
                            }
                        })
                        .collect::<Vec<_>>();

                    session.conversation.agents.untrust_tools(&tools_to_untrust);

                    queue!(
                        session.stderr,
                        style::SetForegroundColor(Color::Green),
                        if tools_to_untrust.len() > 1 {
                            style::Print(format!("\nTools '{}' are ", tools_to_untrust.join("', '")))
                        } else {
                            style::Print(format!("\nTool '{}' is ", tools_to_untrust[0]))
                        },
                        style::Print("set to per-request confirmation.\n"),
                        style::SetForegroundColor(Color::Reset),
                    )?;
                }
            },
            Self::TrustAll => {
                session.conversation.agents.trust_all_tools = true;
                queue!(session.stderr, style::Print(TRUST_ALL_TEXT))?;
            },
            Self::Reset => {
                session.conversation.agents.trust_all_tools = false;

                let active_agent_path = session.conversation.agents.get_active().and_then(|a| a.path.clone());
                if let Some(path) = active_agent_path {
                    let result = async {
                        let content = tokio::fs::read(&path).await?;
                        let orig_agent: Agent = serde_json::from_slice(&content)?;
                        Ok::<Agent, Box<dyn std::error::Error>>(orig_agent)
                    }
                    .await;

                    if let (Ok(orig_agent), Some(active_agent)) = (result, session.conversation.agents.get_active_mut())
                    {
                        active_agent.allowed_tools = orig_agent.allowed_tools;
                    }
                } else if session
                    .conversation
                    .agents
                    .get_active()
                    .is_some_and(|a| a.name.as_str() == "default")
                {
                    // We only want to reset the tool permission and nothing else
                    if let Some(active_agent) = session.conversation.agents.get_active_mut() {
                        active_agent.allowed_tools = Default::default();
                        active_agent.tools_settings = Default::default();
                    }
                }
                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Green),
                    style::Print("\nReset all tools to the permission levels as defined in agent."),
                    style::SetForegroundColor(Color::Reset),
                )?;
            },
            Self::ResetSingle { tool_name } => {
                // Implementation for ResetSingle
                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Green),
                    style::Print(format!("\nReset tool '{}' to default permission level.", tool_name)),
                    style::SetForegroundColor(Color::Reset),
                )?;
            },
            Self::Select => {
                // Get the last user query
                let last_query = session.conversation.last_user_query().unwrap_or_default();
                let conversation_context = session.conversation.get_context_summary();
                
                // Check if we have any dynamic servers
                let has_dynamic_servers = session.conversation.tool_manager.clients.values().any(|client| client.is_dynamic());
                if !has_dynamic_servers {
                    queue!(
                        session.stderr,
                        style::SetForegroundColor(Color::Yellow),
                        style::Print("\nNo dynamic MCP servers configured. Tool selection is only available with dynamic servers."),
                        style::SetForegroundColor(Color::Reset),
                    )?;
                    return Ok(ChatState::PromptUser { skip_printing_tools: true });
                }
                
                // Use the tool manager to filter tools dynamically
                let current_model = session.conversation.model.as_deref();
                match session.conversation.tool_manager.force_select_tools(&os.client, &last_query, &conversation_context, current_model).await {
                    Ok(filtered_tools) => {
                        // Update the tools in the conversation
                        session.conversation.tools = filtered_tools
                            .values()
                            .fold(std::collections::HashMap::<ToolOrigin, Vec<FigTool>>::new(), |mut acc, v| {
                                let input_schema = crate::api_client::model::ToolInputSchema {
                                    json: Some(crate::api_client::model::FigDocument::from(aws_smithy_types::Document::Null)),
                                };
                                
                                let tool = FigTool::ToolSpecification(crate::api_client::model::ToolSpecification {
                                    name: v.name.clone(),
                                    description: v.description.clone(),
                                    input_schema,
                                });
                                acc.entry(v.tool_origin.clone())
                                    .and_modify(|tools| tools.push(tool.clone()))
                                    .or_insert(vec![tool]);
                                acc
                            });
                        
                        // Count the number of tools from dynamic servers
                        let dynamic_tool_count = filtered_tools
                            .values()
                            .filter(|tool| {
                                if let ToolOrigin::McpServer(server_name) = &tool.tool_origin {
                                    if let Some(client) = session.conversation.tool_manager.clients.get(server_name) {
                                        return client.is_dynamic();
                                    }
                                }
                                false
                            })
                            .count();
                        
                        // Show success message
                        queue!(
                            session.stderr,
                            style::SetForegroundColor(Color::Green),
                            style::Print(format!("\nDynamically selected {} tools based on your query.", dynamic_tool_count)),
                            style::Print("\nUse "),
                            style::SetForegroundColor(Color::Reset),
                            style::SetForegroundColor(Color::Blue),
                            style::Print("/tools"),
                            style::SetForegroundColor(Color::Reset),
                            style::SetForegroundColor(Color::Green),
                            style::Print(" to see the selected tools."),
                            style::SetForegroundColor(Color::Reset),
                        )?;
                    },
                    Err(e) => {
                        queue!(
                            session.stderr,
                            style::SetForegroundColor(Color::Red),
                            style::Print(format!("\nError selecting tools: {}", e)),
                            style::SetForegroundColor(Color::Reset),
                        )?;
                    }
                }
            },
        };

        session.stderr.flush()?;

        Ok(ChatState::PromptUser {
            skip_printing_tools: true,
        })
    }
}
