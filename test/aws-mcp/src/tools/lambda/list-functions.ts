import { LambdaClient, ListFunctionsCommand } from '@aws-sdk/client-lambda';
import { Tool } from '../../types/tool';

// Create Lambda client with hardcoded region
const lambdaClient = new LambdaClient({ region: 'us-east-1' });

/**
 * Tool to list Lambda functions
 */
export const lambdaListFunctions: Tool = {
  name: 'lambda_list_functions',
  description: 'Lists Lambda functions',
  parameters: {
    type: 'object',
    properties: {
      maxItems: {
        type: 'integer',
        description: 'Maximum number of functions to return'
      },
      functionVersion: {
        type: 'string',
        description: 'Function version to list'
      }
    },
    required: []
  },
  async execute(args: any) {
    try {
      const command = new ListFunctionsCommand({
        MaxItems: args.maxItems,
        FunctionVersion: args.functionVersion
      });
      
      const response = await lambdaClient.send(command);
      
      return {
        status: 'success',
        functions: response.Functions?.map((func: any) => ({
          functionName: func.FunctionName,
          functionArn: func.FunctionArn,
          runtime: func.Runtime,
          handler: func.Handler,
          codeSize: func.CodeSize,
          description: func.Description,
          timeout: func.Timeout,
          memorySize: func.MemorySize,
          lastModified: func.LastModified,
          version: func.Version,
          role: func.Role
        })) || [],
        nextMarker: response.NextMarker
      };
    } catch (error: any) {
      return {
        status: 'error',
        message: error.message || 'Failed to list Lambda functions'
      };
    }
  }
};