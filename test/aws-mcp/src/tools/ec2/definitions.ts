/**
 * Tool definitions for EC2 service
 */
export const ec2Tools = [
  {
    name: 'ec2_describe_instances',
    description: 'Describes one or more EC2 instances',
    parameters: {
      type: 'object',
      properties: {
        instanceIds: {
          type: 'array',
          items: {
            type: 'string'
          },
          description: 'One or more instance IDs'
        },
        filters: {
          type: 'object',
          description: 'Filters to apply to the request'
        }
      },
      required: []
    }
  },
  {
    name: 'ec2_run_instances',
    description: 'Launches one or more EC2 instances',
    parameters: {
      type: 'object',
      properties: {
        imageId: {
          type: 'string',
          description: 'The ID of the AMI'
        },
        instanceType: {
          type: 'string',
          description: 'The instance type'
        },
        minCount: {
          type: 'integer',
          description: 'The minimum number of instances to launch'
        },
        maxCount: {
          type: 'integer',
          description: 'The maximum number of instances to launch'
        },
        keyName: {
          type: 'string',
          description: 'The name of the key pair'
        }
      },
      required: ['imageId', 'instanceType', 'minCount', 'maxCount']
    }
  },
  {
    name: 'ec2_terminate_instances',
    description: 'Terminates one or more EC2 instances',
    parameters: {
      type: 'object',
      properties: {
        instanceIds: {
          type: 'array',
          items: {
            type: 'string'
          },
          description: 'One or more instance IDs'
        }
      },
      required: ['instanceIds']
    }
  },
  {
    name: 'ec2_start_instances',
    description: 'Starts one or more stopped EC2 instances',
    parameters: {
      type: 'object',
      properties: {
        instanceIds: {
          type: 'array',
          items: {
            type: 'string'
          },
          description: 'One or more instance IDs'
        }
      },
      required: ['instanceIds']
    }
  },
  {
    name: 'ec2_stop_instances',
    description: 'Stops one or more running EC2 instances',
    parameters: {
      type: 'object',
      properties: {
        instanceIds: {
          type: 'array',
          items: {
            type: 'string'
          },
          description: 'One or more instance IDs'
        },
        force: {
          type: 'boolean',
          description: 'Forces the instances to stop'
        }
      },
      required: ['instanceIds']
    }
  }
];
