# Github Control

Command Line Github Pull Requests control center

It is a simple-to-use, efficient for managing your currents open Pull Requests from Github.

## History of creation

I am still working on my terminal. As soon as I complete a pull request, I share it with my team and then start working on something else. I often forget the previous pull request,
and I find myself checking if the CI is passed, if my team reviewed my pull request.
It's quite time consuming on different projects, so I did this to be faster.

Feel free to give me feedback about it :-).

## Screenshots

![Demo usecase for Github-control](github-control-screen.png?raw=true "Github-control Demo")

## Configuration files

There is one essential configuration file wich contain a GITHUB api key.

You'll be able to configurate icons too.

-> What rights should be given ?
(image)

### Linux
Stored inside: `?`

### MAC
Stored inside: `~/Library/Preferences/rs.github-control/`

### Windows
Stored inside: `?`

## Options

- **-r**, **--refresh** <refresh>: Will refresh PRs every X seconds instead of every 60 seconds

## Usage

- Use `j/k` to navigate accross Pull Requests.
- Use `o` to  open the pull request inside your browser.
- Use `q/ESC` to quit.

## Installation

Github-control is written in [Rust](http://www.rust-lang.org). You will need rustc version 1.45.2 or higher. The recommended way to install Rust is from the official download page.
Once you have it set up, a simple `cargo install --path .` will compile github-control and install it into `~/.cargo/bin/`.

### Cargo Install

If you’re using a recent version of Cargo (0.5.0 or higher), you can use the `cargo install` command:

    cargo install github-control

Cargo will build the `github-control` binary and place it in `$HOME/.cargo` (this location can be overridden by setting the `--root` option).

### Homebrew

If you're using [homebrew](https://brew.sh/), you can use the `brew install` command:

    brew tap miaxos/github-control
    brew install github-control

[Formulae](https://github.com/miaxos/homebrew-github-control/blob/master/Formula/github-control.rb)
