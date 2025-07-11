import { S3Client, ListBucketsCommand } from '@aws-sdk/client-s3';
import { Tool } from '../../types/tool';

// Create S3 client with hardcoded region
const s3Client = new S3Client({ region: 'us-east-1' });

/**
 * Tool to list all S3 buckets
 */
export const s3ListBuckets: Tool = {
  name: 's3_list_buckets',
  description: 'Lists all S3 buckets in the AWS account',
  parameters: {
    type: 'object',
    properties: {},
    required: []
  },
  async execute() {
    try {
      const command = new ListBucketsCommand({});
      const response = await s3Client.send(command);
      
      return {
        status: 'success',
        buckets: response.Buckets?.map(bucket => ({
          name: bucket.Name,
          creationDate: bucket.CreationDate?.toISOString()
        })) || []
      };
    } catch (error: any) {
      return {
        status: 'error',
        message: error.message || 'Failed to list S3 buckets'
      };
    }
  }
};
