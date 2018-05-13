package driver

import (
	"go/ast"

	"github.com/natumn/goruda/lexer"
	"github.com/natumn/goruda/parser"
	// "go/token"
	"os"
	// "flag"
)

type Driver struct {
	Options
	Lexer         lexer.Lexer
	Parser        parser.Parser
	PT            parser.ParseTree
	CodeGenerator codegen.CodeGenerator
}

type Options struct {
	Path string
}

func (d *Driver) ParseOptions() {
	d.Opts.Path = os.Args[0]
	// オプションあとで
	// f := flag.NewFlagSet(os.Args[0], flag.ExitOnError)
}

func (d *Driver) ParseFiles(filepaths []*string) (err error) {
	for _, filepath := range filepaths {
		parser.ParseFile(filepath)
		if err != nil {
			return err
		}
	}
	return nil
}

func (d *Driver) Codegen() {

}
