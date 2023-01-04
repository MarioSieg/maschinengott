# Maschinengott
The machine god (German: Maschinengott) is a very fast terminal disassembler for x86-64.
It uses all available CPU cores to disassemble huge binaries in a very short amount of time.
It also shows some information about the machine code - for example the most used assembly instructions
and the used ISA extensions.

## Why another disassembler?
I created my own little disassembler because the other ones were just too slow when disassembling huge
(300 MB+) binaries. I also wanted some quick information about used ISA extensions and instructions.

## Current features:
 * Works with Windows and Linux binaries
 * PE multithreaded disassembly for executable and dynamic link libraries (.exe and .dll)
 * View most used assembly instructions
 * View total assembly instruction count
 * View used ISA extensions (e. g. AVX, FMA3 or AVX512F)
 * View disassembled code in AT&T or Intel syntax
 * View instruction encoding

## Examples
Example output from a program analysis:<br>
![Example output from a program analysis](https://user-images.githubusercontent.com/49988901/178153948-0068ee20-a192-4e6b-a052-0334fb5dd22e.png)

Example disassembly in AT&T syntax:<br>
The relative address, machine language and instruction encoding is displayed too.<br>

![image](https://user-images.githubusercontent.com/49988901/178154008-b7409a45-c8b8-4ade-933d-6ee34147001e.png)

