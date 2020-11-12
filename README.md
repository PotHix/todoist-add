Todoist add
===========

This project is a proof of concept to show that we can have a simple library
implementing Todoist add and exporting bindings for multiple languages using
Rust.

The idea here is to implement the request via Rust and export multiple formats
with conditional compilation.

## Implementations

There are multiple implementations as part of this library. All of them are
written in Rust (not considering what is inside the `examples` directory, of
course). They are:

* Main library in Rust to make the request to the Todoist API
* CLI writen in pure Rust and exposing a `todoist` command
* Java bindings using the `jni` crate and exposing a simple command to Java
* FFI functions to export the functionality to languages that use FFI

## How to compile

You can just compile a simple library by running the vanilla `cargo build` if
you just want to use the library for other **Rust projects**.

        cargo build --release

### All features

If you want to have all features compiled, you can rely on the `all` feature
when compiling:

        cargo build --release --features=all

It contains all the features listed above and has a bigger binary because of
that.

### FFI (for Ruby)

If you want to use the FFI interface to test examples like the one for Ruby,
you have to specify the FFI feature:

        cargo build --release --features=ffi

### JNI

If you want to test the java example, you have to compile the code with support
to JNI:

        cargo build --release --features=java

With JNI enabled, you have to generate the header files and compile the java
program:

        javac -h . examples/Todoist.java
        cd examples
        javac Todoist.java
        LD_LIBRARY_PATH=/full/path/to/todoist-add/target/release java Todoist

The `LD_LIBRARY_PATH` considers that you're using a Linux system. If you're
using something else, you will have to specify the path for the `libtbdoist`
library in a way your system understands.


## Author

* PotHix <pothix at pothix dot com>
