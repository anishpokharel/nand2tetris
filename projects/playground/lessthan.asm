// This program is going to do the following things.
// It is going to pop value of y from the stack. Pop means stack counter goes -1
// It is going to pop the value of x from the stack. Same here.
// It is going to compare X < Y.
        // If true, returns -1. Meaning pushes this value on the stack.
        // If false, returns 0

	@SP
	M=M-1      // Decrease stack pointer to point to next available variable, in our case the y.
	A=M        // Select the Stack Pointer's memory location by loading up the A Register.
	D=M        // D holds our first operand, y.
	@SP
	M=M-1
	A=M
	A=M
	D=D-A      // y - x
	@POSITIVE
	D;JGT
	// If execution comes here, then the condition is false, push 0 to indicate such.
	@SP
	A=M
	M=0
	@SP
	M=M+1
	@END
	0;JMP
(POSITIVE)
	//Return -1 then goto end.
	@SP
	A=M
	M=-1       // Push -1 because X is greater than Y
        @SP
	M=M+1      // Move stack pointer to empty slot.
	@END
	0;JMP
(END)
	@SP

//@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nA=M\nD=D-A\n@POSITIVE_LT\nD;JGT\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@END_LT\n0;JMP\n(POSITIVE_LT)\n@SP\nA=M\nM=-1\n@SP\nM=M+1\n@END_LT\n0;JMP\n(END_LT)
