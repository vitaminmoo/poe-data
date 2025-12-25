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
- [ ] **Performance & Allocations**
    - [ ] Remove `Vec<Bytes>` allocation in `get_column_claims`: Refactor checkers to consume `column_rows_iter` lazily to avoid allocating a vector for every row in every column.
    - [ ] Optimize Entropy & Stat Calculation: Use a fixed-size array for `u16` entropy and a shared `HashMap` buffer for `u32` to reduce allocation overhead.
    - [ ] Implement "Phase 2" Fast-Fail for Strings: Early exit during string iteration if an invalid sequence is found, rather than collecting all strings first.
- [ ] **Modularity & Organization**
    - [ ] Extract Type Scanners to `src/scanners/`: Move specific type heuristics out of `heuristics.rs` into specialized modules (e.g., `integers.rs`, `strings.rs`, `arrays.rs`) using a unified scanning trait.
    - [ ] Unify `Scalar` and `Array` handling: Refactor the detection loop to treat Arrays as a container type more consistently with Scalars.
- [ ] **Type Detection Coverage**
    - [x] Implement `Interval` Detection: Detect 8-byte columns where `i32_min <= i32_max` consistently.
    - [ ] Implement `DateTime` Detection: Identify `u64` values that fall within valid game-relevant epoch ranges.
    - [ ] Improve `ForeignRow` vs `Int` logic: Refine the distinction between 128-bit keys and large integers using clustering and distribution analysis.
- [x] Split type validation into a few phases. The first phase would be to do checks that can with 100% confidence rule out the possibility of a type being at an offset. The second would be for checks that demonstrate with a high degree of certainty but not 100% that a type is not at an offset. The third would indicate that an offset likely is a specific type. The fourth would be for tests that have close to a 100% certainty that a type exists at a current offset. The result of these checks should be optimized for evaluating the possibilities of a specific offset extremely quickly with minimal memory and cpu overhead. I'm thinking a struct that contains booleans for each possible type so we can bitmap check each, but I'm not certain.
  - **Status**: *Completed*. Implemented `TypeSet` (bitmask) and `check_phase_1_absolutes` in `src/heuristics.rs`. This allows fast, absolute rejection of invalid types before expensive heuristics.
- [x] Push as much precomputation of statistical things up into the shared code like the xor and max and min. Make a new struct that holds the stats per byte, and pre-calculate it all once for the entire table. Then the individual type checks can avoid iterating the entire column over and over to calculate the same or similar stats
