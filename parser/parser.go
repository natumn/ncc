package parser

import (
	"os"
)

type AST struct {
}

func ParseFile(filepath string) (map[string]*AST, error) {
	f, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}

}
