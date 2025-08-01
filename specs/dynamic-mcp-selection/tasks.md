# Dynamic MCP Selection Implementation Plan

## Codebase Insights

- **MCP Server Configuration**: The Q CLI already has a robust system for managing MCP server configurations through the `mcp.rs` module. This provides a foundation for adding the new `dynamic` flag.

- **Tool Manager**: The `tool_manager.rs` file handles the loading and management of MCP servers and their tools. This component will need to be modified to support dynamic selection of tools based on conversation context.

- **LLM Integration**: The codebase already has abstractions for working with different LLM models in `model.rs`, which we can leverage for our tool selector implementation.

- **Command Interface**: The existing `/tools` command can be extended to support our new functionality, making it intuitive for users to work with dynamically selected tools.

## Core Architecture

- [x] Add `dynamic` flag to MCP server configuration
  - [x] Update `McpServerConfig` struct to include a `dynamic` boolean field
  - [x] Update MCP server configuration serialization/deserialization
  - [x] Modify `AddArgs` in `mcp.rs` to accept a `--dynamic` flag
  - [x] Update `add` command implementation to handle the new flag

- [x] Implement module metadata format
  - [x] Define JSON schema for modules in `_meta` field
  - [x] Add validation for module definitions when loading MCP servers
  - [x] Create data structures to represent modules and their relationships to tools

- [x] Create tool selector infrastructure
  - [x] Define `ToolSelector` trait for abstraction
  - [x] Implement LLM-based tool selector using Claude 3.5-Sonnet
  - [x] Add configuration for tool selector model selection (hardcoded initially)
  - [x] Create prompt templates for tool selection

## LLM Integration for Tool Selection

- [x] Create tool selection client
  - [x] Implement `ToolSelectionClient` that reuses the existing `ApiClient`
  - [x] Use same authentication mechanism as main chat
  - [x] Support Claude 3.5 Sonnet model (hardcoded initially)
  - [x] Map Q CLI model IDs correctly (e.g., CLAUDE_3_5_SONNET_20241022_V2_0)

- [x] Integrate LLM client with tool selector
  - [x] Replace heuristic implementation with LLM call in `select_tools_with_model`
  - [x] Use existing prompt template and response parser
  - [x] Fall back to heuristic implementation on LLM errors
  - [x] Add proper error handling and logging

- [ ] Testing and optimization
  - [ ] Test with AWS MCP server for S3 and Lambda queries
  - [ ] Verify tool selection accuracy
  - [ ] Monitor selection latency
  - [ ] Optimize prompt template based on results

- [x] Integrate server instructions
  - [x] Update ServerCapabilities to include instructions field
  - [x] Update ServerMetadata to include instructions field
  - [x] Modify CustomToolClient to store and retrieve server instructions
  - [x] Update prompt template to include server instructions
  - [x] Update tool selector to use server instructions for better selection

- [x] Modify tool manager to support dynamic selection
  - [x] Update `ToolManager` to track which servers are dynamic vs. pinned
  - [x] Add logic to determine when to trigger tool selection
  - [x] Implement caching of tool selection results

## Tool Selection Logic

- [x] Implement two-phase selection architecture
  - [x] Create module for tool selection phase
  - [x] Integrate with existing agent phase
  - [x] Define interfaces between phases

- [x] Develop selection algorithm
  - [x] Create prompt engineering for tool selection
  - [x] Implement relevance scoring mechanism
  - [x] Add filtering based on relevance threshold

- [x] Add conversation context tracking
  - [x] Track conversation history for context-aware selection
  - [x] Implement heuristics for when to re-select tools

- [x] Implement optimization strategies
  - [x] Add batched processing of modules
  - [x] Implement caching of selection results
  - [ ] Add background processing for tool selection

## User Interface

- [x] Enhance `/tools` command
  - [x] Update to show only dynamically selected tools by default
  - [x] Add `-a, --all` flag to show all tools
  - [x] Use visual indicators for active vs. inactive tools

- [x] Add `/tools select` command
  - [x] Implement force re-selection of tools
  - [x] Add feedback on selection process

- [x] Improve status reporting
  - [x] Add loading indicator during tool selection
  - [x] Show summary of selected tools

## Configuration and Persistence

- [x] Update configuration file format
  - [x] Add `dynamic` field to server configuration
  - [x] Ensure backward compatibility

- [x] Implement module configuration validation
  - [x] Validate module definitions against schema
  - [x] Add warning for invalid module definitions

- [x] Add caching mechanism
  - [x] Cache tool selection results
  - [x] Implement cache invalidation based on conversation context

## Testing and Evaluation

- [x] Create unit tests
  - [x] Test dynamic flag in MCP server configuration
  - [x] Test tool selection algorithm
  - [x] Test module configuration parsing
  - [x] Test dynamic vs. pinned behavior

- [x] Develop integration tests
  - [x] Test end-to-end tool selection flow
  - [x] Test with various conversation contexts

- [ ] Create synthetic benchmarks
  - [ ] Measure selection accuracy
  - [ ] Measure selection latency
  - [ ] Test with varying numbers of tools

- [x] Real-world testing
  - [x] Test LLM-based selection with AWS MCP server
  - [x] Test fallback to heuristic when LLM is unavailable
  - [ ] Test with multiple dynamic servers
  - [x] Verify that non-dynamic tools remain available

## ValidationException Investigation

- [x] Debug ValidationException on second query after tool selection
  - [x] Identified state contamination in CodewhispererStreamingClient
  - [x] Confirmed issue by skipping tool selection after first call
  - [x] Documented findings in validation-exception-investigation.md
- [x] Implement solution for ValidationException
  - [x] Add tool_selection_api_client field to ToolManager struct
  - [x] Initialize tool selection API client in ToolManager::new when dynamic servers exist
  - [x] Update filter_tools_dynamically to use dedicated tool selection client
  - [x] Pass tool selection client to ToolSelector instead of main conversation client
  - [x] Add lazy initialization to avoid creating client if not needed
  - [x] Share authentication and configuration with main client
  - [x] Add error handling for missing tool selection client
  - [x] Update ToolManager initialization in chat mod.rs
  - [x] Test with AWS MCP server to verify ValidationException is resolved
  - [x] Verify tool selection works correctly with dedicated client
  - [x] Test that non-dynamic servers still work without tool selection client
  - [x] Remove temporary debug logging from api_client mod.rs
  - [ ] Add unit tests for tool selection client initialization
  - [ ] Add integration test to verify client isolation

## ValidationException - Further Investigation Needed

- [x] Separate API client implementation completed and working
- [ ] ValidationException persists - appears to be backend issue
- [ ] Investigate specific conversation patterns that trigger the error
- [ ] Test with different history lengths and message formats
- [ ] Consider alternative workarounds for the backend issue
- [ ] Report detailed findings to AWS Q team

## Documentation

- [ ] Update user documentation
  - [ ] Document `--dynamic` flag for MCP servers
  - [ ] Document `/tools` command enhancements
  - [ ] Provide examples of module definitions
  - [ ] Document server instructions feature

- [ ] Create developer documentation
  - [ ] Document tool selection architecture
  - [ ] Document module configuration format
  - [ ] Provide examples for MCP server authors
  - [ ] Document how to use server instructions
