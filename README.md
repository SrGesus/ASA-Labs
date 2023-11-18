# Lab Projects - ASA

## Source Code

Available code for each project:
* [Project 0](proj00/src/main.rs)

Macro for parsing input:
* [line_parser](line_parser/src/lib.rs)

## Requirements

To run rust crates you should use [rustup](https://www.rust-lang.org/tools/install).

### With Nix

If you have nix you can simply open a nix-shell:
```bash
nix-shell
```

## Running 

Clone the repo and run the desired project: 
```bash
git clone https://github.com/SrGesus/ASA-Labs
cd ASA-Labs
cargo run -p proj00
```

Pipe an input:
```bash
cat proj00/input/A.txt | cargo run -p proj00
```
