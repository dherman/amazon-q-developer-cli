# AWS MCP Server

A Model Context Protocol (MCP) server that exposes AWS services as tools for Amazon Q CLI.

## Overview

This MCP server advertises hundreds of AWS service operations as tools, organized into modules based on AWS services. It's designed to test the dynamic MCP selection feature of Amazon Q CLI by providing a large surface area of tools.

**Note:** This is a test implementation that only fully implements a small subset of the advertised tools.

## Features

- Hundreds of AWS service operations advertised as tools
- Tools organized into modules based on AWS services
- Full implementation of selected tools:
  - `s3_list_buckets`: List all S3 buckets
  - `s3_list_objects`: List objects in a bucket
  - `ec2_describe_instances`: List EC2 instances

## Installation

1. Clone this repository
2. Install dependencies: `npm install`
3. Build the server: `npm run build`
4. Register with Q CLI: `q mcp add aws-mcp --path /path/to/aws-mcp/dist/index.js --dynamic`

## Usage

Once registered with Q CLI, the AWS MCP server will be available for use in Q chat sessions. The dynamic MCP selection feature will automatically select relevant AWS service tools based on the conversation context.

Example conversation:

```
> How can I list my S3 buckets?

I can help you list your S3 buckets using the AWS CLI. Let me do that for you.

[Using tool: aws-mcp___s3_list_buckets]

Here are your S3 buckets:
- example-bucket-1 (Created: 2023-01-15)
- example-bucket-2 (Created: 2023-03-22)
- example-bucket-3 (Created: 2023-06-10)
```

## Development

- `npm run build`: Build the server
- `npm run test`: Run tests
- `npm run generate`: Generate tool definitions from AWS SDK

## Architecture

The AWS MCP server is organized into the following components:

- **Server Core**: Handles MCP protocol communication
- **Module Registry**: Maintains metadata about AWS service modules
- **Tool Registry**: Maintains information about all tools
- **AWS SDK Interface**: Executes AWS operations for implemented tools

## License

MIT
