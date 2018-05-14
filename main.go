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
	// Phase2: type check and type inference.
	d.TypeCheck(pt)
	// Phase3: K Normal

	// Phase4: alpha conversion

	// Phase5:

	// Phase : assigned IR code generate.
	ir, err := d.Codegen(pt)
	if err != nil {
		panic(err)
	}
}
