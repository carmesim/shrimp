# Shrimp

Simplistic password generator writen in Rust.

## Installation

```shell
                        # Clone the repository
git clone https://github.com/carmesim/shrimp 
cd shrimp               # Change directory into it: 
cargo run --release   # Build it with Cargo
```

## Example

    $ shrimp --length=20
    Password: 7TS@utsKi*4peY5bIX7C
    Quality: Excellent

## Usage

    USAGE:
        shrimp [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help         Prints help information
        -L, --lowercase    Enable lowercase letters.
        -n, --numbers      Enable whole numbers like 1, 2, or 8.
        -S, --silence      Suppress all output bar the generated password.
        -s, --symbols      Enable symbols like #, $ or @.
        -u, --uppercase    Enable uppercase letters.
        -V, --version      Prints version information
        -v, --verbose      Activates verbose output. Can be set multiple times for greater info.

    OPTIONS:
        -l, --length <LENGTH>    Sets generated password's length. If not set, 15 will be used.
        -p, --pool <POOL>        Sets the pool of characters from which the password can be made of.

If no pool is set or if none of `-Lnsu` are set, a standard charset of upper and lowercase characters, numbers and special symbols will be used.
