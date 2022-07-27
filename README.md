# Lance

> CLI for generating Mandelbrot set images.

## Introduction

This is a [Rust] program which plots the [Mandelbrot] set and creates a `.png`
output file. Lance is a work-in-progress and as such it's lacking features
which will be added over time... if I get around to it.

[rust]: https://www.rust-lang.org/
[mandelbrot]: https://en.wikipedia.org/wiki/Mandelbrot_set

## Compile

Once this program is compiled using the command below, copy the binary file
located at `./target/release/lance` and place it on your `$PATH`. Assuming you
want to keep it around.

```
cargo build --release
```

## Launch

Alternatively you can run this program without compiling it first using the
command below.

```
cargo run --release mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20
```

## Tests

Unit tests can be launched by running the command below.

```
cargo test
```

## License

[BSD](LICENSE) © [@KRB OSS](https://github.com/krb-oss)
