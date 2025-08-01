// This file is part of playground while learning
// the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: playground/ThePlayground.asm


@8000         // Assuming there are 512 pixels and 256 rows to work with total
D=A           // Load the content of A register to D register for the next operation. 
@counter      // Declare a memory location @variable. 
M=D           // On the selected memory location, load the content of D register to memory. 

@SCREEN       // Load a register with Ram location of the first pixel of the screen. 
D=A           // Move the value of A register to D for the next operation. 
M=-1          // Set the ram's value to -1 to turn the pixel black. 

@PIXEL_LOC    // Declare another variable named test. Here variable refers to memory location. 
M=D           // Save the content of D register to memory. 


A=M           // Move the content of M register to A register. 
D=A           // Move the content of A register to D register. 

(Loop)
@counter      // Loading A register with the memory location @variable. 
D=M
D=D-1         // Decrement the value of D counting backwards. 
M=D           // On the selected memory location @variable, load content of D register (decremented value)


@PIXEL_LOC
D=M
D=D+1
A=D
M=-1

@PIXEL_LOC
M=D

@counter
D=M

@Loop
D; JGT 

@END
0; JMP