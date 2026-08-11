#!/usr/bin/env node
/**
 * TypeScript / JavaScript AST Node Parser (ast-parser@v1)
 *
 * Extracts interface/class/type declarations, method names, line numbers, and
 * parameter types from TypeScript/JS source files into unified AST JSON format.
 */

const fs = require('fs');

function parseTypeScriptFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  const interfaces = [];

  let currentInterface = null;
  let braceDepth = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    const lineNo = i + 1;

    if (!line || line.startsWith('//') || line.startsWith('/*') || line.startsWith('*')) {
      continue;
    }

    // Match interface, class, or type declaration: `export class UserMemoryAdapter {`
    if (!currentInterface) {
      const ifaceMatch = line.match(/(?:export\s+|public\s+)?(?:interface|type|class|struct)\s+([A-Za-z0-9_]+)/);
      if (ifaceMatch) {
        currentInterface = {
          name: ifaceMatch[1],
          line: lineNo,
          methods: []
        };
        braceDepth = (line.match(/\{/g) || []).length - (line.match(/\}/g) || []).length;
        continue;
      }
    } else {
      // Track inner brace depth
      const openBraces = (line.match(/\{/g) || []).length;
      const closeBraces = (line.match(/\}/g) || []).length;
      braceDepth += openBraces - closeBraces;

      if (braceDepth <= 0) {
        interfaces.push(currentInterface);
        currentInterface = null;
        braceDepth = 0;
        continue;
      }

      // Match method signature or implementation: `getUser(id: string): Promise<User> {`
      const isStatement = /^(?:return|if|for|while|const|let|var|throw|await|async|import|export)\b/.test(line);
      const methodMatch = !isStatement && line.match(/([A-Za-z0-9_]+)\s*\(([^)]*)\)(?:\s*:\s*([^;{\n]+))?/);
      if (methodMatch) {
        const methodName = methodMatch[1];
        const rawParams = methodMatch[2] ? methodMatch[2].trim() : '';
        const returnType = methodMatch[3] ? methodMatch[3].trim() : 'any';

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
  console.error('Usage: node ast_parse_ts.cjs <file-path>');
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
