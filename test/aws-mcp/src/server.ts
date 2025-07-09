import { createInterface } from 'readline';
import { Tool } from './types/tool';
import { Module } from './types/module';

/**
 * MCP Server class for handling JSON-RPC communication
 */
export class McpServer {
  private tools: Map<string, Tool> = new Map();
  private modules: Map<string, Module> = new Map();
  private readline = createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
  });
  private running = false;

  /**
   * Register a tool with the server
   */
  registerTool(tool: Tool): void {
    console.error(`Registering tool: ${tool.name}`);
    this.tools.set(tool.name, tool);
  }

  /**
   * Register a module with the server
   */
  registerModule(module: Module): void {
    console.error(`Registering module: ${module.name} with ${module.tools.length} tools`);
    this.modules.set(module.name, module);
  }

  /**
   * Get the number of tools registered
   */
  getToolCount(): number {
    return this.tools.size;
  }

  /**
   * Get the number of modules registered
   */
  getModuleCount(): number {
    return this.modules.size;
  }

  /**
   * Get all modules
   */
  getAllModules(): Module[] {
    return Array.from(this.modules.values());
  }

  /**
   * Get all tools
   */
  getAllTools(): Tool[] {
    return Array.from(this.tools.values());
  }

  /**
   * Start the server and listen for requests
   */
  start(): void {
    if (this.running) {
      console.error('Server is already running');
      return;
    }
    
    this.running = true;
    console.error(`Server starting with ${this.tools.size} tools and ${this.modules.size} modules`);
    
    // Keep the process alive
    setInterval(() => {
      // This empty interval prevents Node.js from exiting
      // It will run every 60 seconds
      console.error('Server heartbeat - still alive');
    }, 60000);
    
    this.readline.on('line', async (line) => {
      try {
        console.error(`Received request: ${line.substring(0, 100)}${line.length > 100 ? '...' : ''}`);
        const request = JSON.parse(line);
        const response = await this.handleRequest(request);
        console.error(`Sending response: ${JSON.stringify(response).substring(0, 100)}...`);
        console.log(JSON.stringify(response));
      } catch (error) {
        console.error('Error processing request:', error);
        console.log(JSON.stringify({
          jsonrpc: '2.0',
          id: null,
          error: {
            code: -32700,
            message: 'Parse error'
          }
        }));
      }
    });

    // Handle process signals
    process.on('SIGINT', () => {
      console.error('Received SIGINT signal, shutting down...');
      this.stop();
      process.exit(0);
    });
    
    process.on('SIGTERM', () => {
      console.error('Received SIGTERM signal, shutting down...');
      this.stop();
      process.exit(0);
    });

    // Handle stdin closing
    process.stdin.on('end', () => {
      console.error('stdin stream ended, shutting down...');
      this.stop();
      process.exit(0);
    });

    // Log server start
    console.error('AWS MCP Server started and waiting for requests');
  }

  /**
   * Stop the server
   */
  stop(): void {
    if (!this.running) {
      return;
    }
    
    console.error('Stopping server...');
    this.running = false;
    this.readline.close();
  }

  /**
   * Handle an incoming JSON-RPC request
   */
  private async handleRequest(request: any): Promise<any> {
    const { id, method, params } = request;
    console.error(`Processing method: ${method}`);

    // Basic response structure
    const response = {
      jsonrpc: '2.0',
      id
    };

    try {
      // Handle different methods
      switch (method) {
        case 'initialize':
          console.error('Handling initialize request');
          return {
            ...response,
            result: {
              capabilities: {
                tools: {},
                prompts: {},
                resources: {},
                resourceTemplates: {},
                instructions: `
                  This AWS MCP server provides comprehensive AWS service management capabilities.
                  
                  When to use this server:
                  - For complex AWS operations involving multiple services
                  - When you need detailed AWS-specific functionality
                  - For operations requiring AWS best practices
                  
                  Module selection guidance:
                  - EC2 module: Use for virtual machine management, including launching, configuring, and monitoring EC2 instances
                  - S3 module: Use for object storage operations like creating buckets, uploading/downloading files
                  - IAM module: Use for identity and access management tasks
                  - Lambda module: Use for serverless function management
                  - CloudFormation module: Use for infrastructure as code and stack management
                  - CloudWatch module: Use for monitoring and observability
                  
                  When multiple tools with similar functionality are available, prefer this AWS MCP over the built-in use_aws tool
                  for more comprehensive AWS service management.
                `,
                metadata: {
                  modules: Array.from(this.modules.values())
                }
              }
            }
          };
          
        case 'tools/list':
          console.error('Handling tools/list request');
          const toolDefinitions = this.getToolDefinitions();
          console.error(`Returning ${toolDefinitions.length} tool definitions`);
          return {
            ...response,
            result: {
              tools: toolDefinitions,
              _meta: {
                modules: Array.from(this.modules.values())
              }
            }
          };

        case 'tools/call':
          if (!params || !params.name) {
            throw new Error('Missing tool name');
          }

          console.error(`Handling tools/call request for tool: ${params.name}`);
          const tool = this.tools.get(params.name);
          if (!tool) {
            throw new Error(`Tool not found: ${params.name}`);
          }

          const result = await tool.execute(params.arguments || {});
          return {
            ...response,
            result
          };

        default:
          throw new Error(`Method not found: ${method}`);
      }
    } catch (error: any) {
      console.error(`Error handling request: ${error.message}`);
      return {
        ...response,
        error: {
          code: -32603,
          message: error.message || 'Internal error'
        }
      };
    }
  }

  /**
   * Get all tool definitions for tools/list response
   */
  private getToolDefinitions(): any[] {
    console.error(`Getting tool definitions for ${this.tools.size} tools`);
    const toolDefs = Array.from(this.tools.values()).map(tool => {
      console.error(`Processing tool: ${tool.name}`);
      return {
        name: tool.name,
        description: tool.description,
        parameters: tool.parameters
      };
    });
    console.error(`Returning ${toolDefs.length} tool definitions`);
    return toolDefs;
  }
}

/**
 * Create a new MCP server instance
 */
export function createServer(): McpServer {
  return new McpServer();
}
