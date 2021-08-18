# E# Bytecode Standard<sup><sup><sub>`0.3.2-alpha`</sub></sup></sup>

## Type Modifiers
### Description
A type modifier is used to describe or modify types. For example, you may prefix the signature `B` with `U` resulting in the fully qualified signature `UB`: an unsigned byte.
### Table
Delimiter | Modifier | Description | Example
--------- | -------- | ----------- | -------
`U` | `unsigned` | Tells the VM to treat the type as an unsigned type. | `UI`
`R` | `reference` | A pointer to a location in memory. | `RTlang.type.Object;`
`E` | `data-type` | A type definition. This may be a struct or trait. | `Efoo.bar.ExampleTrait;`

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
`O` | `struct-object` | An instance of a struct. Structs hold data, and may inherit traits and implement methods. | `Olang.type.String;`
`T` | `trait-object` | An instance of a trait; also known as a "trait object". Traits cannot hold data, as they hold methods. structs may inherit traits, but traits may not inherit structs. | `RTfoo.bar.ExampleTrait;`
`O<class-id>;` | `struct-type-signature` | A fully qualified struct type signature. | `Olang.type.String;`
`T<class-id>;` | `trait-type-signature` | A fully qualified trait type signature. | `Tlang.type.Object;`

## Instructions
### Description
Opcodes that the VM will interpret at runtime.
### Limitations
There may only be up to 255 instructions. This is because the VM represents instructions in memory using a `u8`.
### Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`op` | `N/A` | An empty instruction that does nothing. | `0x00`
`add` | `i8/u8` *(flags)*, `i8/u8`, `i8/u8` | `N/A` | `0x01`
`sub` | `i8/u8` *(flags)*, `i8/u8`, `i8/u8` | `N/A` | `0x02`
`mul` | `i8/u8` *(flags)*, `i8/u8`, `i8/u8` | `N/A` | `0x03`
`div` | `i8/u8` *(flags)*, `i8/u8`, `i8/u8` | `N/A` | `0x04`
`add` | `i16/u16` *(flags)*, `i16/u16`, `i16/u16` | `N/A` | `0x05`
`sub` | `i16/u16` *(flags)*, `i16/u16`, `i16/u16` | `N/A` | `0x06`
`mul` | `i16/u16` *(flags)*, `i16/u16`, `i16/u16` | `N/A` | `0x07`
`div` | `i16/u16` *(flags)*, `i16/u16`, `i16/u16` | `N/A` | `0x08`
`add` | `i32/u32` *(flags)*, `i32/u32`, `i32/u32` | `N/A` | `0x09`
`sub` | `i32/u32` *(flags)*, `i32/u32`, `i32/u32` | `N/A` | `0x0A`
`mul` | `i32/u32` *(flags)*, `i32/u32`, `i32/u32` | `N/A` | `0x0B`
`div` | `i32/u32` *(flags)*, `i32/u32`, `i32/u32` | `N/A` | `0x0C`
`add` | `i64/u64` *(flags)*, `i64/u64`, `i64/u64` | `N/A` | `0x0D`
`sub` | `i64/u64` *(flags)*, `i64/u64`, `i64/u64` | `N/A` | `0x0E`
`mul` | `i64/u64` *(flags)*, `i64/u64`, `i64/u64` | `N/A` | `0x0F`
`div` | `i64/u64` *(flags)*, `i64/u64`, `i64/u64` | `N/A` | `0x10`
