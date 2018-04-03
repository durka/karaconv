# karaconv

[![Travis CI](https://travis-ci.org/durka/karaconv.svg)](https://travis-ci.org/durka/karaconv)

Karabiner was a very flexible keyboard/mouse remapping program that ran on macOS up to 10.11. Karabiner-Elements is its replacement, and it does mostly the same thing. But there was no way to migrate your old configuration. Until now.

This program converts configuration files from Karabiner (`private.xml`) to Karabiner-Elements (`karabiner.json`).

Caveats:

- The format of `private.xml` is documented, but it is sometimes vague or inconsistent
- The format of `karabiner.json` is ~~completely undocumented~~ documented in a place I did not know about
- Taking the above two points under consideration, I wrote this tool by looking at the docs, my own configuration, and guessing, and I stopped when it was powerful enough to parse my own `private.xml`. It doesn't support the entire format.

If you try `karaconv` and it can't parse your `private.xml`, please post an issue (or pull request)!

## Installation

If you have Rust and Cargo installed, you can simply run `cargo install karaconv`.

Otherwise, you can download the binary from the [Releases tab](https://github.com/durka/karaconv/releases).

## Usage

First, you need to find `private.xml`. On my machine it's in `~/Library/Application Support/Karabiner`. You can find it by opening Karabiner Preferences (the old one), going to the "Misc & Uninstall" tab, and clicking on "Open private.xml". This will open Finder to the folder containing `private.xml`.

Next, find `karabiner.json`. There's no comparable way to open it from Karabiner-Elements Preferences, but I assume (again in the absence of documentation) that it is always in `~/.config/karabiner`.

Now you can run the converter:

```
karaconv -i /path/to/private.xml -o /path/to/karabiner.json
```

This will *add* all configuration from `private.xml` into `karabiner.json` (overwriting any Complex Modifications with the same name, which is helpful if you edit `private.xml` and then run the converter again). The old `karabiner.json` will be backed up first, but you can pass `-n` if you want to just see the new JSON without having it printed anywhere.

