# combine_traits macro for rust
A combine_traits! is a single macro wich can be used to declare a new trait wich is no more than a combination of existing traits.
# How to use
To create a new Trait call the macro `combine_traits!` with the name as the first argument.
After a `;` list all "sub_traits" seperated by `,`.
# Example:
```rust
use combine_traits::combine_traits;
use std::fmt::{Display, Debug};
combine_traits!(DisplayAndDebug; Display, Debug);
 
fn display_vs_debug<T: DisplayAndDebug>(x: T)->String {
    format!("Display:{}Debug:{:?}", x, x) }
assert_eq!(display_vs_debug(10), "Display:10Debug:10");
```
