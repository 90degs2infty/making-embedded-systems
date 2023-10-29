# `command-rs`

A basic commandline interface targeting the `micro:bit v2` running on the RTIC concurrency framework.

## Requirements

### Hardware

A [`micro:bit v2`](https://microbit.org/).
Note that this crate targets `v2` (as opposed to `v1`).

### Software

#### 0. `rust`

See [the official installation guide](https://www.rust-lang.org/tools/install).

#### 1. `flip-link`:

```console
$ cargo install flip-link
```

#### 2. `probe-run`:

``` console
$ # make sure to install v0.2.0 or later
$ cargo install probe-rs --features cli
```

#### 3. `git`

This crate's build script uses `git` (via the `vergen` crate) to embed VCS information into the binary.
Make sure the `git` executable is available in your `$PATH`.

## Building

```console
$ cargo brb command
```

## Running

```console
$ cargo rbb command
```

Note that this project has _two_ ways of logging to some commandline:

- via RTT, for development purposes\
  To make this available, make sure to build with `DEFMT_LOG=trace`, that is

  ```console
  $ DEFMT_LOG=trace cargo rbb command
  ```

- via UART, for "every day usage" (that is to solve the assignment)

### Interacting with the target

Run the target as described above.

Then open a terminal emulator of your choice (`putty`, `minicom`, ...).
Make sure to use a baud rate of `115200` and disable parity checking.
E.g.

```console
$ minicom -D /dev/ttyACM0 -b 115200
$ # inside the terminal
$ help;
```

Commands have to be specified with a trailing `;`.
Type `;` to clear your input on typos.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
