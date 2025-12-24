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
| `row`         | 8            | Reference to a row in the current table (Local Reference). Likely an index.              |
| `foreignrow`  | 16           | Reference to a row in another table. 128-bit value (Key/GUID?).                          |
| `enumrow`     | 4            | Index (`i32`) into a predefined enumeration.                                             |

### Arrays

Arrays are a "meta-type" or container layer. An array field in the table data does **not** contain the data itself, but a reference to it.

- **Structure**: `[Count: u64, Offset: u64]` (16 bytes).
- **Data Location**: The `Offset` points to the start of the array's data in the Variable Data section.
- **Element Size**: The size of the data at the offset is `Count * SizeOf(ElementType)`.
  - **Crucial Note**: The `datc64` format does _not_ encode the element type within the array reference. You **must** know the schema (the type of elements the array holds) to correctly read the data.
  - Example: An array of `i32` with count 10 will occupy 40 bytes at the specified offset.
  - Example: An array of `string` with count 5 will occupy 40 bytes at the specified offset (5 * 8 bytes per string reference), where each of those 8 bytes is *another\* offset to the actual string text.

### Higher-Level Types

These types are semantically distinct but are implemented using the basic types above.

| High-Level Type | Underlying Type | Description                                                                                                                                            |
| :-------------- | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Interval**    | `i32` (x2)      | Represents a numeric range. Stored as two consecutive 32-bit integers (Min, Max). Often technically valid to treat as `i64` but semantically distinct. |
| **File**        | `string`        | A string that is a path to a specific game asset file (e.g., `.dds`, `.ao`, `.sm`).                                                                    |
| **Directory**   | `string`        | A string representing a directory path, often used as a prefix for multiple files.                                                                     |
| **Color**       | `string`        | Typically a string representation of a color (e.g., hex code). Needs verification for specific tables.                                                 |
| **Point**       | `i32` (x2)?     | Likely two consecutive `i32`s representing X and Y coordinates.                                                                                        |
| **DateTime**    | `u64`?          | A timestamp. Format (Unix epoch, Windows ticks, etc.) needs verification.                                                                              |
| **Hash16**      | `u16`           | A 16-bit hash value.                                                                                                                                   |
| **Hash32**      | `u32`           | A 32-bit hash value.                                                                                                                                   |

### Strings

Strings are stored in the Variable Data section as UTF-16LE, terminated by two null bytes (`0x0000`).
The value in the Table Data is a `u64` offset relative to the start of the Variable Data section.

- **Deduplication**: If the same string value is used multiple times in a table (across different rows or columns), the generator usually writes it once and points all references to that single copy.
- **Magic Bytes**: The separator `0xBB...` (8 bytes) is considered part of the "offset space" at the start of the Variable Data. Offsets lower than 8 are invalid as they would point into these magic bytes.

### Validation & Completeness

Because Variable Data is typically written sequentially during table generation:

1.  **Ordering**: Referenced values appear in the Variable Data section in the order their corresponding cells appear in the table (column by column, then row by row).
2.  **Exception (Shared Strings)**: Due to deduplication, a string might be referenced "out of order" relative to the sequential flow if it was already written earlier.
3.  **No Gaps**: In a "perfect" table, every byte of the Variable Data section (after the initial 8 magic bytes) should be accounted for by at least one reference (either a string, an array, or a reference inside an array).
4.  **Zero-Length Arrays**: Arrays with a count of 0 still have a valid (and increasing) offset, but they point to zero bytes. Multiple adjacent empty arrays might point to the same offset.

### References

- **Foreign Row**: A 16-byte value linking to a row in another table. The exact mechanism (index vs key) is 128-bit wide.
- **Local Row**: An 8-byte value linking to a row in the same table.

## Discrepancies & Notes

- **Legacy Formats**: Older `.dat` files (32-bit) used 4-byte offsets for strings and 8-byte `{count, offset}` (2x u32) for arrays. The `.datc64` format updates these to 64-bit widths.
