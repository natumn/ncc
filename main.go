package main

import (
	"github.com/natumn/goruda/driver"
)

type Compiler struct {
	version string
	arch    *Arch
}

type Arch struct{}

func main() {
	compiler := Compiler{}
	compiler.Init()
	compiler.Run()
}

func (c *Compiler) Init() {
	c.version = "0.0.1"
	c.arch = &Arch{}

}

func (c *Compiler) Run() {
	driver := driver.Driver{}
	driver.ParseOptions()
	pkgAst, err := driver.ParsePkgs()
	if err != nil {

	}
	evmByteCode, err := driver.Codegen(pkgAst)
	if err != nil {

	}
}
