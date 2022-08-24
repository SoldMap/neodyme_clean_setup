# Neodyme POC_framework

## From me

This is just a ***'clean setup'*** (does not contain pocs and contracts from the awesome breakpoint workshop).
I'm using it for creating new **poc_framework** ***"v. 0.2.0"*** instances locally.

**The actual framework isn't built, or maintained by me**

The original **README.md** is:

## From neodyme

Welcome to our Solana Security Workshop!

All details are in the docs. To check it out online, visit [https://workshop.neodyme.io](https://workshop.neodyme.io).

To build it yourself, install mdbook (`cargo install mdbook`) and run `mdbook serve`.


#### How to add contracts for testing

1. Open the root **/Cargo.toml** file. Add a new workspace named after the actual program: `"program_name"`, 
AND `"pocs"`
2. Open the **/pocs/Cargo.toml**. Add a path to the program: `program_name = { path = "../program_name" }`
3. Make sure you are consistent in namings **;)**
   Open the **/program_name/Cargo.toml**:
    `[package]`
    `name = "program_name"`

#### How to add and run exploits

1. Create **/pocs/src/bin/exploit.rs** 
2. Compile your workspace: `cargo build-bpf --workspace`
3. Run: `cargo run --bin exploit`


##### The Environment 

   `Ubuntu 22.04.1 LTS`
   `rustc 1.60.0`
   `solana-cli 1.10.32`
