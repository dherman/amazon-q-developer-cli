# Dynamic MCP Selection Design

This document outlines the design for implementing Dynamic MCP Selection in the Amazon Q CLI, a feature that enables intelligent selection of MCP tools based on user context and queries.

## Overview

The Dynamic MCP Selection feature will allow Q CLI to work effectively with large numbers of MCP tools by dynamically selecting the most relevant ones for each user interaction. This addresses the needs of enterprise customers and large platform vendors who need to publish numerous tools without overwhelming users with manual selection.

## Architecture

We will implement a two-phase architecture inspired by the MCP-Zero paper:

1. **Tool Selector Phase**: A separate, smaller LLM evaluates user queries against tool descriptions to identify relevant tools
2. **Agent Phase**: The main LLM (already used in Q CLI) receives only the selected tools and uses them to complete the task

### Integration with Existing Architecture

The tool selection mechanism will be invoked:
- After the first prompt in a new chat
- Based on heuristics for subsequent prompts (to be determined during implementation)

The tool selector will run as a separate process from the main agent, using a smaller, more efficient LLM.

### Backward Compatibility

- Existing MCP servers will be considered "pinned" by default (always included)
- A new `"dynamic": true | false` flag will be added to MCP server configuration
- Servers installed with `"dynamic": true` opt in to the dynamic selection mechanism

## Tool Selection Strategy

### Selection Process

1. The tool selector LLM receives:
   - The user's query
   - Descriptions of available tools/modules from dynamic MCP servers
   - Context about the conversation history

2. The selector evaluates and scores the relevance of each module to the current query

3. Modules exceeding a relevance threshold are selected and their tools are made available to the main agent

### Model Selection

- A separate, smaller LLM will be used for tool selection (e.g., Claude 3.5-Sonnet)
- An abstraction layer will allow configuring which model to use for tool selection

### Performance vs. Accuracy Balance

- Initial implementation will favor performance over perfect accuracy
- A `/tools select` command will allow users to force tool re-selection for debugging

## Modules Concept

To respect vendor-defined tool relationships, we introduce the concept of "modules":

- A module is a grouping of related tools, prompts, or resources
- Each module has an English description for selection purposes
- Modules are defined by MCP server authors via a `"_meta"` field in server configuration
- Tools can belong to multiple modules
- Any tool not explicitly assigned to a module is considered its own module

Example module definition:
```json
"_meta": {
  "modules": [
    {
      "name": "AWS EC2 Management",
      "description": "Tools for creating, managing, and monitoring EC2 instances",
      "tools": ["ec2_create_instance", "ec2_describe_instances", "ec2_terminate_instance"]
    },
    {
      "name": "AWS S3 Operations",
      "description": "Tools for working with S3 buckets and objects",
      "tools": ["s3_create_bucket", "s3_list_objects", "s3_upload_file"]
    }
  ]
}
```

## Configuration and Persistence

### User Configuration

- MCP servers can be configured with `"dynamic": true | false`
- Servers are pinned (always included) by default
- Only servers with `"dynamic": true` participate in dynamic selection

### Environment Configuration

- Users will need to manually adjust their client configuration for different environments
- Future enhancements could include environment-specific configurations

## Scalability Optimizations

To handle potentially thousands of tools efficiently:

1. **Model Efficiency**: Use a smaller, faster model for tool selection

2. **Precomputation**: Eagerly precompute prompt templates for the tool selector when loading MCP servers

3. **Caching**: 
   - Cache tool lists and prompt templates to disk
   - Cache selection results for similar queries within the same conversation

4. **Background Processing**: Start running tool selection in a background thread before it's required

5. **Hierarchical Selection**: First select relevant modules, then include all tools within those modules

6. **Batched Processing**: Evaluate modules in batches rather than one by one

## User Experience

### Commands and Interfaces

- `/tools` - Shows currently selected tools (default behavior)
- `/tools -a, --all` - Shows all tools, with non-selected ones in light gray
- `/tools select` - Forces re-selection of tools based on conversation context

### Transparency

- When tool selection occurs, a status line will be displayed
- Upon completion, a single line synopsis will show (e.g., "✓ Dynamic tool selection completed in 1.10s")
- The `/tools` command provides visibility into which tools are currently active

## Implementation Phases

1. **Phase 1: Core Architecture**
   - Implement the two-phase selection architecture
   - Add the `"dynamic"` configuration option
   - Create the module metadata format

2. **Phase 2: Optimization**
   - Implement caching strategies
   - Add background processing
   - Optimize prompt templates

3. **Phase 3: User Experience**
   - Implement the `/tools` command enhancements
   - Add the tool re-selection command
   - Improve status reporting

## Evaluation Strategy

- Test with large MCP servers in real-world environments
- Develop synthetic tests with varying numbers of tools
- Measure:
  - Selection accuracy (how often the right tools are selected)
  - Selection latency
  - Overall task completion success rate
  - User satisfaction with tool selection

## Open Questions and Future Work

1. Refining heuristics for when to re-select tools during a conversation
2. Developing more sophisticated caching strategies
3. Exploring local model options for reduced latency
4. Enhancing the module concept with additional metadata
5. Adding more granular user controls if needed based on feedback
