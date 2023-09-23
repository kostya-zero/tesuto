# Tesuto

<img src="imgs/logo.svg" align="right" width="128px">

Tesuto is a minimalist and lightweight tool for testing.
Tesuto designed to be easy to set up and be as fast as possible.
Tesuto allows developers to focus on solving deployment problems.

## Installation

#### From releases

1. Go to the releases section and select the version you want to install.
2. Download archive and unpack it.
3. Place executable file to the location that exists in  the `PATH` environment variable.

> If you cant run executable with error `permission denied`, run `sudo chmod +x <path_to_tesuto>` to make it possible to run it.

#### From crates.io

1. Download `rustup` and install it by following instructions.
2. Run `cargo install --locked tesuto` and wait until end of compilation.
3. Tesuto ready to use.

#### Build from source

1. Download `rustup` and install it by following instructions.
2. Clone this repository and enter it directory.
3. Run `git checkout latest` to use sources from latest version or `git checkout main` to use unstable version.
4. You can run `cargo build` to build executable with debug info or `cargo build --releases` to build version with optimizations.

## Configuration

Look up in [CONFIGURATION.md](CONFIGURATION.md) file for guides.

## Project layout
```
├─ imgs/           Images for README.
├─ target/         Build directory
├─ tesuto/         App source code.
└─ tesuto_project/ Library that contains project structure for Tesuto.
```
