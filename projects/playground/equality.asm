	@SP
	M=M-1      //  Move Stack Pointer up to where our first operand is.
	A=M        //  Move SP address to Register A.
	D=M        //  Move the first operand to register D.
	@SP
	M=M-1
	A=M
	A=M
	D=D-A
	@EQ_RETURN
	D;JEQ      //  If Register D is Zero then both operands are equal return -1
	// Else return 0
	@SP
	A=M
	M=0
	@SP
	M=M+1
	@END
	0;JMP
	// IF Equal return -1
(EQ_RETURN)
	@SP
	A=M
	M=-1
	@SP
	M=M+1
	@END
	0;JMP
(END)
