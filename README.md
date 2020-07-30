
# type_name_value

Assign a unique type to a value. This crate was made for a [demo of a cool concept](https://github.com/Torrencem/fixed_vec/blob/master/post.md), and I thought it would definitely be useful for other applications. Unfortunately, the original application was unsound, but it might be useful for someone still.

An important note if you plan to use this crate! The reason this was unsound for its original purpose is that it only attaches unique types to macro invocations, not actually to values, and so it can't quite guarantee what it needs to. Check the original repo for more information before using this crate!

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
