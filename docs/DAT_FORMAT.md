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

1.  **Ordering**: Referenced values generally appear in the Variable Data section in the order their corresponding cells appear in the table (column by column, then row by row).
2.  **No Gaps**: In a "perfect" table, every byte of the Variable Data section (after the initial 8 magic bytes) should be accounted for by at least one reference (either a string, an array, or a reference inside an array).
3.  **Exception (Shared Strings)**: Due to deduplication, a string might be referenced "out of order" relative to the sequential flow if it was already written earlier.
4.  **Zero-Length Arrays**: Arrays with a count of 0 still have a valid (and increasing) offset, but they point to zero bytes. Multiple adjacent empty arrays might point to the same offset.

## Heuristics, Validation & Type Detection

Identifying column types and validating data in undocumented tables relies on statistical analysis and structural constraints.

### Basic Type Constraints

- **Boolean**: Must strictly be `0` or `1`. Any other byte value invalidates this type.
- **Hashes (u16/u32)**: High entropy. Values should be distributed across the entire range of the integer type (unlike counts or indices which cluster near zero). MurmurHash is commonly used.
- **Floats**: `f32`/`f64` columns usually contain values with fractional parts. If a column is valid as a float but all values are integers (e.g., `1.0`, `50.0`), it is more likely an integer type unless the context strongly suggests a rate or multiplier.
- **EnumRow**: Values are highly concentrated at the lower end of the `i32` range (e.g., 0 to 100). Large values are extremely unlikely.

### Variable Width Data (Arrays & Strings)

- **Valid Offsets**:
  - Must be `>= 8` (after the magic bytes).
  - Must be `< TotalVariableDataSize`.
  - **Strings**: Offset must be at least 2 bytes before the end of the section (for `0x0000` terminator).
  - **Arrays**: Offset + (Count \* ElementSize) must be `<= TotalVariableDataSize`.
- **Array Patterns**:
  - **Count Limit**: Counts are rarely high. Values `> 30` are suspicious for generic data arrays (though technically possible).
  - **Double Spike Histogram**: An array column (16 bytes) often shows two value distributions:
    - **Count (first 8 bytes)**: Clustered very low (0-30).
    - **Offset (next 8 bytes)**: Monotonically increasing (mostly) and spread across the variable data range.
- **String/Array Differentiation**:
  - If a `u64` column's values point to valid UTF-16 double-null terminated sequences, it's likely a `String`.
  - If a `u128` (16-byte) column's second `u64` points to data, check the first `u64` (count). If the data at the offset looks like a sequence of `n` items, it's an array.

### References

- **Values**:
  - **Null**: `0xFE` repeated (8 or 16 bytes). A non-reference column having this specific pattern is rare.
  - **Range**: Valid references must be `< MaxRows` of the target table.
- **Target Inference**:
  - A column is only a valid reference to Table X if _all_ non-null values in that column are valid indices in Table X.
  - **Correlation**: If a column is suspected to reference Table X, correlate the text data. Do the strings in the source column (or the source table's name) semantically relate to the string columns in Table X? (e.g., `MonsterId` column pointing to `Monsters.dat` which has a `Name` column like "Zombie").

### Structural & Contextual Analysis

- **Neighbor Inference**: A single byte column strictly containing `0` or `1`, sandwiched between two clearly identified string columns, is almost certainly a boolean. This is statistically stronger than an isolated byte column being 0/1 (which could be padding, small integers, etc.).
- **Gap Analysis**:
  - Map all known references (strings, arrays) to the Variable Data section.
  - Identify "dead zones" (unreferenced bytes).
  - Analyze the dead zones: Do they look like arrays of integers? Text?
  - Find columns in the fixed data whose values (or derived offsets) point exactly to the start of these gaps. This can reveal previously unidentified array columns.
- **Files & Directories**:
  - Treat as `String` first.
  - Validate if the string value corresponds to a real file path inside the game's GGPK/Bundle system.
  - Directories act as prefixes for other file columns.

### Byte Histograms (Little Endian)

- **Integers/Indices**: Heavy weighting towards low values (0x00, 0x01...).
- **Offsets**: Even distribution or gradual shifting across the file's size range.
- **Entropy**:
  - **Low**: Booleans, small enums, sparse arrays.
  - **High**: Hashes, compressed data (unlikely here), encryption keys (unlikely here).

### References

- **Foreign Row**: A 16-byte value linking to a row in another table. The exact mechanism (index vs key) is 128-bit wide.
- **Local Row**: An 8-byte value linking to a row in the same table.
- **Null Values**: If a reference (Local or Foreign) is null, the entire cell is filled with `0xFE` bytes (e.g., 8 bytes of `0xFE` for Local Row, 16 bytes for Foreign Row). This is distinct from a value of 0, which may be a valid index.

## Discrepancies & Notes

- **Legacy Formats**: Older `.dat` files (32-bit) used 4-byte offsets for strings and 8-byte `{count, offset}` (2x u32) for arrays. The `.datc64` format updates these to 64-bit widths.
