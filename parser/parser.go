package parser

import (
	"go/ast"
	"go/parser"
	"go/token"
)

func ParsePkgs(path string) (map[string]*ast.Package, error) {
	fset := token.NewFileSet()
	ast, err := parser.ParseDir(fset, path, nil, 0)
	if err != nil {
		return ast, err
	}
	return ast, err
}
