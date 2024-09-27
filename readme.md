# hold-macro

This small library crate provides an alternative `todo!()` macro, with the additional functionality of being able to suppress any errors relating to unused values.

See [the source code docs](https://docs.rs/hold-macro/0.1.0/hold_macro/) for usage information.

## Rationale

Using rust's standard `todo!()` macro does not suppresss any warnings for unused values.
This can be a bit annoying, as sometimes you want to add placeholder values that you're gonna end up using. An example is defining a function signature to add elsewhere in your code, or defining variables you know you're gonna end up using but not implementing the rest of the logic.

Leaving those errors untouched can result in a polluted `cargo check` or clippy output, possibly making you ignore other legitimately unused values or other issues in your code.

You can throw on a proc macro to suppress all unused warnings, but you'd have to use both the proc macro and `todo!()`, and this could hide any actually unwanted values. You may also forget to remove both elements when working on your implementation.

Using `hold!` provides an atomic solution to both issues, so you can temporarily work elsewhere in your code and not have any more warnings than necessary.