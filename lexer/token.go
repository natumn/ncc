package lexer

type TokenType int

const (
	_ TokenType = iota

	_EOF // EOF

	// name and literal
	_Name    // name
	_Literal // literal

	// operators
	_Op     // op
	_Assign // =

	// delimiters
	_Lparen // (
	_Rparen // )
	_Lbreak // [
	_Rbreak // ]
	_Lbrace // {
	_Rbrace // }
	_Comma  // ,
	_Semi   // ;
	_Colon  // :
	_Dot    // .

	// keywords
	_If     // if
	_Else   // else
	_For    // for
	_Func   // function
	_Return // return
	_Data   // type
)
