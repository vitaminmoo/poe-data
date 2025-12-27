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
- [x] **Performance & Allocations**
    - [x] Remove `Vec<Bytes>` allocation in `get_column_claims`: Refactor checkers to consume `column_rows_iter` lazily.
    - [x] Optimize Entropy & Stat Calculation: Use a fixed-size array for `u16` entropy.
    - [x] Implement "Phase 2" Fast-Fail for Strings: Early exit during string iteration if an invalid sequence is found.
- [x] **Modularity & Organization**
    - [x] Extract Type Scanners to `src/scanners/`: Moved specific type heuristics out of `heuristics.rs` into specialized modules using a unified scanning approach.
    - [x] Unify `Scalar` and `Array` handling: Refactored the detection loop to treat Arrays as a container type via `src/scanners/arrays.rs`.
- [x] **Type Detection Coverage**
    - [x] Implement `Interval` Detection: Detect 8-byte columns where `i32_min <= i32_max` consistently.
    - [x] Implement `DateTime` Detection: Identify `u64` values that fall within valid game-relevant epoch ranges.
- [x] Split type validation into a few phases. ...
  - **Status**: *Completed*. Implemented `TypeSet` (bitmask) and `check_phase_1_absolutes` in `src/heuristics.rs`.
  - **Status**: *Refined*. Isolated absolute rejection logic in `src/validators.rs` and exposed via `DatFile::validate_types`.
- [x] Push as much precomputation of statistical things up into the shared code like the xor and max and min.
- [x] Remove duplicate checks from scanners where validators already exclude offsets.
    - [x] Added `is_valid_color` to `src/validators.rs`.
    - [x] Updated `src/heuristics.rs` to filter Color in Phase 1.
    - [x] Simplified `src/scanners/strings.rs` to rely on Phase 1 candidates.
    - [x] Removed redundant iteration in `src/scanners/bool.rs` and `src/scanners/arrays.rs`.
    - [x] Removed redundant zero checks in `src/scanners/hashes.rs`.

## Remaining Tasks
- [ ] Create a binary/tool to export `validate_types` results (e.g., to JSON) for external schema validation.
- [ ] (Optional) Add logic to support both 32-bit and 64-bit string arrays if support for legacy .dat files is needed (currently defaults to 64-bit).
- [ ] Improve `ForeignRow` vs `Int` logic: Refine the distinction between 128-bit keys and large integers using clustering and distribution analysis.