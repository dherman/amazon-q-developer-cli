#!/usr/bin/env node

import { createServer } from './server';
import { registerTools } from './tools';
import { registerModules } from './modules';
import * as fs from 'fs';
import * as path from 'path';

// Set up logging to a file
const logFile = path.join(__dirname, '../mcp-server.log');
const logStream = fs.createWriteStream(logFile, { flags: 'a' });

// Override console.error to write to the log file
const originalConsoleError = console.error;
console.error = function(...args) {
  const message = args.map(arg => 
    typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
  ).join(' ');
  
  logStream.write(`${new Date().toISOString()} - ${message}\n`);
  originalConsoleError.apply(console, args);
};

/**
 * Main entry point for the AWS MCP server
 */
async function main() {
  try {
    // Add debug logging
    console.error('=== AWS MCP SERVER STARTING ===');
    console.error(`Process ID: ${process.pid}`);
    console.error(`Current directory: ${process.cwd()}`);
    console.error(`Node version: ${process.version}`);
    
    // Create the MCP server
    console.error('Creating MCP server...');
    const server = createServer();
    
    // Register all modules
    console.error('Registering modules...');
    registerModules(server);
    
    // Register all tools
    console.error('Registering tools...');
    registerTools(server);
    
    // Log the number of tools and modules
    console.error(`Server has ${server.getToolCount()} tools and ${server.getModuleCount()} modules`);
    
    // Log all modules and tools
    console.error('=== MODULES ===');
    server.getAllModules().forEach(module => {
      console.error(`Module: ${module.name}`);
      console.error(`  Description: ${module.description}`);
      console.error(`  Tools: ${module.tools.join(', ')}`);
    });
    
    console.error('=== TOOLS ===');
    server.getAllTools().forEach(tool => {
      console.error(`Tool: ${tool.name}`);
      console.error(`  Description: ${tool.description}`);
    });
    
    // Start the server
    console.error('Starting server...');
    server.start();
    
    console.error('=== AWS MCP SERVER STARTED SUCCESSFULLY ===');
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
}

// Handle process exit
process.on('exit', () => {
  console.error('Process exiting, closing log stream');
  logStream.end();
});

// Run the server
main();
