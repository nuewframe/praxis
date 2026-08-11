// Go AST Node Parser (ast-parser@v1)
//
// Extracts interface declarations, method names, line numbers, and parameter
// types from Go source files into unified AST JSON format.
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
)

type ParamJSON struct {
	Name string `json:"name"`
	Type string `json:"type"`
}

type MethodJSON struct {
	Name       string      `json:"name"`
	Line       int         `json:"line"`
	Params     []ParamJSON `json:"params"`
	ReturnType string      `json:"returnType"`
}

type InterfaceJSON struct {
	Name    string       `json:"name"`
	Line    int          `json:"line"`
	Methods []MethodJSON `json:"methods"`
}

type ResultJSON struct {
	File       string          `json:"file"`
	Language   string          `json:"language"`
	Interfaces []InterfaceJSON `json:"interfaces"`
}

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: go run ast_parse_go.go <file-path>\n")
		os.Exit(1)
	}

	filePath := os.Args[1]
	fset := token.NewFileSet()
	node, err := parser.ParseFile(fset, filePath, nil, parser.ParseComments)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing Go file %s: %v\n", filePath, err)
		os.Exit(1)
	}

	var interfaces []InterfaceJSON

	ast.Inspect(node, func(n ast.Node) bool {
		typeSpec, ok := n.(*ast.TypeSpec)
		if !ok {
			return true
		}

		ifaceType, ok := typeSpec.Type.(*ast.InterfaceType)
		if !ok {
			return true
		}

		line := fset.Position(typeSpec.Pos()).Line
		var methods []MethodJSON

		for _, field := range ifaceType.Methods.List {
			funcType, ok := field.Type.(*ast.FuncType)
			if !ok || len(field.Names) == 0 {
				continue
			}

			methodName := field.Names[0].Name
			methodLine := fset.Position(field.Pos()).Line
			var params []ParamJSON

			if funcType.Params != nil {
				for _, p := range funcType.Params.List {
					pType := "interface{}"
					pName := ""
					if len(p.Names) > 0 {
						pName = p.Names[0].Name
					}
					params = append(params, ParamJSON{Name: pName, Type: pType})
				}
			}

			methods = append(methods, MethodJSON{
				Name:       methodName,
				Line:       methodLine,
				Params:     params,
				ReturnType: "error",
			})
		}

		interfaces = append(interfaces, InterfaceJSON{
			Name:    typeSpec.Name.Name,
			Line:    line,
			Methods: methods,
		})

		return true
	})

	res := ResultJSON{
		File:       filePath,
		Language:   "go",
		Interfaces: interfaces,
	}

	output, _ := json.MarshalIndent(res, "", "  ")
	fmt.Println(string(output))
}
