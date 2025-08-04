// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

(INFINITE)        // Label named INFINITE
   //  Read content of RAM location. 
   @24576         // RAM Location 24576. This is where keyinputs are registered. 
   D=M            // D = value stored at RAM[24576]
   @PRESSED
   D;JGT          // IF D > 0, jump to PRESSED

   @NOT_PRESSED   // Jump to Not Pressed if key is not pressed.
   D;JEQ

   @INFINITE
   0;JMP          // JMP to INFINITE forever.
(PRESSED)

   @SCREEN       // Load a register with Ram location of the first pixel of the screen. 
   D=A           // Move the value of A register to D for the next operation. 
   M=-1          // Set the ram's value to -1 to turn the pixel black. 

   @PIXEL_LOC    // Declare another variable named test. Here variable refers to memory location. 
   M=D           // Save the content of D register to memory. 
   
   @8192         // There are 8192 pixels to control, setting D to 8192
   D=A

(INNER_LOOP)
   @counter      
   D=D-1
   M=D
   
   @24576         // RAM Location 24576. This is where keyinputs are registered. 
   D=M            // D = value stored at RAM[24576]
   @NOT_PRESSED
   D;JEQ

   @PIXEL_LOC
   D=M
   D=D+1
   A=D
   M=-1

   @PIXEL_LOC
   M=D

   @counter
   D=M
  
   @INNER_LOOP
   D; JGT 

   @INFINITE
   0; JMP
   

(NOT_PRESSED)
   // Keyboard is released or not pressed
   @SCREEN       // Load a register with Ram location of the first pixel of the screen. 
   D=A           // Move the value of A register to D for the next operation. 
   M=0          // Set the ram's value to -1 to turn the pixel black. 

   @PIXEL_LOC    // Declare another variable named test. Here variable refers to memory location. 
   M=D           // Save the content of D register to memory. 
   
   @8192
   D=A
   
   @counter2
   M=D
   D=M

   
(OTHER_LOOP)
   @SCREEN
   D=A
   M=0

   @counter2
   D=M
   D=D-1
   M=D

   @PIXEL_LOC
   D=M
   D=D+1
   A=D
   M=0

   @PIXEL_LOC
   M=D
   
   @24576         // RAM Location 24576. This is where keyinputs are registered. 
   D=M            // D = value stored at RAM[24576]
   @PRESSED
   D;JGT          // If key is pressed, jump back to filling the screen. 
   
   @counter2      
   D=M            // Move counter2's memory value into D register. 

   @OTHER_LOOP    // While counter2 is less greater than 0, loop.
   D; JGT

   @INFINITE      // Jump back to infinite loop.
   0; JMP
