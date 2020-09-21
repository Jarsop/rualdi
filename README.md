# Rualdi

[![crates.io](https://img.shields.io/crates/v/rualdi?logo=Rust)](https://crates.io/crates/rualdi)
[![Crates.io](https://img.shields.io/crates/d/rualdi?label=crates.io%20downloads&logo=Rust)](https://crates.io/crates/rualdi)
[![Travis (.com)](https://img.shields.io/travis/com/Jarsop/rualdi?label=Travis%20CI&logo=Travis)](https://travis-ci.com/Jarsop/rualdi)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/Jarsop/rualdi/Main%20workflow/master?label=GitHub&logo=GitHub)](https://github.com/Jarsop/rualdi/actions?query=workflow%3A%22Main+workflow%22)
[![GitHub releases](https://img.shields.io/github/v/release/Jarsop/rualdi?color=blue&label=GitHub%20Releases&&logo=GitHub&sort=semver)](https://github.com/Jarsop/rualdi/releases)
![Crates.io](https://img.shields.io/crates/l/rualdi/0.1.4)
[![Codecov](https://img.shields.io/codecov/c/github/Jarsop/rualdi)](https://codecov.io/gh/Jarsop/rualdi)

Rust Aliasing Directory

## Table of contents

- [Introduction](#introduction)
- [Examples](#examples)
- [Getting started](#getting-started)
  - [Installing `rualdi`](#step-1-installing-rualdi)
  - [Adding `rualdi` to your shell](#step-3-adding-rualdi-to-your-shell)
    - [bash](#bash)
    - [zsh](#zsh)
- [Configuration](#configuration)
  - [`init` flags](#init-flags)
  - [Environment variables](#environment-variables)

## Introduction

Rualdi allows you to create aliases on directories and to provide
an encapsulation of the built-in `cd` command function for easy change of working directory.

Inspired by [`zoxide`](https://github.com/ajeetdsouza/zoxide) code.

## Examples
```sh
rada workdir                     # Add current directory with workdir as alias
rada www /var/www                # Add /var/www directory with www as alias
rada stuff ~/stuff               # Works with home tild alias

rad www/some-site                # Perform cd in /var/www/some-site
rad -                            # Go back to previous directory as cd do it

radr workdir                     # Remove workdir alias
radr www stuff                   # Works with multiple aliases at same time

radl                             # List aliases
```

## Getting started

### Step 1: Installing `rualdi`

#### From Cargo registry
```sh
cargo install rualdi -f
```

#### From source
```sh
cargo build --release
cp target/release/rualdi <path>
```
Where `<path>` is the path where you store your binaries.

#### On Debian

##### From source
```sh
cargo install cargo-deb
cargo deb
sudo dpkg -i /target/debian/rualdi_<version>_<arch>.deb
```

##### From .deb prebuilt released
You can download a precompiled `.deb` package from the
[releases](https://github.com/Jarsop/rualdi/releases) page and add run:

```sh
sudo dpkg -i /target/debian/rualdi_<version>_<arch>.deb
```

#### Other (via precompiled binary) [![GitHub releases](https://img.shields.io/github/v/release/Jarsop/rualdi?color=blue&label=GitHub%20Releases&&logo=GitHub&sort=semver)](https://github.com/Jarsop/rualdi/releases)
Alternatively, you can also download a precompiled binary from the
[releases](https://github.com/Jarsop/rualdi/releases) page and add it to
your `PATH`.

### Step 2: Adding `rualdi` to your shell

Currently only `bash` and `zsh` are supported.

#### bash

Add the following line to your `~/.bashrc`:

```sh
eval "$(rualdi init bash)"
```

#### zsh

Add the following line to your `~/.zshrc`:

```sh
eval "$(rualdi init zsh)"
```

## Configuration

### `init` flags

- `--cmd`: change the `rad` command (and corresponding aliases) to something else.

### Environment variables

- `$_RAD_ALIASES_DIR`: directory where `rualdi` will store its aliases configuration file
  (default: platform-specific; see the [`dirs` documentation] for more information)
- `$_RAD_NO_ECHO`: when set to `1`, `rad` will not print the matched directory before navigating to it
- `$_RAD_RESOLVE_SYMLINKS`: when set to `1`, `rad` will resolve symlinks before print the matched directory.

[`dirs` documentation]: https://docs.rs/dirs/latest/dirs/fn.data_local_dir.html
