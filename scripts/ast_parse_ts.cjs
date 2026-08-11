#!/usr/bin/env node
/**
 * TypeScript / JavaScript AST Node Parser (ast-parser@v1)
 *
 * Extracts interface declarations, method names, line numbers, and parameter
 * types from TypeScript/JS source files into unified AST JSON format.
 */

const fs = require('fs');
const path = require('path');

function parseTypeScriptFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  const interfaces = [];

  let currentInterface = null;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    const lineNo = i + 1;

    // Match interface or type declaration: `export interface CheckoutPort {`
    const ifaceMatch = line.match(/(?:export\s+)?(?:interface|type)\s+([A-Za-z0-9_]+)/);
    if (ifaceMatch) {
      if (currentInterface) {
        interfaces.push(currentInterface);
      }
      currentInterface = {
        name: ifaceMatch[1],
        line: lineNo,
        methods: []
      };
      continue;
    }

    if (currentInterface) {
      // Match closing brace
      if (line === '}' || line.startsWith('};')) {
        interfaces.push(currentInterface);
        currentInterface = null;
        continue;
      }

      // Match method signature: `processOrder(order: OrderPayload): Promise<OrderResult>;`
      const methodMatch = line.match(/([A-Za-z0-9_]+)\s*\(([^)]*)\)\s*:\s*([^;]+)/);
      if (methodMatch) {
        const methodName = methodMatch[1];
        const rawParams = methodMatch[2].trim();
        const returnType = methodMatch[3].trim();

        const params = rawParams ? rawParams.split(',').map(p => {
          const parts = p.trim().split(':');
          return {
            name: parts[0] ? parts[0].trim().replace(/\?$/, '') : '',
            type: parts[1] ? parts[1].trim() : 'any'
          };
        }) : [];

        currentInterface.methods.push({
          name: methodName,
          line: lineNo,
          params: params,
          returnType: returnType
        });
      }
    }
  }

  if (currentInterface) {
    interfaces.push(currentInterface);
  }

  return {
    file: filePath,
    language: 'typescript',
    interfaces: interfaces
  };
}

if (process.argv.length < 3) {
  console.error('Usage: node ast_parse_ts.js <file-path>');
  process.exit(1);
}

const filePath = process.argv[2];
try {
  const result = parseTypeScriptFile(filePath);
  console.log(JSON.stringify(result, null, 2));
} catch (err) {
  console.error(`Error parsing ${filePath}: ${err.message}`);
  process.exit(1);
}
