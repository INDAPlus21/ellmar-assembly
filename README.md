# IiLA, Iila is a limited architecture

Emulated (interpreter) assembly language/architecture.
There are 8 registers:
#0 & #1 are set at 1 and -1 respectively at start, but can be manipulated by the program.
The rest of the registers are given
random values (now an option). All registers hold value of
signed 32 bit integer.

If the value in a register overflows it starts counting
from the opposite end of the i32 range.

There are no immediate values or assigning registers.
There is only addition. This also applies to stdin,
which functions as an add function.

Instruction | Description
-- | --
add | r1 = r1 + r2
j | jump lines specified in reg (only counts lines containing code)
sk | skip 1 line if r1 == r2 (only counts lines containing code)
io | if reg1 == 0: print reg2, if reg1 == 1: add value from stdin to reg2

## KOMPLETTERING
To make examples run faster, random initialized registers are now a command line
option. If ran without the option, the random initialized registers
are instead set to -3 (because that was the best value for the examples).

## Syntax
*instruction* *reg1* (*reg2*)

Indentation using tabs is not supported.

Comments start with */*

## TODO (will probably not be done)
- [ ] Dont allow jump adresses that are more than 6 bit (signed)
- [ ] Interpreter error checking stuff instead of panicking
- [x] Making it run atleast a factorial calculator

## Usage
cargo run *file_path*

## Examples
### factorial.li
input a number + enter.

The program will output the factorial.

Test with smaller number first, as computing larger ones will take a long time.
Running with optimzations is recommended for larger numbers.

With optimizations I have computed 11! within seconds, and 12! in less
than 2 minutes (I think).

## mul.li
input number x + enter.

input number y + enter.

The program will output x*y.

# Options
`cargo run *file_path* --random`, will initialize the registers with random
values, except #0 and #1.

It is extremely recommended to compile with optimzation if you run the examples with the --random option.