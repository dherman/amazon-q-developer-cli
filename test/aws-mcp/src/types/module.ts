/**
 * Interface for MCP module definitions
 */
export interface Module {
  /**
   * Name of the module
   */
  name: string;

  /**
   * Description of the module
   */
  description: string;

  /**
   * List of tool names in this module
   */
  tools: string[];
}
