import { EC2Client, DescribeInstancesCommand } from '@aws-sdk/client-ec2';
import { Tool } from '../../types/tool';

// Create EC2 client
const ec2Client = new EC2Client({});

/**
 * Tool to describe EC2 instances
 */
export const ec2DescribeInstances: Tool = {
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
  },
  async execute(params) {
    try {
      // Convert filters to AWS format if provided
      const filters = params.filters ? Object.entries(params.filters).map(([Name, Values]) => ({
        Name,
        Values: Array.isArray(Values) ? Values : [Values]
      })) : undefined;
      
      const command = new DescribeInstancesCommand({
        InstanceIds: params.instanceIds,
        Filters: filters
      });
      
      const response = await ec2Client.send(command);
      
      // Format the response
      const instances = response.Reservations?.flatMap(reservation => 
        reservation.Instances?.map(instance => ({
          instanceId: instance.InstanceId,
          instanceType: instance.InstanceType,
          state: instance.State?.Name,
          privateIpAddress: instance.PrivateIpAddress,
          publicIpAddress: instance.PublicIpAddress,
          launchTime: instance.LaunchTime?.toISOString(),
          tags: instance.Tags?.map(tag => ({
            key: tag.Key,
            value: tag.Value
          }))
        })) || []
      ) || [];
      
      return {
        status: 'success',
        instances
      };
    } catch (error: any) {
      return {
        status: 'error',
        message: error.message || 'Failed to describe EC2 instances'
      };
    }
  }
};
