# E# Bytecode Standard<sup><sup><sub>`0.7-alpha.1`</sub></sup></sup>

# Definitions
Identifier | Name | Description
---------- | ---- | -----------
`class` | Class | A struct or trait type.
`class-id` | Class Identifier | A UTF-8 string terminated by `;` representing a class.
`object` | Object | An instance of a class.
`struct-object` | Struct Object | An instance of a struct.
`trait-object` | Trait Object | An instance of a trait.
`type-id` | [Type ID](#type-id) | A `u4` representing a primitive type.
`type-modifier` | [Type Modifier](#type-modifier) | A `u4` representing a [Type Modifier](#type-modifier).
`type-flags` | Type Flags | A `u8` representing a [Type Modifier](#type-modifier) (first 4 bits) and a [Type ID](#type-id) (last 4 bits)
`fn-ref` | Function Reference | A UTF-8 string referencing a function.

# File Structure
## Magic
`E5 00 C0 DE`
## Class Table
### Description
The class table holds class defenitions. The last 4 bytes of the table are `DE AD CA FE`.
### Format
#### Identifier
###### Description
The first bytes in the class definition are the identifier. A fully qualified identifier consists of a string of UTF-8 characters starting with the package identifier seperated by a `.`, terminated by a `;`, where the bytes, starting from the last byte after the `;` and ending before the first `.`, are the class name, and the rest of the bytes, seperated by a `.`, are the package qualifiers.
###### Example
```
foo.Bar;
```
#### TODO
## Function Table
### Description
The function table holds function definitions. The last 4 bytes of the table are `DE AD C0 DE`.
### Format
#### Identifier
###### Example
```
foo.Bar#bar
```
#### Parameters
###### Example
```
A0 B6 F
```
#### Return Type
###### Example
```
F
```
#### TODO

# Type ID
## Description
A `u4` representing a primitive type.
## Table
Type | Identifier | Operands
---- | ---------- | --------
`u8` | `0` | `(none)`
`u16` | `1` | `(none)`
`u32` | `2` | `(none)`
`u64` | `3` | `(none)`
`i8` | `4` | `(none)`
`i16` | `5` | `(none)`
`i32` | `6` | `(none)`
`i64` | `7` | `(none)`
`f32` | `8` | `(none)`
`f64` | `9` | `(none)`
`object` | `A` | `(none)`
`function` | `B` | `[u8]` *(`fn-ref`)*
`array` | `C` | `[u8]` *(`type-id`)*
`void / ()` | `F` | `(none)`

# Type Modifier
## Description
A `u4` used to describe or modify types. **Note: Using an undefined type modifier will result in undefined behavior.**
## Table
Flag Index | Modifier | Description
---------- | -------- | -----------
`0` | `data-type` | A type definition. This may be a struct or trait.

# Opcodes
## Description
Opcodes that denote the operands and behavior of an instruction.
## Limitations
There may only be up to 256 instructions. This is because the VM represents every opcode with a `u8`.
## Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`nop` | `N/A` | An empty instruction that does nothing | `00`
`addb` | `i8`, `i8` | `(none)` | `01`
`adds` | `i16`, `i16` | `(none)` | `02`
`addi` | `i32`, `i32` | `(none)` | `03`
`addl` | `i64`, `i64` | `(none)` | `04`
`uaddb` | `u8`, `u8` | `(none)` | `05`
`uadds` | `u16`, `u16` | `(none)` | `06`
`uaddi` | `u32`, `u32` | `(none)` | `07`
`uaddl` | `u64`, `u64` | `(none)` | `08`
`addf` | `f32`, `f32` | `(none)` | `09`
`addd` | `f64`, `f64` | `(none)` | `0A`
`subb` | `i8`, `i8` | `(none)` | `0B`
`subs` | `i16`, `i16` | `(none)` | `0C`
`subi` | `i32`, `i32` | `(none)` | `0D`
`subl` | `i64`, `i64` | `(none)` | `0E`
`usubb` | `u8`, `u8` | `(none)` | `0F`
`usubs` | `u16`, `u16` | `(none)` | `10`
`usubi` | `u32`, `u32` | `(none)` | `11`
`usubl` | `u64`, `u64` | `(none)` | `12`
`subf` | `f32`, `f32` | `(none)` | `13`
`subd` | `f64`, `f64` | `(none)` | `14`
`mulb` | `i8`, `i8` | `(none)` | `15`
`muls` | `i16`, `i16` | `(none)` | `16`
`muli` | `i32`, `i32` | `(none)` | `17`
`mull` | `i64`, `i64` | `(none)` | `18`
`umulb` | `u8`,`u8` | `(none)` | `19`
`umuls` | `u16`,`u16` | `(none)` | `1A`
`umuli` | `u32`,`u32` | `(none)` | `1B`
`umull` | `u64`,`u64` | `(none)` | `1C`
`umulf` | `f32`,`f32` | `(none)` | `1D`
`umuld` | `f64`,`f64` | `(none)` | `1E`
`divb` | `i8`,`i8` | `(none)` | `1F`
`divs` | `i16`,`i16` | `(none)` | `20`
`divi` | `i32`,`i32` | `(none)` | `21`
`divl` | `i64`,`i64` | `(none)` | `22`
`udivb` | `u8`, `u8` | `(none)` | `23`
`udivs` | `u16`, `u16` | `(none)` | `24`
`udivi` | `u32`, `u32` | `(none)` | `25`
`udivl` | `u64`, `u64` | `(none)` | `26`
`divf` | `f32`, `f32` | `(none)` | `27`
`divd` | `f64`, `f64` | `(none)` | `28`
`push` | `u8` *(`type-id`)*, `u<n>` | Push value onto stack | `2A`
`pop` | `(none)` | Pop first value off stack | `2B`
`popl` | `(none)` | Pop last value off stack | `2C`
`i2u` | `u8` *(`type-id`)*, `i<n>` | Converts a signed integer to an unsigned integer | `2D`
`u2i` | `u8` *(`type-id`)*, `u<n>` | Converts an unsigned integer to a signed integer | `2E`
`i2f` | `u8` *(`type-id`)*, `i<n>` | Converts a signed integer to a floating-point integer | `2F`
`f2i` | `u8` *(`type-id`)*, `f<n>` | Converts a floating-point integer to a signed integer | `30`
`u2f` | `u8` *(`type-id`)*, `u<n>` | Converts an unsigned integer to a floating-point ineger | `31`
`f2u` | `u8` *(`type-id`)*, `f<n>` | Converts a floating-point integer to an unsigned integer | `32`
`call` | `u64` *(`fun-ref`)* | Calls a function | `33`
`ret` | `(none)` | Returns from a function | `34`
