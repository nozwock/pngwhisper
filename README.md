# PngWhisper

An implementation of [this project idea](https://picklenerd.github.io/pngme_book/introduction.html#what-are-we-making).

## Build Instructions

To build the project, follow these steps:

1. Install the Rust toolchain for your operating system. You can find installation instructions for [Unix-like OS](https://www.rust-lang.org/tools/install) or [Windows](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers).
2. Navigate to the project directory.
3. Compile the project using the following command:
    ```
    cargo build --release
    ```
4. After successful compilation, the binary will be located in the `./target/release/` directory.

## Usage

**Command Overview:**

* [`pngwhisper`↴](#pngwhisper)
* [`pngwhisper encode`↴](#pngwhisper-encode)
* [`pngwhisper decode`↴](#pngwhisper-decode)
* [`pngwhisper remove`↴](#pngwhisper-remove)
* [`pngwhisper print`↴](#pngwhisper-print)
* [`pngwhisper completions`↴](#pngwhisper-completions)

## `pngwhisper`

**Usage:** `pngwhisper <COMMAND>`

###### **Subcommands:**

* `encode` — Encode a message into a PNG image
* `decode` — Decode a message in a PNG image
* `remove` — Remove a chunk from a PNG image
* `print` — Prints all of the chunks in a PNG file
* `completions` — Generate tab-completion scripts for your shell



## `pngwhisper encode`

Encode a message into a PNG image

**Usage:** `pngwhisper encode [OPTIONS] <FILE> <MESSAGE>`

###### **Arguments:**

* `<FILE>`
* `<MESSAGE>`

###### **Options:**

* `-k`, `--kind <KIND>` — Chunk type for the message

  Default value: `wsPr`



## `pngwhisper decode`

Decode a message in a PNG image

**Usage:** `pngwhisper decode [OPTIONS] <FILE>`

###### **Arguments:**

* `<FILE>`

###### **Options:**

* `-k`, `--kind <KIND>` — Chunk type used for the encoded message

  Default value: `wsPr`



## `pngwhisper remove`

Remove a chunk from a PNG image

**Usage:** `pngwhisper remove [OPTIONS] <FILE>`

###### **Arguments:**

* `<FILE>`

###### **Options:**

* `-k`, `--kind <KIND>` — Chunk type used for the encoded message

  Default value: `wsPr`
* `-a`, `--all`



## `pngwhisper print`

Prints all of the chunks in a PNG file

**Usage:** `pngwhisper print <FILE>`

###### **Arguments:**

* `<FILE>`



## `pngwhisper completions`

Generate tab-completion scripts for your shell

**Usage:** `pngwhisper completions <SHELL>`

###### **Arguments:**

* `<SHELL>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`

---

