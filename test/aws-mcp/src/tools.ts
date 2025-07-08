import { McpServer } from './server';
import { Tool } from './types/tool';
import { s3ListBuckets } from './tools/s3/list-buckets';
import { s3ListObjects } from './tools/s3/list-objects';
import { ec2DescribeInstances } from './tools/ec2/describe-instances';

// Import tool definitions
import { s3Tools } from './tools/s3/definitions';
import { ec2Tools } from './tools/ec2/definitions';
import { lambdaTools } from './tools/lambda/definitions';
import { dynamodbTools } from './tools/dynamodb/definitions';
import { cloudformationTools } from './tools/cloudformation/definitions';

/**
 * Create a stub tool that returns a "not implemented" message
 */
function createStubTool(name: string, description: string, parameters: any): Tool {
  return {
    name,
    description,
    parameters,
    async execute() {
      return {
        status: 'error',
        message: `Tool ${name} is not implemented yet. This is a stub for testing dynamic MCP selection.`
      };
    }
  };
}

/**
 * Register all tools with the server
 */
export function registerTools(server: McpServer): void {
  // Register implemented tools
  server.registerTool(s3ListBuckets);
  server.registerTool(s3ListObjects);
  server.registerTool(ec2DescribeInstances);
  
  // Register stub tools for S3
  s3Tools
    .filter(tool => !['s3_list_buckets', 's3_list_objects'].includes(tool.name))
    .forEach(tool => {
      server.registerTool(createStubTool(tool.name, tool.description, tool.parameters));
    });
  
  // Register stub tools for EC2
  ec2Tools
    .filter(tool => tool.name !== 'ec2_describe_instances')
    .forEach(tool => {
      server.registerTool(createStubTool(tool.name, tool.description, tool.parameters));
    });
  
  // Register stub tools for other services
  [...lambdaTools, ...dynamodbTools, ...cloudformationTools].forEach(tool => {
    server.registerTool(createStubTool(tool.name, tool.description, tool.parameters));
  });
}
