# E# Bytecode Standard<sup><sup><sub>`0.7-alpha.2`</sub></sup></sup>

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
`type-flags` | Type Flags | A `u8` representing a [Type ID](#type-id) (first 4 bits) and a [Type Modifier](#type-modifier) (last 4 bits)
`fn-ref` | Function Reference | A `usize` referencing a function.
`fn-id` | Function Identifier | A UTF-8 string representing a function's identifier terminated by "`;`".
`imm<n>` | Immediate `<n>` | An immediate byte or string of bytes represented as a bytecode operand.
`"<end>"` | End | The end of a file, byte, function return type, etc. Any data after this is considered to be undefined.
***Note:** Parts of identifiers surrounded by "`"`" are to be interpreted literally.*
`any` | Any | An unsized primitive type that represents a type determined at runtime.

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
The function table holds function definitions. The last 4 bytes of the table are `DE AD C0 DE`. Each function in the table starts with `C0 DE` (2 bytes).
### Function
#### Identifier
***Note:** Function identifiers must be terminated by "`;`".*
###### Examples
*Method*
```
foo.Bar#bar;
```
*Function*
```
foo.baz;
```
#### Parameters
An array of `type-flags` terminated by `FF`.
###### Example
```
22 22 FF
```
#### Return Type
###### Example
```
F0
```
#### Code
###### Example
```
10 22 01    10 22 01    01 22    00
```

# Type ID
## Description
A `u4` representing a primitive type.
## Table
Type | Identifier | Operands
---- | ---------- | --------
`n8` | `0` | `N/A`
`n16` | `1` | `N/A`
`n32` | `2` | `N/A`
`n64` | `3` | `N/A`
`f32` | `8` | `N/A`
`f64` | `9` | `N/A`
`object` | `A` | `N/A`
`function` | `B` | `[u8]` *(`fn-ref`)*
`array` | `C` | `u8` *(`type-flags`)*
`void / ()` | `F` | `N/A`

# Type Modifier
## Description
A `u4` used to describe or modify types.<br>
***Note:** Using an undefined type modifier will result in undefined behavior.*
## Table
Flag Index | Modifier | Description
---------- | -------- | -----------
`0` | `data-type` | A type definition. This may be a struct or trait.
`1` | `unsigned` | An unsigned integer.

# Opcodes
## Description
Opcodes that denote the operands and behavior of an instruction.
## Limitations
There may only be up to 256 opcodes. This is because the VM represents every opcode with a `u8`.
## Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`nop` | `N/A` | Increments the instruction pointer. | `00`
`add` | `imm8` *(`type-flags`)* | Adds two numbers. | `01`
`sub` | `imm8` *(`type-flags`)* | Subtracts two numbers. | `02`
`mul` | `imm8` *(`type-flags`)* | Multiplies two numbers. | `03`
`div` | `imm8` *(`type-flags`)* | Divides two numbers. | `04`
`push` | `imm8` *(`type-flags`)*, `any` *(`value`)* | Push value onto stack. | `10`
`pop` | `N/A` | Pop first value off stack. | `11`
`popl` | `N/A` | Pop last value off stack. | `12`
`cast` | `imm8` *(`type-flags`)* *from*, `imm8` *(`type-flags`)* *to* | Casts a value from type `A` to type `B` | `14`
`call` | `usize` *(`fun-ref`)* | Calls a function. | `18`
`call` | `[imm8]` *(`fn-id`)* | Calls a function. | `19`
`ret` | `N/A` | Returns from a function. | `1A`
