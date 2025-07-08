#!/usr/bin/env node

import { createServer } from './server';
import { registerTools } from './tools';
import { registerModules } from './modules';

/**
 * Main entry point for the AWS MCP server
 */
async function main() {
  try {
    // Create the MCP server
    const server = createServer();
    
    // Register all modules
    registerModules(server);
    
    // Register all tools
    registerTools(server);
    
    // Start the server
    server.start();
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
}

// Run the server
main();
