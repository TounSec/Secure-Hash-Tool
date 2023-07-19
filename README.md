# Secure-Hash-Tool

**SecureHash** _is tool that will first allow you to hash a text, then it can also do the opposite using multiple algorithms_

# Install

```bash
cd Secure-Hash-Tool
cargo build
```

# Release

You can get the binaries compiled under different os in `Releases`

*The text `hash` feature will be released first, in a future version you will also be able to reverse the operation from a hash with the `reverse hash` feature*

# Usage

```bash
cd Secure-Hash-Tool/target/debug
./Secure-Hash-Tool -h
```

    Secure-Hash-Tool v1.0.0

          _        _        _        _        _        _        _        _        _        _
        _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__   _( )__
      _|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|_|     _|
     (_ S _ (_(_ E _ (_(_ C _ (_(_ U _ (_(_ R _ (_(_ E _ (_(_ H _ (_(_ A _ (_(_ S _ (_(_ H _ (_
       |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__| |_( )__|
            🦀🔢 Rust tool for hashing text using multiple algorithms 🔢🦀


    USAGE:
        Secure-Hash-Tool.exe [OPTIONS] <TEXT>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -a, --algorithm <algorithm>    Choose the hash algorithm (
                                                sha256,
                                                sha512
                                            ) [default: sha256]

    ARGS:
        <TEXT>    The text to hash