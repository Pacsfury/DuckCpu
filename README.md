# DuckCpu

A basic and lightweight CPU emulator written in Rust that implements the P16b architecture.

**Work in progress**

---

## Why DuckCpu

DuckCpu's name comes from the Debugger Rubber Duck, popularized by the book The Pragmatic Programmer.

The duck debugging is telling to the duck what every line of code does, so we realize where's the bug easily.

The duck's message is that we all do stupid errors, and has been a symbol for programmers aroud the world.

## Technical Specifications (Specs)

* **ISA**: P16b (custom architecture)
* **RAM**: 4096 bytes of internal memory
* **Registers**: 4 registers of `i32` width
* **Ports**: 256 virtual I/O ports (`i32`)

---

## Instruction Format

Each instruction line is a 16-character binary string structured as follows:

| Bits | Length | Description |
| :--- | :--- | :--- |
| `0-7` | 8 bits | **Opcode**: The operation to execute |
| `8-11` | 4 bits | **Operand 1**: Destination register or port |
| `12-15` | 4 bits | **Operand 2**: Source register, port, or value |

---

## Implemented Operations

| Opcode (Binary) | Opcode (Decimal) | Name | Description |
| :--- | :--- | :--- | :--- |
| `00000001` | `1` | **SUM** | Adds the source reg. to the destination reg. |
| `00000010` | `2` | **SUB** | Subtracts the source reg. from the destination reg. |
| `00000011` | `3` | **MUL** | Multiplies the destination reg. by the source reg. |
| `00000100` | `4` | **DIV** | Floored integer division of destination by source |
| `00000101` | `5` | **NAND**| Bitwise NAND logical operation |
| `00000110` | `6` | **OR**   | Bitwise OR logical operation |
| `00000111` | `7` | **XOR**  | Bitwise XOR logical operation |
| `00001000` | `8` | **NOT**  | Inverts the bits of the destination reg. |
| `00001001` | `9` | **AND**  | Bitwise AND logical operation |
| `00001010` | `10`| **NOR**  | Bitwise NOR logical operation |
| `00001011` | `11`| **MOV**  | Copies a 4-bit value to the destination reg. |
| `00001100` | `12`| **PSN**  | *Port Send*: Sends the value of a reg. to a port |
| `00001101` | `13`| **PRD**  | *Port Read*: Reads the value of a port into a reg. |

---

## Coming Soon

* [ ] Function recording and calling
* [ ] Direct modification of RAM memory
* [ ] Support for function arguments
* [ ] Creation of simple test programs
* [ ] Design of a custom Assembler

---

