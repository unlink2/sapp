
# sapp

![](https://github.com/unlink2/sapp/actions/workflows/build.yml/badge.svg)
![](https://github.com/unlink2/sapp/actions/workflows/test.yml/badge.svg)

Sprite Atlas Pre Processor (sapp) is a simple tile sheet helper tool. 
It is designed as a helper tool for quickly mirroring, moving and copying tiles to create a more complete tilesheet easily.

## Table of content

- [Installation](#Installation)
- [Usage](#Usage)
- [License](#License)
- [Contributing](#Contributing)
- [TODO](#TODO)

## Installation

This program requires the latest version of Rust.
To install minutecat-cli simplt clone the repository and run:

```sh
cargo install --path ./cli
```

## Usage

The current CLI tool is based entierly on a json config file. 
To run the example use the following command:

```sh
cargo run -- lib/assets/source.png -c example/example.json -o out.png
```

This will apply the transofmration to the source image.

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.

## TODO

Initially this will just be a json-configured command line tool. The intention is to offer a more complete editor in the near future.
