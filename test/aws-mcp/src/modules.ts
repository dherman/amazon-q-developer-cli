import { McpServer } from './server';
import { Module } from './types/module';

// Define AWS service modules
const modules: Module[] = [
  {
    name: 's3',
    description: 'Amazon S3 is object storage built to store and retrieve any amount of data from anywhere',
    tools: ['s3_list_buckets', 's3_list_objects', 's3_create_bucket', 's3_delete_bucket', 's3_upload_file', 's3_download_file']
  },
  {
    name: 'ec2',
    description: 'Amazon EC2 is a web service that provides secure, resizable compute capacity in the cloud',
    tools: ['ec2_describe_instances', 'ec2_run_instances', 'ec2_terminate_instances', 'ec2_start_instances', 'ec2_stop_instances']
  },
  {
    name: 'lambda',
    description: 'AWS Lambda is a serverless compute service that lets you run code without provisioning or managing servers',
    tools: ['lambda_list_functions', 'lambda_invoke_function', 'lambda_create_function', 'lambda_delete_function', 'lambda_update_function']
  },
  {
    name: 'dynamodb',
    description: 'Amazon DynamoDB is a key-value and document database that delivers single-digit millisecond performance at any scale',
    tools: ['dynamodb_list_tables', 'dynamodb_create_table', 'dynamodb_delete_table', 'dynamodb_put_item', 'dynamodb_get_item', 'dynamodb_query']
  },
  {
    name: 'cloudformation',
    description: 'AWS CloudFormation provides a common language to describe and provision all the infrastructure resources in your cloud environment',
    tools: ['cloudformation_list_stacks', 'cloudformation_create_stack', 'cloudformation_delete_stack', 'cloudformation_describe_stack_events']
  }
  // Additional modules would be defined here
];

/**
 * Register all modules with the server
 */
export function registerModules(server: McpServer): void {
  modules.forEach(module => {
    server.registerModule(module);
  });
}
