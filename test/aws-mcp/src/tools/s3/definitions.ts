/**
 * Tool definitions for S3 service
 */
export const s3Tools = [
  {
    name: 's3_list_buckets',
    description: 'Lists all S3 buckets in the AWS account',
    parameters: {
      type: 'object',
      properties: {},
      required: []
    }
  },
  {
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
    }
  },
  {
    name: 's3_create_bucket',
    description: 'Creates a new S3 bucket',
    parameters: {
      type: 'object',
      properties: {
        bucket: {
          type: 'string',
          description: 'The name of the bucket to create'
        },
        region: {
          type: 'string',
          description: 'The AWS region in which to create the bucket'
        }
      },
      required: ['bucket']
    }
  },
  {
    name: 's3_delete_bucket',
    description: 'Deletes an S3 bucket',
    parameters: {
      type: 'object',
      properties: {
        bucket: {
          type: 'string',
          description: 'The name of the bucket to delete'
        }
      },
      required: ['bucket']
    }
  },
  {
    name: 's3_upload_file',
    description: 'Uploads a file to an S3 bucket',
    parameters: {
      type: 'object',
      properties: {
        bucket: {
          type: 'string',
          description: 'The name of the bucket'
        },
        key: {
          type: 'string',
          description: 'The key (path) where the file will be stored in the bucket'
        },
        filePath: {
          type: 'string',
          description: 'The local path to the file to upload'
        }
      },
      required: ['bucket', 'key', 'filePath']
    }
  },
  {
    name: 's3_download_file',
    description: 'Downloads a file from an S3 bucket',
    parameters: {
      type: 'object',
      properties: {
        bucket: {
          type: 'string',
          description: 'The name of the bucket'
        },
        key: {
          type: 'string',
          description: 'The key (path) of the file in the bucket'
        },
        filePath: {
          type: 'string',
          description: 'The local path where the file will be saved'
        }
      },
      required: ['bucket', 'key', 'filePath']
    }
  }
];
