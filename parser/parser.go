package parser

import (
	"github.com/natumn/lexer"
	"os"
)

type AST struct {
}

func ParseFile(filename string) (map[string]*AST, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	tokens, err := lexer.Tokenize(f)
	if err != nil {
		return nil, err
	}

}
