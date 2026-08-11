#!/usr/bin/env python3
"""
Python & Polyglot AST Node Parser (ast-parser@v1)

Extracts class/protocol/interface declarations, method signatures, line numbers,
and field types into unified AST JSON format.
"""

import sys
import json
import re
import ast

def parse_python_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    interfaces = []
    try:
        tree = ast.parse(content, filename=file_path)
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                methods = []
                for item in node.body:
                    if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                        params = []
                        for arg in item.args.args:
                            if arg.arg != 'self':
                                annotation = ast.unparse(arg.annotation) if arg.annotation else 'Any'
                                params.append({"name": arg.arg, "type": annotation})
                        return_type = ast.unparse(item.returns) if item.returns else 'None'
                        methods.append({
                            "name": item.name,
                            "line": item.lineno,
                            "params": params,
                            "returnType": return_type
                        })
                interfaces.append({
                    "name": node.name,
                    "line": node.lineno,
                    "methods": methods
                })
    except Exception as e:
        # Fallback structural parse
        interfaces = parse_generic_file(file_path, content)

    return {
        "file": file_path,
        "language": "python",
        "interfaces": interfaces
    }

def parse_generic_file(file_path, content):
    """Fallback structural parser for TS, Go, Rust, C#, Java, Kotlin."""
    lines = content.splitlines()
    interfaces = []
    current_iface = None

    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Match interface/class/trait/struct definition
        m_iface = re.search(r'(?:public\s+|export\s+|pub\s+)?(?:interface|class|trait|type|struct)\s+([A-Za-z0-9_]+)', stripped)
        if m_iface:
            if current_iface:
                interfaces.append(current_iface)
            current_iface = {
                "name": m_iface.group(1),
                "line": i,
                "methods": []
            }
            continue

        if current_iface:
            if stripped == '}' or stripped.startswith('};'):
                interfaces.append(current_iface)
                current_iface = None
                continue

            # Match method declaration
            m_method = re.search(r'(?:public\s+|fn\s+|def\s+|async\s+)?([A-Za-z0-9_]+)\s*\(([^)]*)\)', stripped)
            if m_method and not stripped.startswith('//') and not stripped.startswith('#'):
                method_name = m_method.group(1)
                raw_params = m_method.group(2).strip()
                params = []
                if raw_params:
                    for p in raw_params.split(','):
                        p_stripped = p.strip()
                        if p_stripped and p_stripped != 'self' and p_stripped != '&self':
                            parts = p_stripped.split(':') if ':' in p_stripped else p_stripped.split(' ')
                            p_name = parts[0].strip() if parts else 'param'
                            p_type = parts[1].strip() if len(parts) > 1 else 'Any'
                            params.append({"name": p_name, "type": p_type})
                current_iface["methods"].append({
                    "name": method_name,
                    "line": i,
                    "params": params,
                    "returnType": "Any"
                })

    if current_iface:
        interfaces.append(current_iface)

    return interfaces

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 ast_parse_py.py <file-path>", file=sys.stderr)
        sys.exit(1)

    file_path = sys.argv[1]
    ext = file_path.split('.')[-1].lower()
    
    if ext == 'py':
        result = parse_python_file(file_path)
    else:
        with open(file_path, 'r', encoding='utf-8') as f:
            c = f.read()
        result = {
            "file": file_path,
            "language": ext,
            "interfaces": parse_generic_file(file_path, c)
        }

    print(json.dumps(result, indent=2))
