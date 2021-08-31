# E# Bytecode Standard<sup><sup><sub>`0.4`</sub></sup></sup>

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
`T` | `trait-object` | An instance of a trait; also known as a "trait object". Traits cannot hold data; they may only hold methods. Structs may inherit traits, but traits may not inherit structs. | `Tfoo.bar.ExampleTrait;`
`R<signature>` | `reference` | A pointer to a location in memory. | `RTlang.type.Object;`
`O<class-id>;` | `struct-type-signature` | A fully qualified struct type signature. | `Olang.type.String;`
`T<class-id>;` | `trait-type-signature` | A fully qualified trait type signature. | `Tlang.type.Object;`

## Instructions
### Description
Opcodes that the VM will interpret at runtime.
### Limitations
There may only be up to 255 instructions. This is because the VM represents every opcode with a `u8`.
### Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`nop` | `N/A` | An empty instruction that does nothing. | `0x00`
`add` | `i<n>`, `i<n>` | `(none)` | `0x01`
`sub` | `i<n>`, `i<n>` | `(none)` | `0x02`
`mul` | `i<n>`, `i<n>` | `(none)` | `0x03`
`div` | `i<n>`,`i<n>` | `(none)` | `0x04`
`uadd` | `u<n>`, `u<n>` | `(none)` | `0x05`
`usub` | `u<n>`, `u<n>` | `(none)` | `0x06`
`umul` | `u<n>`,`u<n>` | `(none)` | `0x07`
`udiv` | `u<n>`, `u<n>` | `(none)` | `0x08`
`push` | `u16` *(index)*, `RO` | `(none)` | `0x09`
`tpush` | `u16` *(index)*, `RT` | `(none)` | `0x0A`
`bpush` | `u16` *(index)*, `u8` *(flags)*, `i8/u8` | `(none)` | `0x0B`
`spush` | `u16` *(index)*, `u8` *(flags)*, `i16/u16` | `(none)` | `0x0C`
`ipush` | `u16` *(index)*, `u8` *(flags)*, `i32/u32` | `(none)` | `0x0D`
`lpush` | `u16` *(index)*, `u8` *(flags)*, `i64/u64` | `(none)` | `0x0E`
`fpush` | `u16` *(index)*, `f32` | `(none)` | `0x0F`
`dpush` | `u16` *(index)*, `f64` | `(none)` | `0x10`
`pop` | `u16` *(index)*, `RO` | `(none)` | `0x11`
`tpop` | `u16` *(index)*, `RT` | `(none)` | `0x12`
`bpop` | `u16` *(index)*, `u8` *(flags)*, `i8/u8` | `(none)` | `0x13`
`spop` | `u16` *(index)*, `u8` *(flags)*, `i16/u16` | `(none)` | `0x14`
`ipop` | `u16` *(index)*, `u8` *(flags)*, `i32/u32` | `(none)` | `0x15`
`lpop` | `u16` *(index)*, `u8` *(flags)*, `i64/u64` | `(none)` | `0x16`
`fpop` | `u16` *(index)*, `f32` | `(none)` | `0x17`
`dpop` | `u16` *(index)*, `f64` | `(none)` | `0x18`
