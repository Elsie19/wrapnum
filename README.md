# WrapNum

> Have you ever found yourself needing a counter with an arbitrary wraparound limit and having to handroll some mediocre implementation? Well here you go. Just set the max value and forget about ever needing to wonder, "did I need to call my wrap function here?"

## Installation

To add `WrapNum` to your project, just run:

```bash
cargo add wrapnum
```

And you're all good to go!

## Usage

The main entrypoint for `WrapNum` is with the `wrap!` macro. There are multiple ways of using the macro, but the most common use-case is with one value: the max limit:

```rs
use wrapnum::wrap;

fn main() {
    let mut my_num = wrap!(5);
    assert_eq!(my_num + 6, 1)
}
```

It also works with subtraction 😜 (as it should, idk why I wrote that).

You can also pass in two values to specify a min and max, or a range:

```rs
use wrapnum::wrap;

fn main() {
    let my_num = wrap!(5..=50); // Same as `wrap!(5, 49)`.
}
```

Refer to [the docs](https://docs.rs/wrapnum/latest/wrapnum/macro.wrap.html#running) for more information on how to create a `WrapNum`.

Other than that, there isn't much else. It should behave exactly like an integer type should, and if not, [open an issue](https://github.com/Elsie19/wrapnum/issues) and we can get it fixed.
