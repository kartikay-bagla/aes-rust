# AES in Rust
This is my implementation of AES in rust.

### Instructions
Run `aes command input_file_path output_file_path key_string` where
- command can be one of `encrypt` or `decrypt`
- `input_file_path` is path to input file
- `output_file_path` is path to output file
- `key_string` is a 128bit string (currently only AES-128 supported)

### TODOs
Components:
- [x] AddRoundKeys - ~~basic, inverse, tests~~
- [x] MixColumns - ~~basic, inverse, tests~~
- [x] ShiftRows - ~~basic, inverse, tests~~
- [x] SubBytes - ~~basic, inverse, tests~~
- [x] KeyExpansion - ~~basic, tests~~
- [x] Chunk Splitting and Padding - ~~basic~~
- [x] CLI - ~~basic~~
- [ ] AES-192 and AES-256 support