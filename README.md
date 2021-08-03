# Rualdi (Rust Aliasing Directory)

[![crates.io](https://img.shields.io/crates/v/rualdi?logo=Rust)](https://crates.io/crates/rualdi)
[![Crates.io](https://img.shields.io/crates/d/rualdi?label=crates.io%20downloads&logo=Rust)](https://crates.io/crates/rualdi)
[![Travis (.com)](https://img.shields.io/travis/com/Jarsop/rualdi?label=Travis%20CI&logo=Travis)](https://travis-ci.com/Jarsop/rualdi)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/Jarsop/rualdi/Main%20workflow/master?label=Main%20workflow&logo=GitHub)](https://github.com/Jarsop/rualdi/actions?query=workflow%3A%22Main+workflow%22)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Jarsop/rualdi/Release?label=Release&logo=Github)](https://github.com/Jarsop/rualdi/actions?query=workflow%3A%22Release%22)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Jarsop/rualdi/Security%20audit?label=Audit&logo=Github)](https://github.com/Jarsop/rualdi/actions?query=workflow%3A%22Security+audit%22)
[![GitHub releases](https://img.shields.io/github/v/release/Jarsop/rualdi?color=blue&label=GitHub%20Releases&logo=GitHub&sort=semver)](https://github.com/Jarsop/rualdi/releases)
[![Crates.io](https://img.shields.io/crates/l/rualdi/0.1.4)](https://crates.io/crates/rualdi)
[![Codecov](https://img.shields.io/codecov/c/github/Jarsop/rualdi)](https://codecov.io/gh/Jarsop/rualdi)

[![asciicast](https://asciinema.org/a/428773.svg)](https://asciinema.org/a/428773)

The fork is adding color to the output when listing the aliases and environment variables, as well as creating an `IndexMap` that will shorten directory names based off of common environment variables.

### TODO
* Fix/add tests now after adding colored output
* Bash completions
* Color the `%HASH` mapping differently

### Table of Contents

- [Rualdi (Rust Aliasing Directory)](#rualdi-rust-aliasing-directory)
    - [TODO](#todo)
    - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Examples](#examples)
  - [Getting started](#getting-started)
    - [Step 1: Installing `rualdi`](#step-1-installing-rualdi)
      - [From Cargo registry](#from-cargo-registry)
      - [From source](#from-source)
      - [On Debian](#on-debian)
        - [From source](#from-source-1)
        - [From .deb pre-built released](#from-deb-pre-built-released)
      - [Other (via pre-compiled binary)](#other-via-pre-compiled-binary-)
    - [Step 2: Adding `rualdi` to your shell](#step-2-adding-rualdi-to-your-shell)
      - [`bash`](#bash)
      - [`zsh`](#zsh)
  - [Configuration](#configuration)
    - [`init` flags](#init-flags)
    - [Environment variables](#environment-variables)
  - [`fzf` integration](#fzf-integration)
      - [No arguments](#no-arguments)
      - [`pushd` wrapper](#pushd-wrapper)
      - [Recently visited directories](#recently-visited-directories)
      - [`rad` wrapper](#rad-wrapper)
  - [Completions](#completions)
      - [Installation](#installation)
  - [Extra](#extra)
      - [Subcommand Aliases](#subcommand-aliases)
      - [Documentation for Crates](#documentation-for-crates)

## Introduction

Rualdi allows you to create aliases for directories and provides
an encapsulation of the builtin `cd` command for easy change of the working directory.
You can also add environment variable which points on an alias.
All variables sourced in your environment are prefixed by `RAD_`.

Inspired by [`zoxide`](https://github.com/ajeetdsouza/zoxide) code.

## Examples
```sh
rada workdir           # Add current directory with workdir as alias
rada www /var/www      # Add /var/www directory with www as alias
rada stuff ~/stuff     # Works with home tilde alias

radax workdir          # Add current directory with workdir as alias
                       # and add environment variable named RAD_WORKDIR
                       # in current environment and in configuration file

radax workdir . wd     # Add current directory with workdir as alias
                       # and add environment variable named RAD_WD
                       # in current environment and to the configuration file

radx workdir wd        # Add environment variable named RAD_WD which points
                       # on alias workdir in current environment
                       # and to the configuration file

radx workdir           # Add environment variable named RAD_WORKDIR
                       # which points on alias workdir in current environment
                       # and to the configuration file

radxn workdir wd       # Add environment variable named RAD_WD which points
                       # on alias workdir in current environment
                       # without adding it to the configuration file

rad www/some-site      # Perform cd in /var/www/some-site
rad -                  # Go back to previous directory by cd'ing to it
rad -4                 # With zsh, this acts as a pushd wrapper

radr workdir           # Remove workdir alias and environment variable associated if exists
radr www stuff         # Works with multiple aliases at same time

radrx workdir          # Remove environment variable which points on alias workdir

radl                   # List aliases and environment variables

radf                   # List directories with fzf and cd to selection
                       # There are several more options with this function explained below
```

## Getting started

### Step 1: Installing `rualdi`

#### From Cargo registry
```sh
cargo install rualdi -f
```

#### From source
For this fork, install it this way
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

##### From .deb pre-built released
You can download a pre-compiled `.deb` package from the
[releases](https://github.com/Jarsop/rualdi/releases) page and add run:

```sh
sudo dpkg -i /target/debian/rualdi_<version>_<arch>.deb
```

#### Other (via pre-compiled binary) [![GitHub releases](https://img.shields.io/github/v/release/Jarsop/rualdi?color=blue&label=GitHub%20Releases&&logo=GitHub&sort=semver)](https://github.com/Jarsop/rualdi/releases)
Alternatively, you can also download a pre-compiled binary from the
[releases](https://github.com/Jarsop/rualdi/releases) page and add it to
your `PATH`.

### Step 2: Adding `rualdi` to your shell

Currently only `bash` and `zsh` are supported.

#### `bash`

Add the following line to your `~/.bashrc`:

```sh
eval "$(rualdi init bash)"
```

#### `zsh`

Add the following line to your `~/.zshrc`:

```sh
eval "$(rualdi init zsh)"
```

## Configuration

### `init` flags

- `--cmd`: change the `rad` command (and corresponding aliases) to something else.

### Environment variables

- `$_RAD_ALIASES_DIR`: directory where `rualdi` will store its aliases configuration file
  (default: platform-specific; see the [`dirs-next` documentation] for more information)
- `$_RAD_NO_ECHO`: when set to `1`, `rad` will not print the matched directory before navigating to it
- `$_RAD_RESOLVE_SYMLINKS`: when set to `1`, `rad` will resolve symlinks before print the matched directory.

## `fzf` integration

**Requires**:
  * `fzf`

The command `radf` (or `<your_cmd>f`) provides a way to use `rualdi` like [`formarks`](https://github.com/wfxr/formarks) (a `zsh` plugin), which
allows one to display the directory aliases with `fzf` and then `cd` to the selection.

#### No arguments
```sh
# A query here is optional
radf <query>
```

#### `pushd` wrapper
```sh
# This can be any digit
radf -3
```

#### Recently visited directories
```sh
# A query here is optional
radf -d <query>
```

#### `rad` wrapper
```sh
# Will also go back to most recently visited directory
radf -
rad -

# If query is an exact match with an alias, then it's the same behavior as `rad`
radf <query>
```

## Completions

As of now, only `zsh` completions are available.
This works the best with [`fzf-tab`](https://github.com/aloxaf/fzf-tab), which completes your command with `fzf` when using `<TAB>`

#### Installation

This command prints the completions to `stdout`, so it can be redirected to file file and placed in your `fpath`.
These completions only work with the actual `rualdi` binary, and therefore will not work with the aliases that are set because they are all individual functions.

To get completions for the aliases, move the file `completions/_rualdi_funcs` into your `fpath` as well.

```sh
rualdi completions shell zsh > _rualdi
```

## Extra

#### Subcommand Aliases

Another way to use `rualdi` is to set `rualdi` itself to an alias, and use each subcommands' own alias.

For example, in your `.zshrc` or `.bashrc`, place `alias r="rualdi"`. Then use the following aliases:

```sh
r a    # rualdi add
r ax   # rualdi add-env
r i    # rualdi init
r l    # rualdi list
r la   # rualdi list-alias
r lx   # rualdi list-env
r r    # rualdi remove
r rx   # rualdi remove-env
r res  # rualdi resolve
r resx # rualdi resolve-env

r comp # rualdi completions
```

#### Documentation for Crates

* [`dirs-next`]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
