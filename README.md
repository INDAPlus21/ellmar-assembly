# IiLA, Iila is a limited architecture

Emulated (via interpreter) assembly language/architecture.
There are 8 registers:
#0 & #1 are set at 1 and -1 at start.
The rest of the registers are given
random values. All registers hold value of 
signed 32 bit integer.

If the value in a register overflows it starts counting
from the opposite end of the i32 range.

There are no immediate values or assigning registers.
There is only addition. This also applies to stdin,
which functions as an add function.

Instruction | Description
-- | --
add | r1 = r1 + r2
j | jump lines specified reg (only counts lines containing code)
sk | skip 1 line if r1 == r2 (only counts lines containing code)
io | if reg1 == 0: print reg2, if reg1 == 1: add value from stdin to reg2

### Syntax
*instruction* *reg1* (*reg2*)

Indentation using tabs is not supported.

Comments start with */*