[![currencies](https://img.shields.io/badge/BTC,_BCH-blueviolet.svg?logo=bitcoin&style=flat)](https://github.com/TheAwiteb#donating)

<div align="center">
    <img src="./logos/solwalrs-logo.png">
    <h1>Solwalrs</h1>
    A simple and easy to use CLI Solana wallet<br>

<a href="https://www.gnu.org/licenses/">
  <img src="https://img.shields.io/badge/license-GPLv3-orange.svg" alt="License">
</a>
<a href="https://rust-lang.org/">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg" alt="Rust">
</a>
<br>
<a href="https://crates.io/crates/solwalrs">
    <img src="https://img.shields.io/crates/v/solwalrs.svg">
  </a>
<br>
<a href="https://github.com/TheAwiteb/solwalrs/actions/workflows/ci.yml">
  <img src="https://github.com/TheAwiteb/solwalrs/actions/workflows/ci.yml/badge.svg" alt="Continuous Integration">
</a>
<br>
<a href="https://github.com/TheAwiteb/solwalrs/actions/workflows/release.yml">
  <img src="https://github.com/TheAwiteb/solwalrs/actions/workflows/release.yml/badge.svg" alt="Release">
</a>

</div>


## Requirements
- Cargo 1.62.0 or higher (https://doc.rust-lang.org/cargo/getting-started/installation.html)
- OpenSSL (https://www.openssl.org/source/)

## Installation
### Using cargo
You can install solwalrs using cargo (recommended):
```bash
cargo install solwalrs
```
After installing, you can run solwalrs using `solwalrs` command. If you get an error, make sure that your `PATH` environment variable contains the directory where cargo installs binaries. You can find the binary directory here:
`$HOME/.cargo/bin`
### Building from source
```bash
git clone https://github.com/TheAwiteb/solwalrs.git
cd solwalrs
cargo build --release
```
After building, the binary will be located at `target/release/solwalrs`, you can copy it to your `PATH` or run it directly from the `target/release` directory.


## Usage
```bash
A simple CLI Solana wallet, supporting multiple keypairs

Usage: solwalrs [OPTIONS] [COMMAND]

Commands:
  keypair  Commands for managing keypairs [aliases: kp]
  help     Print this message or the help of the given subcommand(s)

Options:
      --app-file <APP_FILE>  The path to the app file
  -h, --help                 Print help information (use `--help` for more detail)
  -V, --version              Print version information
```
> Use `solwalrs help <command>` to get more information about a command. For example, `solwalrs help keypair`

## Features
- Supports multiple keypairs
- Possibility to mark some keypair as default keypair
- Create a new keypair
- View your keypairs
- Delete your keypair

## Our goals (roadmap)
You can see our goals in this issue: [#1](https://github.com/TheAwiteb/solwalrs/issues/1)

## Safety
Solwalrs stores your private key in a file called `solwalrs.json`[1]. This file is encrypted using Fernet (symmetric encryption) by [fernet](https://crates.io/crates/fernet) crate. The encryption key is derived from a password that you provide. The password is never stored anywhere. If you lose your password, you will lose access to your wallet. Use a password manager to generate a strong password and store it somewhere safe.

[1] The file path will printed to the console when you create a new keypair, you can change the file path by setting the `--app-file` flag. For example, `solwalrs --app-file /path/to/file keypair new testwalletname`

## Security
If you discover a security vulnerability within this project, please send me an email at [Awiteb@hotmail.com](mailto:awiteb@hotmail.com) or through the telegram [@TheAwiteb](https://t.me/TheAwiteb). All security vulnerabilities will be promptly addressed.

## Images
<!-- Table contain the images -->
| Create a new keypair | View your keypairs |
|:---:|:---:|
| ![Create a new keypair](https://i.suar.me/A8YlV/l) | ![View your keypairs](https://i.suar.me/yMm47/l)

## License
<div align="center">
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html">
      <img  src="https://www.gnu.org/graphics/gplv3-with-text-136x68.png" alt="GPLv3 logo" width="100" height="50">
  </a>

This project is licensed under the terms of the GNU General Public License v3.0. See <https://www.gnu.org/licenses/gpl-3.0.html> for more details.
</div>

## Contributors
<div align="center">
Thanks for all the contributors who helped make this project better!<br>

<a href="https://github.com/TheAwiteb/solwalrs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=TheAwiteb/solwalrs" />
</a>
</div>
