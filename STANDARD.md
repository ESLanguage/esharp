# E# Bytecode Standard<sup><sup><sub>`0.6`</sub></sup></sup>

## Type Modifiers
### Description
A type modifier is used to describe or modify types. For example, you may prefix the signature `B` with `U` resulting in the fully qualified signature `UB`: an unsigned byte.
### Table
Delimiter | Flag Index | Modifier | Description | Example
--------- | ---------- | -------- | ----------- | -------
`U` | `0` | `unsigned` | Tells the VM to treat the type as an unsigned type. | `UI`
`E` | `1` | `data-type` | A type definition. This may be a struct or trait. | `Efoo.bar.ExampleTrait;`

## Definitions
Identifier | Name | Description | Example
---------- | ---- | ----------- | -------
`struct-object` | Struct Object | An instance of a struct. | `N/A`
`trait-object` | Trait Object | An instance of a trait. | `N/A`
`class` | Class | A "class" may refer to a struct or trait. | `N/A`
`class-id` | Class Identifier | A unique identifier representing a class. | `lang.type.Object`
`type-flags` | Type Flags | A `u8` representing a primitive type. See [Type Flags](#Type%20Flags) for more details. | `N/A` | `N/A`
`modifier-flags` | Type Modifier Flags | A `u8` representing a [Type Modifier](#Type%20Modifiers). | `N/A` | `N/A`

## Type Flags
### Description
A `u8` representing a primitive type. **Note: The last 4 bits are reserved.**
### Table
Type | Flag
---- | ----
`B` | `0x00`
`S` | `0x01`
`I` | `0x02`
`L` | `0x03`
`F` | `0x04`
`D` | `0x05`
`O` | `0x06`
`T` | `0x07`

## Signatures
Signature | Type | Description | Example
--------- | ---- | ----------- | -------
`B` | `i8`/`u8` | 8-bit integer/byte | `N/A`
`S` | `i16`/`u16` | 16-bit integer | `N/A`
`I` | `i32`/`u32` | 32-bit integer | `N/A`
`L` | `i64`/`u64` | 64-bit integer | `N/A`
`F` | `f32` | Single-precision floating-point integer | `N/A`
`D` | `f64` | Double-precision floating-point integer | `N/A`
`B` | `bool` | A boolean. | `N/A`
`O` | `struct-object` | An instance of a struct. Structs may hold data, implement methods, and inherit traits. | `Olang.type.String;`
`T` | `trait-object` | An instance of a trait; also known as a "trait object". Traits cannot hold data; they may only hold methods. Trait objects hold references to methods. Structs may inherit traits, but traits may not inherit structs. | `Tfoo.bar.ExampleTrait;`
`R<signature>` | `reference` | A pointer to a location in memory. | `RTlang.type.Object;`
`O<class-id>;` | `struct-type-signature` | A fully qualified struct type signature. | `Olang.type.String;`
`T<class-id>;` | `trait-type-signature` | A fully qualified trait type signature. | `Tlang.type.Object;`

## Instructions
### Description
Opcodes that the VM will interpret at runtime.
### Limitations
There may only be up to 256 instructions. This is because the VM represents every opcode with a `u8`.
### Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`nop` | `N/A` | An empty instruction that does nothing | `0x00`
`iadd` | `i<n>`, `i<n>` | `(none)` | `0x01`
`uadd` | `u<n>`, `u<n>` | `(none)` | `0x02`
`fadd` | `f<n>`, `f<n>` | `(none)` | `0x03`
`isub` | `i<n>`, `i<n>` | `(none)` | `0x04`
`usub` | `u<n>`, `u<n>` | `(none)` | `0x05`
`fsub` | `f<n>`, `f<n>` | `(none)` | `0x06`
`imul` | `i<n>`, `i<n>` | `(none)` | `0x07`
`umul` | `u<n>`,`u<n>` | `(none)` | `0x08`
`fmul` | `f<n>`,`f<n>` | `(none)` | `0x09`
`idiv` | `i<n>`,`i<n>` | `(none)` | `0x0A`
`udiv` | `u<n>`, `u<n>` | `(none)` | `0x0B`
`fdiv` | `f<n>`, `f<n>` | `(none)` | `0x0C`
`rpush` | `u8` *(`type-flags`)*, `u64` *(`reference`)* | Push reference onto stack | `0x0D`
`ipush` | `u8` *(`type-flags`)*, `i<n>` | Push signed integer onto stack | `0x0E`
`upush` | `u8` *(`type-flags`)*, `u<n>` | Push unsigned integer onto stack | `0x0F`
`fpush` | `u8` *(`type-flags`)*, `f<n>` | Push floating-point integer onto stack | `0x10`
`pop` | `(none)` | Pop first value off stack | `0x11`
`popl` | `(none)` | Pop last value off stack | `0x12`
`i2u` | `u8` *(`type-flags`)*, `i<n>` | Converts a signed integer to an unsigned integer | `0x13`
`u2i` | `u8` *(`type-flags`)*, `u<n>` | Converts an unsigned integer to a signed integer | `0x14`
`i2f` | `u8` *(`type-flags`)*, `i<n>` | Converts a signed integer to a floating-point integer | `0x15`
`f2i` | `u8` *(`type-flags`)*, `f<n>` | Converts a floating-point integer to a signed integer | `0x16`
`u2f` | `u8` *(`type-flags`)*, `u<n>` | Converts an unsigned integer to a floating-point ineger | `0x17`
`f2u` | `u8` *(`type-flags`)*, `f<n>` | Converts a floating-point integer to an unsigned integer | `0x18`
