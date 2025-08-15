// Consider the following vm command.
	// push local val

	// push the LOCAL + val's content into the stack.

	@12
	D=A
	// @LOCAL is RAM[1]
	@1
	A=D+M
	D=M
	@SP
	A=M
	M=D
	@SP
	M=M+1
