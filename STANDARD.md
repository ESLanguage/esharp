# E# Bytecode Standard<sup><sup><sub>`0.2+alpha`</sub></sup></sup>

## Type Modifiers
### Description
A type modifier is used to describe or modify types. For example, you may prefix the signature `B` with `U` resulting in the fully qualified signature `UB`: an unsigned byte.
### Table
Delimiter | Modifier | Description | Example
--------- | -------- | ----------- | -------
`U` | `unsigned` | Tells the VM to treat the type as an unsigned type. | `UI`
`R` | `reference` | A location in memory that corresponds to data. | `RKtype.Object;`

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
`K` | `object` | An instance of a class. | `Ktype.String;`
`?` | `dyn` | A "dynamic" type that will be determined on runtime. | `N/A`
`K<class-path>;` | `type-signature` | A fully qualified type signature.

## Instructions
### Description
Instructions the VM will interpret at runtime.
### Limitations
There may only be up to 255 instructions. This is because the VM represents instructions in memory using a `u8`.
### Table
Instruction | Operands | Description | Opcode
----------- | -------- | ----------- | ------
`nop` | `N/A` | An empty instruction that does nothing. | `0x00`
`add` | `u8` *(flags)*, `number`, `number` | `N/A` | `0x01`
`sub` | `u8` *(flags)*, `number`, `number` | `N/A` | `0x02`
`mul` | `u8` *(flags)*, `number`, `number` | `N/A` | `0x03`
`div` | `u8` *(flags)*, `number`, `number` | `N/A` | `0x04`
`goto` | `u64` | Set the program counter to the specified address | `0x05`
