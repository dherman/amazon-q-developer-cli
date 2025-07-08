/**
 * Tool definitions for DynamoDB service
 */
export const dynamodbTools = [
  {
    name: 'dynamodb_list_tables',
    description: 'Lists DynamoDB tables',
    parameters: {
      type: 'object',
      properties: {
        limit: {
          type: 'integer',
          description: 'Maximum number of tables to return'
        },
        exclusiveStartTableName: {
          type: 'string',
          description: 'The first table name that this operation will evaluate'
        }
      },
      required: []
    }
  },
  {
    name: 'dynamodb_create_table',
    description: 'Creates a new DynamoDB table',
    parameters: {
      type: 'object',
      properties: {
        tableName: {
          type: 'string',
          description: 'The name of the table to create'
        },
        attributeDefinitions: {
          type: 'array',
          description: 'An array of attributes that describe the key schema'
        },
        keySchema: {
          type: 'array',
          description: 'Specifies the attributes that make up the primary key'
        },
        provisionedThroughput: {
          type: 'object',
          description: 'Throughput for the specified table'
        }
      },
      required: ['tableName', 'attributeDefinitions', 'keySchema']
    }
  },
  {
    name: 'dynamodb_delete_table',
    description: 'Deletes a DynamoDB table',
    parameters: {
      type: 'object',
      properties: {
        tableName: {
          type: 'string',
          description: 'The name of the table to delete'
        }
      },
      required: ['tableName']
    }
  },
  {
    name: 'dynamodb_put_item',
    description: 'Creates a new item, or replaces an old item with a new item',
    parameters: {
      type: 'object',
      properties: {
        tableName: {
          type: 'string',
          description: 'The name of the table to write the item to'
        },
        item: {
          type: 'object',
          description: 'A map of attribute name/value pairs, one for each attribute'
        },
        conditionExpression: {
          type: 'string',
          description: 'A condition that must be satisfied in order for a conditional put to succeed'
        }
      },
      required: ['tableName', 'item']
    }
  },
  {
    name: 'dynamodb_get_item',
    description: 'Returns a set of attributes for the item with the given primary key',
    parameters: {
      type: 'object',
      properties: {
        tableName: {
          type: 'string',
          description: 'The name of the table containing the requested item'
        },
        key: {
          type: 'object',
          description: 'A map of attribute names to AttributeValue objects, representing the primary key'
        },
        projectionExpression: {
          type: 'string',
          description: 'A string that identifies one or more attributes to retrieve from the table'
        }
      },
      required: ['tableName', 'key']
    }
  },
  {
    name: 'dynamodb_query',
    description: 'Finds items based on primary key values',
    parameters: {
      type: 'object',
      properties: {
        tableName: {
          type: 'string',
          description: 'The name of the table containing the requested items'
        },
        keyConditionExpression: {
          type: 'string',
          description: 'The condition that specifies the key values for items to be retrieved'
        },
        expressionAttributeValues: {
          type: 'object',
          description: 'Values that can be substituted in an expression'
        },
        limit: {
          type: 'integer',
          description: 'The maximum number of items to evaluate'
        }
      },
      required: ['tableName', 'keyConditionExpression']
    }
  }
];
