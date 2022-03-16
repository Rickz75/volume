<br />
<div align="center">
    <h1>Volume</h1>
    <p>A simple CLI used to control the master volume on linux systems.</p>
</div>

## About

I wrote this a while ago, and it was originally intended as a learning project for C-Rust FFI, though it still works fine on all the systems I've tested it on.

## Built With

-   Rust and C

## Installation

1. Clone the repo

```sh
git clone https://github.com/Rickz75/volume.git
cd volume
```

1. Build it.

```sh
cargo build --release
```

## Usage

Consider adding the binary to your path, [like so](https://unix.stackexchange.com/questions/26047/how-to-correctly-add-a-path-to-path).

Retrieve the master volume.

```sh
$ volume get
```

Set the master volume, amount must be between 0 and 100.

```sh
$ volume set <amount>
```

Mute the volume

```sh
$ volume mute
```

Unmute the volume

```sh
$ volume unmute
```

### Note

-   Only linux systems are supported.
-   Anyone is more than welcome to contribute, even more so if you can improve the C code.
