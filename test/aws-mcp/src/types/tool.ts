/**
 * Interface for MCP tool definitions
 */
export interface Tool {
  /**
   * Name of the tool
   */
  name: string;

  /**
   * Description of what the tool does
   */
  description: string;

  /**
   * JSON Schema for tool parameters
   */
  parameters: {
    type: string;
    properties: Record<string, any>;
    required?: string[];
  };

  /**
   * Execute the tool with the given parameters
   */
  execute(params: any): Promise<any>;
}
