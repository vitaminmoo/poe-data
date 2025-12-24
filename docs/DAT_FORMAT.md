# Path of Exile DATC64 File Format

This document describes the `.datc64` file format used by Path of Exile (PoE) and Path of Exile 2 (PoE 2).

## File Structure

The file consists of a header, a fixed-width data section (Table Data), a separator, and a variable-width data section (Variable Data).

| Offset | Type   | Description                                                                           |
| :----- | :----- | :------------------------------------------------------------------------------------ |
| 0x00   | `u32`  | Number of rows in the table.                                                          |
| 0x04   | `[u8]` | **Table Data**. Fixed-width cells for each row. Total size = `NumRows * BytesPerRow`. |
| ...    | `u64`  | **Separator**. `0xBBBBBBBBBBBBBBBB` (8 bytes of `0xBB`).                              |
| ...    | `[u8]` | **Variable Data**. Heap for variable-length data (Strings, Arrays).                   |

## Data Types

All data in the Table Data section is fixed-width. Variable-length data is stored in the Variable Data section and referenced by offset.

| Type          | Size (Bytes) | Description                                                                              |
| :------------ | :----------- | :--------------------------------------------------------------------------------------- |
| `bool`        | 1            | Boolean value (0 or 1).                                                                  |
| `i16` / `u16` | 2            | 16-bit integer.                                                                          |
| `i32` / `u32` | 4            | 32-bit integer.                                                                          |
| `i64` / `u64` | 8            | 64-bit integer.                                                                          |
| `f32`         | 4            | 32-bit floating point.                                                                   |
| `f64`         | 8            | 64-bit floating point.                                                                   |
| `string`      | 8            | **Offset** (`u64`) into Variable Data. Points to a UTF-16 double-null-terminated string. |
| `array`       | 16           | **Count** (`u64`) followed by **Offset** (`u64`) into Variable Data.                     |
| `interval`    | 8            | Two `i32` values representing a range/interval.                                          |
| `row`         | 8            | Reference to a row in the current table by index (Local Reference). an index.            |
| `foreignrow`  | 16           | Reference to a row in another table by index. 128-bit value.                             |
| `enumrow`     | 4            | Index (`i32`) into a predefined enumeration.                                             |

### Strings

Strings are stored in the Variable Data section as UTF-16LE, terminated by two null bytes (`0x0000`).
The value in the Table Data is a `u64` offset relative to the start of the Variable Data section.
_Note: The separator `0xBB...` is considered part of the "offset space" in some interpretations, or at least offsets < 8 are invalid._

### Arrays

Arrays are stored in the Variable Data section as a contiguous sequence of values.
The value in the Table Data is a pair of `u64`: `{count, offset}`.

- `count`: Number of elements.
- `offset`: Byte offset into Variable Data where the array begins.

### References

- **Foreign Row**: A 16-byte value linking to a row in another table. The exact mechanism (index vs key) is 128-bit wide.
- **Local Row**: An 8-byte value linking to a row in the same table.

## Discrepancies & Notes

- **Legacy Formats**: Older `.dat` files (32-bit) used 4-byte offsets for strings and 8-byte `{count, offset}` (2x u32) for arrays. The `.datc64` format updates these to 64-bit widths.
- **Implementation Bugs**: The current `src/dat_parser.rs` implementation has incorrect sizes for `Interval` (listed as 16, should be 8) and `ForeignRow` (listed as 8, should be 16).
- **Variable Data Deduping**: String values in Variable Data are often deduplicated. If multiple rows reference the same string, they point to the same offset. Arrays might also share data if identical, though less common.
