package lexer

import (
	"io"
)

type File struct {
	source io.Reader
	tok TokenType

}

func 

func (f *File) Tokenize() ([]*string, error) {
	for f.tok != _EOF {
		switch f.tok {
			
		}
	}
}


func (f *File) next() {

}
