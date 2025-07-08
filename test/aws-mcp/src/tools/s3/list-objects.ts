import { S3Client, ListObjectsV2Command } from '@aws-sdk/client-s3';
import { Tool } from '../../types/tool';

// Create S3 client
const s3Client = new S3Client({});

/**
 * Tool to list objects in an S3 bucket
 */
export const s3ListObjects: Tool = {
  name: 's3_list_objects',
  description: 'Lists objects in an S3 bucket',
  parameters: {
    type: 'object',
    properties: {
      bucket: {
        type: 'string',
        description: 'The name of the S3 bucket'
      },
      prefix: {
        type: 'string',
        description: 'Filter objects by prefix'
      },
      maxKeys: {
        type: 'integer',
        description: 'Maximum number of objects to return'
      }
    },
    required: ['bucket']
  },
  async execute(params) {
    try {
      const command = new ListObjectsV2Command({
        Bucket: params.bucket,
        Prefix: params.prefix,
        MaxKeys: params.maxKeys
      });
      
      const response = await s3Client.send(command);
      
      return {
        status: 'success',
        bucket: params.bucket,
        prefix: params.prefix || '',
        objects: response.Contents?.map(object => ({
          key: object.Key,
          size: object.Size,
          lastModified: object.LastModified?.toISOString(),
          etag: object.ETag
        })) || [],
        isTruncated: response.IsTruncated || false,
        keyCount: response.KeyCount || 0
      };
    } catch (error: any) {
      return {
        status: 'error',
        message: error.message || 'Failed to list objects in bucket'
      };
    }
  }
};
