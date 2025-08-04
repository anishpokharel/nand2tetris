// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/2/ALU-basic.tst

// Tests the basic version of the ALU chip.
// DOES NOT replace the final test provided by ALU.tst.
// Specifically: Tests the ALU logic that computes the 'out' output;
// The 'zr' and 'ng' output bits are ignored.

load ALU.hdl,

set y %B0000000000000011,
set x %B0110111110111111,

set zx 0,
set nx 0,
set zy 1,
set ny 1,
set f  1,
set no 0,
eval;
