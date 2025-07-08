# AWS MCP Server Design

## Architecture Overview

The AWS MCP server will be designed as a Model Context Protocol (MCP) server that exposes AWS services as tools. It will advertise hundreds of tools organized into modules based on AWS services, but will only implement a small subset for testing purposes.

```
┌─────────────────────┐
│     Q CLI Client    │
└─────────┬───────────┘
          │
          │ MCP Protocol
          ▼
┌─────────────────────┐
│   AWS MCP Server    │
├─────────────────────┤
│  Module Definitions │
├─────────────────────┤
│    Tool Registry    │
├─────────────────────┤
│  AWS SDK Interface  │
└─────────────────────┘
```

## Component Design

### 1. Server Core

The server core will handle MCP protocol communication, including:
- JSON-RPC message handling
- Tool registration and discovery
- Request routing to appropriate tool implementations

### 2. Module Registry

The module registry will maintain the metadata about AWS service modules:
- Module name and description
- List of tools in each module
- Relationships between modules

### 3. Tool Registry

The tool registry will maintain information about all tools:
- Tool name, description, and parameters
- Implementation status (implemented or stub)
- Mapping to AWS SDK operations

### 4. AWS SDK Interface

A thin wrapper around the AWS SDK or AWS CLI to execute the implemented tools:
- Authentication handling
- Parameter validation
- Response formatting

## Module Structure

Each AWS service will be represented as a module with the following structure:

```json
{
  "name": "service-name",
  "description": "Human-readable description of the service",
  "tools": ["service_operation1", "service_operation2", ...]
}
```

For example:

```json
{
  "name": "s3",
  "description": "Amazon S3 is object storage built to store and retrieve any amount of data from anywhere",
  "tools": ["s3_list_buckets", "s3_list_objects", "s3_create_bucket", ...]
}
```

## Tool Design

Each tool will follow a consistent naming and structure pattern:

### Naming Convention

Tools will be named using the pattern: `{service}_{operation}`

Examples:
- `s3_list_buckets`
- `ec2_describe_instances`
- `lambda_invoke_function`

### Tool Definition

```json
{
  "name": "service_operation",
  "description": "Human-readable description of what the operation does",
  "parameters": {
    "type": "object",
    "properties": {
      "param1": {
        "type": "string",
        "description": "Description of parameter 1"
      },
      "param2": {
        "type": "integer",
        "description": "Description of parameter 2"
      }
    },
    "required": ["param1"]
  }
}
```

### Implementation Approach

For implemented tools:
1. Validate input parameters
2. Convert parameters to AWS SDK format
3. Execute AWS SDK operation
4. Format response for MCP protocol

For stub tools:
1. Return a "not implemented" message with information about the tool

## Data Flow

1. **Tool Discovery**:
   - Q CLI requests tool list from AWS MCP server
   - Server returns all tool definitions with module metadata
   - Q CLI uses dynamic selection to choose relevant tools

2. **Tool Execution**:
   - Q CLI sends tool execution request
   - Server validates request
   - If tool is implemented, execute AWS SDK operation
   - If tool is a stub, return "not implemented" message
   - Return formatted response to Q CLI

## Implementation Details

### Server Implementation

The server will be implemented as a standalone process that communicates via stdin/stdout following the MCP protocol.

### Tool Generation

Tool definitions will be generated from AWS SDK documentation:
1. Parse AWS SDK service models
2. Extract operation names, descriptions, and parameters
3. Generate tool definitions in MCP format
4. Organize tools into service-based modules

### Implemented Tools

The following tools will be fully implemented:

1. `s3_list_buckets`:
   - No required parameters
   - Returns list of bucket names and creation dates

2. `s3_list_objects`:
   - Required parameter: `bucket` (string)
   - Optional parameters: `prefix` (string), `max_keys` (integer)
   - Returns list of objects with key, size, and last modified date

3. `ec2_describe_instances`:
   - Optional parameters: `instance_ids` (array of strings), `filters` (object)
   - Returns list of instances with id, type, state, and tags

## Testing Strategy

### Unit Testing

- Test tool definition generation
- Test parameter validation
- Test AWS SDK interface

### Integration Testing

- Test MCP protocol compliance
- Test tool discovery and execution
- Test error handling

### Dynamic Selection Testing

- Test with various conversation contexts
- Verify appropriate module selection
- Measure selection performance with large tool set

## Future Considerations

1. **Authentication**: Enhance authentication options beyond default AWS credentials
2. **Pagination**: Add support for paginated results from AWS operations
3. **Caching**: Implement caching for frequently used operations
4. **Error Handling**: Improve error messages and recovery mechanisms
5. **Implementation Coverage**: Gradually implement more tools based on usage patterns
