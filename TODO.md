# PoE Data Project - Task Status

## Completed
- [x] Added `File`, `Directory`, `Color`, `Hash16`, `Hash32` to `Scalar` enum in `src/dat_parser.rs`.
- [x] Removed unused `ScalarRef` enum.
- [x] Implemented heuristic detection in `get_column_claims`:
    - **Hash16/Hash32**: Based on entropy and usage of high-value range.
    - **Color**: Supports `#XXXXXX`, `#XXXXXXXX`, `0xXXXXXX`, and `0xXXXXXXXX`.
    - **File/Directory**: Case-insensitive validation against the VFS file list.
- [x] Updated tests to lowercase and sort the VFS file list for reliable detection.
- [x] Verified `rarity.datc64` offset 32 is correctly detected as `Color`.
- [x] Verified `hideouts.datc64` offset 16 is correctly detected as `Hash16`.
- [x] Verified `hideouts.datc64` offset 18 is correctly detected as `File` (after case-insensitivity fix).
- [x] **Fix Array Detection**: `hideouts.datc64` offset 52 now correctly detected as an array.
- [x] **Verify ForeignRef at Offset 69**: Confirmed as `ForeignRow` using updated max-value heuristic.
- [x] **Conflict Resolution**: Implemented a second-pass conflict resolution stage in `get_all_column_claims` to prioritize high-confidence types (File, Directory, Color, Array) over less likely ones (Hash16, Hash32, String).
- [x] **Heuristic Refinement**: Relaxed count constraints and removed strict offset monotonicity.
- [x] **Cleanup**: Removed debug `println!` statements from `src/dat_parser.rs`.
- [x] **Fix Generated Table Tests**:
    - Filtered out invalid columns (errors in schema) in `lib/types.libsonnet` to prevent OOB reads.
    - Updated `src/dat_parser.rs` to use 64-bit offsets for string arrays, fixing `ground_effects` failures.
    - All 718 tests passed.

## Remaining Tasks
- [ ] (Optional) Add logic to support both 32-bit and 64-bit string arrays if support for legacy .dat files is needed (currently defaults to 64-bit).
- [x] Evaluate breaking cell validation logic in `src/dat_parser.rs` out into a separate modules, one per type, with another for arrays that can use the basic types as a second stage. The goal is to get `dat_parser.rs` to be primarily a parser.
  - **Status**: *Completed*. Created `src/types.rs` for shared types and `src/heuristics.rs` for validation logic. `src/dat_parser.rs` now delegates validation to `heuristics.rs`.
- [ ] Split type validation into a few phases. The first phase would be to do checks that can with 100% confidence rule out the possibility of a type being at an offset. The second would be for checks that demonstrate with a high degree of certainty but not 100% that a type is not at an offset. The third would indicate that an offset likely is a specific type. The fourth would be for tests that have close to a 100% certainty that a type exists at a current offset. The result of these checks should be optimized for evaluating the possibilities of a specific offset extremely quickly with minimal memory and cpu overhead. I'm thinking a struct that contains booleans for each possible type so we can bitmap check each, but I'm not certain.
- [x] Push as much precomputation of statistical things up into the shared code like the xor and max and min. Make a new struct that holds the stats per byte, and pre-calculate it all once for the entire table. Then the individual type checks can avoid iterating the entire column over and over to calculate the same or similar stats
