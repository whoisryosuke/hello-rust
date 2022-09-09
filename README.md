# Hello Rust

Learnings in Rust-land.

> You'll need Rust installed for this, see below. Or you can try Rust online in the [Rust Playground](https://play.rust-lang.org/) without installing anything on your computer.

# Quick Start

1. Clone this repo.
1. Edit entrypoint: `src\main.rs`
1. Build and run using `CTRL + SHIFT + B`

# Getting Started

Check out the official [Getting started guide](http://rust-lang.org/learn/get-started).

The primary way to install Rust is through a tool called Rustup, a Rust installer and version management tool.

[https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

> You may need to install the [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) when prompted to do so. If you are not on Windows see ["Other Installation Methods"](https://forge.rust-lang.org/infra/other-installation-methods.html).

# New Project

Create a new project using Cargo:

```bash
cargo new your-project-name
```

This will generate a new directory called `your-project-name` with the following files:

```
your-project-name
|- Cargo.toml
|- src
  |- main.rs
```

- `Cargo.toml` is the manifest file for Rust. It’s where you keep metadata for your project, as well as dependencies.
- `src/main.rs` is where we’ll write our application code.

## Running the app

Use Cargo to build and run the app:

```bash
cargo run
```

You should see this in your terminal:

```bash
$ cargo run
   Compiling hello-rust v0.1.0 (/Users/ag_dubs/rust/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.34s
     Running `target/debug/hello-rust`
Hello, world!
```

If you look inside the project folder you’ll find a new `target` build folder. Make sure to add this to `.gitignore`.

### VSCode Tasks

- `CTRL + SHIFT + B` to build and run the app.
- `CTRL + SHIFT + P` to open Command Palette, type **Run Task**, then select **cargo run** from list.

[How to launch a Rust application from Visual Studio Code?](https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code)
