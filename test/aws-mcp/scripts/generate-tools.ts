#!/usr/bin/env ts-node

/**
 * Script to generate tool definitions for AWS services
 * 
 * This script would normally parse AWS SDK service models to generate
 * tool definitions, but for this example we'll just generate some
 * placeholder definitions for additional services.
 */

import * as fs from 'fs';
import * as path from 'path';

// Define additional AWS services
const additionalServices = [
  {
    name: 'rds',
    description: 'Amazon RDS is a managed relational database service',
    operations: [
      { name: 'describe_db_instances', description: 'Returns information about provisioned RDS instances' },
      { name: 'create_db_instance', description: 'Creates a new DB instance' },
      { name: 'delete_db_instance', description: 'Deletes a DB instance' },
      { name: 'start_db_instance', description: 'Starts a DB instance that was stopped' },
      { name: 'stop_db_instance', description: 'Stops a DB instance' }
    ]
  },
  {
    name: 'sns',
    description: 'Amazon SNS is a web service for application-to-application and application-to-person notifications',
    operations: [
      { name: 'list_topics', description: 'Returns a list of the requester\'s topics' },
      { name: 'create_topic', description: 'Creates a topic to which notifications can be published' },
      { name: 'delete_topic', description: 'Deletes a topic and all its subscriptions' },
      { name: 'publish', description: 'Sends a message to an Amazon SNS topic' },
      { name: 'subscribe', description: 'Subscribes an endpoint to an Amazon SNS topic' }
    ]
  },
  {
    name: 'sqs',
    description: 'Amazon SQS is a message queuing service',
    operations: [
      { name: 'list_queues', description: 'Returns a list of your queues' },
      { name: 'create_queue', description: 'Creates a new standard or FIFO queue' },
      { name: 'delete_queue', description: 'Deletes the queue specified by the QueueUrl' },
      { name: 'send_message', description: 'Delivers a message to the specified queue' },
      { name: 'receive_message', description: 'Retrieves one or more messages from the specified queue' }
    ]
  },
  {
    name: 'iam',
    description: 'AWS Identity and Access Management (IAM) enables you to manage access to AWS services and resources securely',
    operations: [
      { name: 'list_users', description: 'Lists the IAM users that have the specified path prefix' },
      { name: 'create_user', description: 'Creates a new IAM user' },
      { name: 'delete_user', description: 'Deletes the specified IAM user' },
      { name: 'list_roles', description: 'Lists the IAM roles that have the specified path prefix' },
      { name: 'create_role', description: 'Creates a new role for your AWS account' }
    ]
  },
  {
    name: 'cloudwatch',
    description: 'Amazon CloudWatch monitors your Amazon Web Services (AWS) resources and the applications you run on AWS in real time',
    operations: [
      { name: 'list_metrics', description: 'Lists the specified metrics' },
      { name: 'get_metric_data', description: 'Retrieves metric data from Amazon CloudWatch' },
      { name: 'put_metric_data', description: 'Publishes metric data points to Amazon CloudWatch' },
      { name: 'describe_alarms', description: 'Retrieves the specified alarms' },
      { name: 'put_metric_alarm', description: 'Creates or updates an alarm and associates it with the specified metric' }
    ]
  }
];

/**
 * Generate a tool definition file for a service
 */
function generateServiceDefinition(service: any): string {
  const toolDefinitions = service.operations.map((op: any) => {
    const toolName = `${service.name}_${op.name}`;
    return `  {
    name: '${toolName}',
    description: '${op.description}',
    parameters: {
      type: 'object',
      properties: {
        // Parameters would be generated from AWS SDK models
      },
      required: []
    }
  }`;
  }).join(',\n');

  return `/**
 * Tool definitions for ${service.name.toUpperCase()} service
 */
export const ${service.name}Tools = [
${toolDefinitions}
];`;
}

/**
 * Generate a module definition
 */
function generateModuleDefinition(service: any): string {
  const toolNames = service.operations.map((op: any) => `'${service.name}_${op.name}'`).join(', ');
  
  return `  {
    name: '${service.name}',
    description: '${service.description}',
    tools: [${toolNames}]
  }`;
}

/**
 * Update the modules.ts file with new modules
 */
function updateModulesFile(services: any[]): void {
  const modulesPath = path.join(__dirname, '../src/modules.ts');
  const moduleDefinitions = services.map(generateModuleDefinition).join(',\n');
  
  // Read the current file
  let content = fs.readFileSync(modulesPath, 'utf8');
  
  // Find the modules array
  const modulesArrayStart = content.indexOf('const modules: Module[] = [');
  const modulesArrayEnd = content.indexOf('  // Additional modules would be defined here');
  
  if (modulesArrayStart !== -1 && modulesArrayEnd !== -1) {
    // Insert the new module definitions
    content = content.substring(0, modulesArrayEnd) + 
              moduleDefinitions + ',\n' +
              content.substring(modulesArrayEnd);
    
    // Write the updated file
    fs.writeFileSync(modulesPath, content);
    console.log('Updated modules.ts with new module definitions');
  } else {
    console.error('Could not find the modules array in modules.ts');
  }
}

/**
 * Main function to generate tool definitions
 */
async function main() {
  try {
    // Create directories for each service
    for (const service of additionalServices) {
      const serviceDir = path.join(__dirname, `../src/tools/${service.name}`);
      if (!fs.existsSync(serviceDir)) {
        fs.mkdirSync(serviceDir, { recursive: true });
      }
      
      // Generate definitions file
      const definitionsPath = path.join(serviceDir, 'definitions.ts');
      fs.writeFileSync(definitionsPath, generateServiceDefinition(service));
      console.log(`Generated definitions for ${service.name}`);
    }
    
    // Update the modules file
    updateModulesFile(additionalServices);
    
    // Update the tools.ts file to import and register the new tools
    // This would be more complex in a real implementation
    console.log('Tool definitions generated successfully');
  } catch (error) {
    console.error('Error generating tool definitions:', error);
    process.exit(1);
  }
}

// Run the script
main();
