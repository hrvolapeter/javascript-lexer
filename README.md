# Javascript Lexer

[![Build Status](https://travis-ci.com/retep007/javascript-lexer.svg?branch=master)](https://travis-ci.com/retep007/javascript-lexer)
[![Documentation](https://docs.rs/javascript_lexer/badge.svg)](https://docs.rs/crate/javascript_lexer/)
![Creates](https://img.shields.io/crates/v/javascript_lexer.svg)
![License](https://img.shields.io/crates/l/javascript_lexer.svg)
Javscript lexer implements high performance lexer of javscript as defined by [ECMAScript 9](http://www.ecma-international.org/ecma-262/9.0/index.html)

Output of the lexer together with description can be found in [docs](https://docs.rs/crate/javascript_lexer/)

## Installation

add to `cargo.toml`

``` rust
[dependencies]
javascript_lexer = "0.1"
```

## Example

Lexer is invoked by running

``` rust
extern crate javascript_lexer;

use javascript_lexer::Lexer;

fn main() {
    Lexer::parse_tokens("javascript");
}
```