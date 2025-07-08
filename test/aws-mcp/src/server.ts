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

  /**
   * Register a tool with the server
   */
  registerTool(tool: Tool): void {
    this.tools.set(tool.name, tool);
  }

  /**
   * Register a module with the server
   */
  registerModule(module: Module): void {
    this.modules.set(module.name, module);
  }

  /**
   * Start the server and listen for requests
   */
  start(): void {
    this.readline.on('line', async (line) => {
      try {
        const request = JSON.parse(line);
        const response = await this.handleRequest(request);
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

    // Log server start
    console.error('AWS MCP Server started');
  }

  /**
   * Handle an incoming JSON-RPC request
   */
  private async handleRequest(request: any): Promise<any> {
    const { id, method, params } = request;

    // Basic response structure
    const response = {
      jsonrpc: '2.0',
      id
    };

    try {
      // Handle different methods
      switch (method) {
        case 'tools/list':
          return {
            ...response,
            result: {
              tools: this.getToolDefinitions(),
              _meta: {
                modules: Array.from(this.modules.values())
              }
            }
          };

        case 'tools/call':
          if (!params || !params.name) {
            throw new Error('Missing tool name');
          }

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
    return Array.from(this.tools.values()).map(tool => ({
      name: tool.name,
      description: tool.description,
      parameters: tool.parameters
    }));
  }
}

/**
 * Create a new MCP server instance
 */
export function createServer(): McpServer {
  return new McpServer();
}
