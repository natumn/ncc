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
	d := driver.Driver{}
	d.Init()
	d.ParseOptions()

	// Phase1: transfrom source code into parse tree.
	pt, err := d.ParseFiles(filenames)
	if err != nil {
		panic(err)
	}
	// Phase2: type check.
	d.TypeCheck(pt)
	// Phase : assigned IR code generate.
	ir, err := d.Codegen(pt)
	if err != nil {
		panic(err)
	}
}
