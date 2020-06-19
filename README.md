[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Travis CI](https://travis-ci.org/RamXX/phonetic.svg?branch=master)](https://travis-ci.org/github/RamXX/phonetic)
# Phonetic

Phonetic is a small command line utility written in Rust that takes a text input and prints out the corresponding word for each one of its characters, either:

1. As described in the [NATO Phonetic Alphabet](https://www.nato.int/nato_static_fl2014/assets/pdf/pdf_2018_01/20180111_nato-alphabet-sign-signal.pdf), when no other arguments are passed, or
2. As its equivalent in ITU Morse code when the `-m` argument is passed.

I made this as I grew frustrated when trying to spell reservation codes over the phone to customer service representatives, while never completely memorizing the proper words for each letter.

The Morse code functionality was added later mostly to leverage the array of tuples I already had for that effect.

In NATO mode, regular numbers and a few common symbols are also spelled out. These are **not** officially part of the NATO alphabet, so they are provided as a convenience.

In Morse mode, only letters and numbers are printed out. Any character not implemented will return the `(not implemented)` string.

The main functions with the business logic are in a separate module and can easily be imported to use in a web service if needed.

I hope this work is useful to someone else.

## Building

Make sure you have [Rust installed](rust-lang.org/tools/install).

Clone the repo, and enter in the `phonetic` directory.

Build with `cargo build --release`.

If you are on Mac or Linux, move the executable to a directory in your path for easy access: `mv target/release/phonetic /usr/local/bin`.

To run the unit tests, use `cargo tests`, where you should get an output like this:

```
running 2 tests
test phonetic::tests::test_morse ... ok
test phonetic::tests::test_nato_spell ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Using
`phonetic [-m] <text>`

Example:
```
phonetic 'aBc-19Z'

[Word "aBc-19Z" follows next]
Alfa
Bravo
Charlie
Dash
One
Niner
Zulu
```

For morse code:

```
phonetic -m 'aBc-19Z'

[Word "aBc-19Z" follows next]
.-
-...
-.-.
(not implemented)
.----
----.
--..
```

## Documentation
You can browse the module documentation with 

```
cargo doc --open
```

## Limitations
Given that we use `env::args()` to collect the input, we inherit a known issue where the command can panic if 
it receives an invalid Unicode character as input. I should look into replacing this for a more sophisticated library like [Clap](https://clap.rs/) or similar in the future.

## Disclaimer
This software is provided as-is. I don't make any assurances on correctness, so please don't use it for anything critical.

## License
Apache 2.0.
