# E# Bytecode Standard<sup><sup><sub>`0.8.0-alpha.3`</sub></sup></sup>

# Definitions
| Identifier      | Name                            | Description                                                                                                    |
|-----------------|---------------------------------|----------------------------------------------------------------------------------------------------------------|
| `class`         | Class                           | A struct or trait type.                                                                                        |
| `class-id`      | Class Identifier                | A UTF-8 string terminated by `;` representing a class.                                                         |
| `object`        | Object                          | An instance of a class.                                                                                        |
| `struct-object` | Struct Object                   | An instance of a struct.                                                                                       |
| `trait-object`  | Trait Object                    | An instance of a trait.                                                                                        |
| `type-id`       | [Type ID](#type-id)             | A `u4` representing a primitive type.                                                                          |
| `type-modifier` | [Type Modifier](#type-modifier) | A `u4` representing a [Type Modifier](#type-modifier).                                                         |
| `type-flags`    | Type Flags                      | A `u8` representing a [Type ID](#type-id) (lower 4 bits) and a [Type Modifier](#type-modifier) (higher 4 bits) |
| `fn-id`         | Function Identifier             | A UTF-8 string representing a function's identifier terminated by "`;`".                                       |
| `imm<n>`        | Immediate `<n>`                 | An immediate byte or string of bytes represented as a bytecode operand.                                        |
| `"<end>"`       | End                             | The end of a file, byte, function return type, etc. Any data after this is considered to be undefined.         |
| `any`           | Any                             | An unsized primitive type that represents a type determined at runtime.                                        |
| `local`         | Local Variable                  | An index of a local variable in the local variable stack.                                                      |
| `index`         | Constant Index                  | An index of the constant table.                                                                                |
***Note:** Parts of identifiers surrounded by "`"`" are to be interpreted literally.*

# Executable Structure
## Magic
`E5 00 C0 DE`
## Offsets
Each offset is a `u32` that describes the offset at which specific data is.
### Table
| Name           |
|----------------|
| Constant Table |
| Class Table    |
| Function Table |
| `<reserved>`   |
| `<reserved>`   |
| `<reserved>`   |
| `<reserved>`   |
| `<reserved>`   |
## Constant Table
### Description
The constant table holds constant values.
### Constant
| Name    | Type & Value   | Description                                 |
|---------|----------------|---------------------------------------------|
| Index   | `u16`          | The index at which the constant is defined. |
| Type    | `type-flags`   | The type of constant to define.             |
| Length  | `u32`          | The constant value's length.                |
| Value   | N/A            | The constant value.                         |
| `<end>` | `u16` `0xFFFF` | The end of the constant definition.         |
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
foo.#baz;
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
A `u4` representing a primitive type. The operands of these types go after any `type-flags`.
If there are any operands, they will come immediately after the `type-flags`.
## Table
| Type          | Identifier | Operands                    |
|---------------|------------|-----------------------------|
| `i8`          | `0`        | `N/A`                       |
| `i16`         | `1`        | `N/A`                       |
| `i32`         | `2`        | `N/A`                       |
| `i64`         | `3`        | `N/A`                       |
| `f32`         | `4`        | `N/A`                       |
| `f64`         | `5`        | `N/A`                       |
| `object`      | `6`        | `N/A`                       |
| `function`    | `7`        | `imm16` (`index`) [`fn-id`] |
| `array`       | `8`        | `imm8` (`type-flags`)       |
| `dyn`         | `9`        | `N/A`                       |
| `void` / `()` | `F`        | `N/A`                       |

# Type Modifier
## Description
A `u4` used to describe or modify types.<br>
***Note:** Using an undefined type modifier will result in undefined behavior.*
## Table
| Flag Index | Modifier    | Description                                       |
|------------|-------------|---------------------------------------------------|
| `0`        | `data-type` | A type definition. This may be a struct or trait. |
| `1`        | `unsigned`  | An unsigned integer.                              |

# Instructions
## Description
Instructions, their opcodes, their operands, and their function.
## Limitations
There may only be up to 256 opcodes. This is because the VM represents every opcode with a `u8`.
## Table
| Instruction | Operands                                                 | Stack            | Description                                                       | Opcode |
|-------------|----------------------------------------------------------|------------------|-------------------------------------------------------------------|--------|
| `nop`       | `N/A`                                                    |                  | Increments the instruction pointer.                               | `00`   |
| `add`       | `imm8` (`type-flags`)                                    | ← `i<n>`, `i<n>` | Adds two numbers.                                                 | `01`   |
|             |                                                          | → `i<n>`         |                                                                   |        |
| `sub`       | `imm8` (`type-flags`)                                    | ← `i<n>`, `i<n>` | Subtracts two numbers.                                            | `02`   |
|             |                                                          | → `i<n>`         |                                                                   |        |
| `mul`       | `imm8` (`type-flags`)                                    | ← `i<n>`, `i<n>` | Multiplies two numbers.                                           | `03`   |
|             |                                                          | → `i<n>`         |                                                                   |        |
| `div`       | `imm8` (`type-flags`)                                    | ← `i<n>`, `i<n>` | Divides two numbers.                                              | `04`   |
|             |                                                          | → `i<n>`         |                                                                   |        |
| `inc`       | `imm8` (`type-flags`)                                    | ↔ `i<n>`         | Increments a number.                                              | `05`   |
| `dec`       | `imm8` (`type-flags`)                                    | ↔ `i<n>`         | Decrements a number.                                              | `06`   |
| `push`      | `imm8` (`type-flags`), `imm8` (`local`)                  | ⇐ `any`          | Push local variable onto stack.                                   | `10`   |
|             |                                                          | → `any`          |                                                                   |        |
| `pop`       | `N/A`                                                    | ← `any`          | Store value in local variable stack.                              | `11`   |
|             |                                                          | ⇒ `any`          |                                                                   |        |
| `cast`      | `imm8` (`type-flags`) *from*, `imm8` (`type-flags`) *to* |                  | Casts a value from type `A` to type `B`.                          | `14`   |
| `call`      | `imm16` (`index`) [`fn-id`]                              |                  | Calls a function.                                                 | `18`   |
| `ret`       | `N/A`                                                    |                  | Returns from a function.                                          | `1A`   |
| `vret`      | `imm8` (`type-flags`)                                    |                  | Returns from a function, pushing a value onto the caller's stack. | `1B`   |
| `ldc`       | `imm16` (`index`)                                        |                  | Pushes a constant to the stack.                                   | `1C`   |
