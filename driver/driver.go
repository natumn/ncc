package driver

import (
	"go/ast"

	"github.com/natumn/goruda/parser"
	// "go/token"
	"os"
	// "flag"
)

type Driver struct {
	Pkgs map[string]*ast.Package
	Opts *Options
}

type Options struct {
	Path string
}

func (d *Driver) ParseOptions() {
	d.Opts.Path = os.Args[0]
	// オプションあとで
	// f := flag.NewFlagSet(os.Args[0], flag.ExitOnError)
}

func (d *Driver) ParsePkgs(path string) (err error) {
	d.Pkgs, err = parser.ParsePkgs(path)
	if err != nil {
		return err
	}
	return nil
}

func (d *Driver) Codegen()
