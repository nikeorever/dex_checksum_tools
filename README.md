# `dex_checksum_tools`

A set of command-line tools for calculating and validating Adler-32 checksums for dex files.


## Usage

### Calculates the current checksum from the DEX file's header

```
$ dex_checksum_tools current-checksum /path/to/input.dex
```

### Calculates the expected checksum for the DEX file

```
$ dex_checksum_tools expect-checksum /path/to/input.dex
```

### Corrects the checksum in the DEX file header if it does not match the expected checksum

```
$ dex_checksum_tools correct-checksum /path/to/input.dex /path/to/output.dex
```

## Install

### Rust / Cargo

```
$ cargo install dex_checksum_tools
```

[![Latest version](https://img.shields.io/crates/v/dex_checksum_tools.svg)](https://crates.io/crates/dex_checksum_tools)

The binary will be placed in your `~/.cargo/bin` which should be on your shell's `$PATH`.

### Docker

A container with the binary is available from Docker Hub.

* `lenoxxian/dex_checksum_tools` [![Docker Image Version](https://img.shields.io/docker/v/lenoxxian/dex_checksum_tools?sort=semver)][hub]

[hub]: https://hub.docker.com/r/lenoxxian/dex_checksum_tools/

Use `docker run` instead of directly using the binary.

```
docker run -i -a STDIN -a STDOUT -v /local/volume:/container/volume lenoxxian/dex_checksum_tools dex_checksum_tools <subcommand>
```

## License

    Copyright 2021 Lenox Enjoy

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
