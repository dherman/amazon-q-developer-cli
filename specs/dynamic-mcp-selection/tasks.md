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

- [x] Modify tool manager to support dynamic selection
  - [x] Update `ToolManager` to track which servers are dynamic vs. pinned
  - [x] Add logic to determine when to trigger tool selection
  - [x] Implement caching of tool selection results

## Tool Selection Logic

- [x] Implement two-phase selection architecture
  - [x] Create module for tool selection phase
  - [ ] Integrate with existing agent phase
  - [x] Define interfaces between phases

- [x] Develop selection algorithm
  - [x] Create prompt engineering for tool selection
  - [x] Implement relevance scoring mechanism
  - [x] Add filtering based on relevance threshold

- [x] Add conversation context tracking
  - [x] Track conversation history for context-aware selection
  - [x] Implement heuristics for when to re-select tools

- [ ] Implement optimization strategies
  - [ ] Add batched processing of modules
  - [ ] Implement caching of selection results
  - [ ] Add background processing for tool selection

## User Interface

- [ ] Enhance `/tools` command
  - [ ] Update to show only dynamically selected tools by default
  - [ ] Add `-a, --all` flag to show all tools
  - [ ] Use visual indicators for active vs. inactive tools

- [x] Add `/tools select` command
  - [x] Implement force re-selection of tools
  - [x] Add feedback on selection process

- [ ] Improve status reporting
  - [ ] Add loading indicator during tool selection
  - [ ] Show summary of selected tools

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
  - [ ] Test dynamic vs. pinned behavior

- [ ] Develop integration tests
  - [ ] Test end-to-end tool selection flow
  - [ ] Test with various conversation contexts

- [ ] Create synthetic benchmarks
  - [ ] Measure selection accuracy
  - [ ] Measure selection latency
  - [ ] Test with varying numbers of tools

## Documentation

- [ ] Update user documentation
  - [ ] Document `--dynamic` flag for MCP servers
  - [ ] Document `/tools` command enhancements
  - [ ] Provide examples of module definitions

- [ ] Create developer documentation
  - [ ] Document tool selection architecture
  - [ ] Document module configuration format
  - [ ] Provide examples for MCP server authors
