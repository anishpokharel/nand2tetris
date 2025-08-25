// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

// Load R0's value into D register. 
// Load R1's value into A/M register. 

   
   @0             // Setting A with constant ZERO
   D=A            // Moving the ZERO to D register
   
   @R2            // Selecting RAM location R2
   M=D            // Storing the ZERO in R2, to clear previous results.  
   
   @R0            // Select the R0 Ram location. This sets the value of A=0
   D=M            // Setting the content of selected ram into D register. M=Ram[A]

   @firstNumber   // Setting up the variable first number
   M=D            // Storing the value of first number

   @R1            // Select the R1 Ram location. This sets the value of A=1
   D=M            // Setting the content of the selected ram into D register. 
   
   @secondNumber  // Second number variable's memory location.
   M=D            // Storing the value of second number into RAM[secondNumber]

(MULTIPLY)        // The loop to multiply R0 & R1

   @END           // Label
   D; JEQ         // If D is Zero Jump to the end. 
   
   @counter       // Counter
   D=D-1          // Decreasing the counter.
   M=D            // Saving the counter.

   @R2            // Selecting R2, this is where our result lives. 
   A=M            // Moving the content of R2 to A register
   D=A            // Now, moving it to D register. 

   @firstNumber   // Selecting first number
   A=M            // Moving the content of the ram to A register
   M=A            // Moving first number to M register for calculation on the next step.
   D=D+M          // Adding up 

   @R2            // Selecting R2 to store the value on next instruction.
   M=D            // Save the calculated sum to R2

   @counter       // Select counter to get the value of counter to D register. 
   D=M            // Setting the D to the value of counter from the memory.

   @MULTIPLY      // Label multiply, getting ready to jump.
   D; JGT         // Jump if D is greater than ZERO.

(END)
   @END
   0; JMP