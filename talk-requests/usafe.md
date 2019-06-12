# Unsafe code guidelines
There is opinion, that Rust code doesn't need to be `unsafe` at all,
sometimes even, that `unsafe` shouldn't be available as public
Rust API.

On the other hand, there are people claiming, that `unsafe` isn't bad
at all, and not dangerous as itself, it just can be missused as
everything else.

The third camp points, that `usafe` may possibly degenerate Rust
into level of languages as C, because it allows to disable any
language guarantees possibly in whole codebase.

What is a truth? Maybe there would be good to have lecture what `unsafe`
actually is, with its pros and cons, maybe some examples about "correct"
or "safe" usage of it, and leaving some guidelines about creating `unsafe`
code.

## Sources
* [Rustonomicon](https://doc.rust-lang.org/nomicon/)
* [Unsafe guidelines](https://github.com/rust-lang/unsafe-code-guidelines)
