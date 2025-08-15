// POP the value from stack and move it to segment[base+index]
        @SP
        M=M-1        // Decrease the stack pointer for the pop operation.
	A=M
	D=M
	@7
	M=D
	@2
	D=A
	@1          // 1 is LCL. 
	AD=D+M
	@8
	M=D
	@7
	D=M
	@8
	A=M
	M=D
