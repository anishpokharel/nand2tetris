// This file is part of playground while learning
// the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: playground/Count.asm

// Runs an infinite loop that counts
    
    @100
    D=A
    @200        // use RAM[200] for value
    M=D         // value = 100

    @0
    D=A
    @201        // use RAM[201] for addr
    M=D         // addr = 0

(LOOP)
    @200
    D=M         // D = value

    @201
    A=M         // A = addr
    M=D         // RAM[addr] = value

    @201
    M=M+1       // addr++

    @200
    M=M-1       // value--

    @200
    D=M
    @END
    D;JLT       // If value < 0, end

    @LOOP
    0;JMP       // Repeat

(END)
    @END
    0;JMP       // Halt
