# Phonetic
![](https://travis-ci.org/RamXX/phonetic.svg?branch=master)

Phonetic is a small command line utility written in Rust that takes a text input and prints out the corresponding word for each one of its letters as described in the [NATO Phonetic Alphabet](https://www.nato.int/nato_static_fl2014/assets/pdf/pdf_2018_01/20180111_nato-alphabet-sign-signal.pdf).

I made this as I grew frustrated when trying to spell reservation codes over the phone to customer service representatives, and never completely learning the proper words for each letter.

Regular numbers and a few common symbols are also spelled out.

The main function with the business logic is in a separate module and can  easily be used in a web service for non command-line applications.

I hope it's useful to someone else.

## Building

Make sure you have [Rust installed](rust-lang.org/tools/install).

Clone the repo, and enter in the `phonetic` directory.

Build with `cargo build --release`.

If you are on Mac or Linux, move the executable to a directory in your path for easy access: `mv target/release/phonetic /usr/local/bin`.

## Using
`phonetic <text>`

Example:
```
phonetic 'aBc-19Z'
---
Alfa
Bravo
Charlie
Dash
One
Niner
Zulu
```

## License
Apache 2.0.
