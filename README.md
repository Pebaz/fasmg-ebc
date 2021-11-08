# fasmg-ebc-rs

> **fasmg-ebc-rs** uses the Flat Assembler to assemble UEFI Bytecode
applications very easily. It is a repackaging of the excellent
[fasmg-ebc program](https://github.com/pbatard/fasmg-ebc) but for the Rust 🦀
community. For more information on how to use it, please see that repository.

## Installation

This project relies upon the Flat Assembler G. You can download it from
[here](https://flatassembler.net/download.php), *but make sure you install
`fasmg` and not `fasm`*. It's the one at the bottom of the page above `FASMARM`
which is a distribution of fasm but for ARM processors. Once downloaded, unzip
in a directory of your choice and add that directory to your PATH so that
`fasmg-ebc-rs` can find it at runtime.

Once you have `fasmg` installed, you can install `fasmg-ebc-rs` like so:

```bash
$ cargo install fasmg-ebc-rs                                 # From Crates.io
$ cargo install --git https://github.com/Pebaz/fasmg-ebc-rs  # From GitHub
```

## Usage

```bash
# Assemble input.asm into a bootable or runnable output.efi executable
$ fasmg-ebc-rs input.asm output.efi
```

## Notes

This project is a fork and rebranding of the
[fasmg-ebc](https://github.com/pbatard/fasmg-ebc) project and I make no claim
that this is original software. Everything included in the `src/include`
directory has been written by [Pete Batard](https://github.com/pbatard).
