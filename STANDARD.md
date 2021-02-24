# E# Bytecode Standard<sup><sup><sub>`0.1`</sub></sup></sup>

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
`nop` | `N/A` | An empty instruction that does nothing. | `0`
`badd` | `i8`, `i8` | `N/A` | `1`
`sadd` | `i16`, `i16` | `N/A` | `2`
`iadd` | `i32`, `i32` | `N/A` | `3`
`ladd` | `i64`, `i64` | `N/A` | `4`
`ubadd` | `u8`, `u8` | `N/A` | `5`
`usadd` | `u16`, `u16` | `N/A` | `6`
`uiadd` | `u32`, `u32` | `N/A` | `7`
`uladd` | `u64`, `u64` | `N/A` | `8`
`bsub` | `i8`, `i8` | `N/A` | `9`
`ssub` | `i16`, `i16` | `N/A` | `A`
`isub` | `i32`, `i32` | `N/A` | `B`
`lsub` | `i64`, `i64` | `N/A` | `C`
`ubsub` | `u8`, `u8` | `N/A` | `D`
`ussub` | `u16`, `u16` | `N/A` | `E`
`uisub` | `u32`, `u32` | `N/A` | `F`
`ulsub` | `u64`, `u64` | `N/A` | `10`
`bmul` | `i8`, `i8` | `N/A` | `11`
`smul` | `i16`, `i16` | `N/A` | `12`
`imul`| `i32`, `i32` | `N/A` | `13`
`lmul` | `i64`, `i64` | `N/A` | `14`
`ubmul` | `u8`, `u8` | `N/A` | `15`
`usmul` | `u16`, `u16` | `N/A` | `16`
`uimul` | `u32`, `u32` | `N/A` | `17`
`ulmul` | `u64`, `u64` | `N/A` | `18`
`bdiv` | `i8`, `i8` | `N/A` | `19`
`sdiv` | `i16`, `i16` | `N/A` | `1A`
`idiv` | `i32`, `i32` | `N/A` | `1B`
`ldiv` | `i64`, `i64` | `N/A` | `1C`
`ubdiv` | `u8`, `u8` | `N/A` | `1D`
`usdiv` | `u16`, `u16` | `N/A` | `1E`
`uidiv` | `u32`, `u32` | `N/A` | `1F`
`uldiv` | `u64`, `u64` | `N/A` | `20`
