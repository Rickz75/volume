<br />
<div align="center">
    <h1>Volume</h1>
    <p>A simple CLI used to control the master volume on linux and macos systems.</p>
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

Consider adding the binary to your path.

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

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Note

-   Only linux and macos systems are supported.
-   Anyone is more than welcome to contribute, even more so if you can improve the C code.
