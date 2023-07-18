# ninit

`ninit` is a minimalistic Rust project designed as a platform to swiftly hook into an operating system. It's designed with simplicity and ease of modification in mind, allowing developers to define minimal syscall operations and get a system up and running quickly.

## Project Structure

The project contains a JSON file describing the target architecture, `x86_64-unknown-none.json`, which provides LLVM with details such as the target architecture, pointer width, data layout, endianness, and more.

The main Rust source code is located under the `src` directory. This contains several modules:

- `main.rs`: The entry point for the program.
- `allocator.rs`: Hooks rusts internal allocator into `SYS_MMAP`.
- `internals.rs`: Contains various glue pieces like the `panic_handler`.
- `print.rs`: Contains macros for printing to `STDOUT` or `STDERR` using `SYS_WRITE`.
- `syscall.rs`: Contains definitions for needed x86_64 syscalls only.

## How It Works

The entry point of the program is in `main.rs` with the `_start` function. This function simply calls `main()`, which demonstrates the functionality of the program by printing to standard output and standard error.

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO: pre/post tweaks and error handling might go here.
    // This function only serves to GOTO main() currently.
    main();
    sys_exit(0);
}

fn main() {
    // print macros always send to STDOUT
    println!( "STDOUT: this works?"    );
    print!(   "        this works?!\n" );

    // eprint macros always send to STDERR
    eprintln!("STDERR: works?!?"       );
    eprint!(  "        works?!?\n"     );

    sys_exit(0);
}
```

## Usage

The purpose is to provide a clean, minimal platform to jumpstart any OS-related project in Rust.

You probably shouldn't use this, other than as a template to begin a new project where much of this code is replaced.
