/**
 * Tool definitions for Lambda service
 */
export const lambdaTools = [
  {
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
    }
  },
  {
    name: 'lambda_invoke_function',
    description: 'Invokes a Lambda function',
    parameters: {
      type: 'object',
      properties: {
        functionName: {
          type: 'string',
          description: 'The name or ARN of the Lambda function'
        },
        payload: {
          type: 'object',
          description: 'The payload to pass to the function'
        },
        invocationType: {
          type: 'string',
          description: 'The invocation type: RequestResponse, Event, or DryRun'
        }
      },
      required: ['functionName']
    }
  },
  {
    name: 'lambda_create_function',
    description: 'Creates a new Lambda function',
    parameters: {
      type: 'object',
      properties: {
        functionName: {
          type: 'string',
          description: 'The name of the Lambda function'
        },
        runtime: {
          type: 'string',
          description: 'The runtime identifier'
        },
        role: {
          type: 'string',
          description: 'The ARN of the function execution role'
        },
        handler: {
          type: 'string',
          description: 'The name of the method within your code that Lambda calls'
        },
        code: {
          type: 'object',
          description: 'The function code'
        }
      },
      required: ['functionName', 'runtime', 'role', 'handler', 'code']
    }
  },
  {
    name: 'lambda_delete_function',
    description: 'Deletes a Lambda function',
    parameters: {
      type: 'object',
      properties: {
        functionName: {
          type: 'string',
          description: 'The name or ARN of the Lambda function'
        }
      },
      required: ['functionName']
    }
  },
  {
    name: 'lambda_update_function',
    description: 'Updates a Lambda function',
    parameters: {
      type: 'object',
      properties: {
        functionName: {
          type: 'string',
          description: 'The name or ARN of the Lambda function'
        },
        role: {
          type: 'string',
          description: 'The ARN of the function execution role'
        },
        handler: {
          type: 'string',
          description: 'The name of the method within your code that Lambda calls'
        },
        code: {
          type: 'object',
          description: 'The function code'
        }
      },
      required: ['functionName']
    }
  }
];
