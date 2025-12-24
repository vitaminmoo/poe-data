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
- [x] **Heuristic Refinement**: Relaxed count constraints and removed strict offset monotonicity.
- [x] **Cleanup**: Removed debug `println!` statements from `src/dat_parser.rs`.
- [x] **Fix Generated Table Tests**:
    - Filtered out invalid columns (errors in schema) in `lib/types.libsonnet` to prevent OOB reads.
    - Updated `src/dat_parser.rs` to use 64-bit offsets for string arrays, fixing `ground_effects` failures.
    - All 718 tests passed.

## Remaining Tasks
- [ ] (Optional) Add logic to support both 32-bit and 64-bit string arrays if support for legacy .dat files is needed (currently defaults to 64-bit).
