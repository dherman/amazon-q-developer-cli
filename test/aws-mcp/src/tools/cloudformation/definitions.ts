/**
 * Tool definitions for CloudFormation service
 */
export const cloudformationTools = [
  {
    name: 'cloudformation_list_stacks',
    description: 'Lists CloudFormation stacks',
    parameters: {
      type: 'object',
      properties: {
        stackStatusFilter: {
          type: 'array',
          items: {
            type: 'string'
          },
          description: 'Stack status to filter'
        }
      },
      required: []
    }
  },
  {
    name: 'cloudformation_create_stack',
    description: 'Creates a CloudFormation stack',
    parameters: {
      type: 'object',
      properties: {
        stackName: {
          type: 'string',
          description: 'The name of the stack to create'
        },
        templateBody: {
          type: 'string',
          description: 'Structure containing the template body'
        },
        templateURL: {
          type: 'string',
          description: 'Location of file containing the template body'
        },
        parameters: {
          type: 'array',
          description: 'A list of Parameter structures that specify input parameters for the stack'
        },
        capabilities: {
          type: 'array',
          description: 'Capabilities that you must specify to allow certain stack resources'
        }
      },
      required: ['stackName']
    }
  },
  {
    name: 'cloudformation_delete_stack',
    description: 'Deletes a CloudFormation stack',
    parameters: {
      type: 'object',
      properties: {
        stackName: {
          type: 'string',
          description: 'The name or the unique stack ID of the stack to delete'
        }
      },
      required: ['stackName']
    }
  },
  {
    name: 'cloudformation_describe_stack_events',
    description: 'Returns all stack related events for a specified stack',
    parameters: {
      type: 'object',
      properties: {
        stackName: {
          type: 'string',
          description: 'The name or the unique stack ID of the stack'
        },
        nextToken: {
          type: 'string',
          description: 'A string that identifies the next page of events'
        }
      },
      required: ['stackName']
    }
  }
];
