# PngWhisper

An implementation of [this project idea](https://picklenerd.github.io/pngme_book/introduction.html).

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

