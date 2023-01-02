
/// Macro to generate a new trait which just combines other already existing traits. Including an default implementation for every type which fulfills the "sub traits"
/// It requires the name of the new trait as the first argument and the traits the new ones are based on as the second argument
/// # Example:
/// ```
/// use combine_traits::combine_traits;
/// use std::fmt::{Display, Debug};
/// combine_traits!(DisplayAndDebug; Display, Debug);
/// 
/// fn display_vs_debug<T: DisplayAndDebug>(x: T)->String {
///     format!("Display:{}Debug:{:?}", x, x)
/// }
/// assert_eq!(display_vs_debug(10), "Display:10Debug:10");
/// ```
#[macro_export]
macro_rules! combine_traits {
    ($name:ident; $($sub_traits:path),+) => {
        //Define the new Trait
        
        trait $name: $($sub_traits +)+ {}
        //Default/Empty implementation for every type which fulfills the required traits
        impl<T> $name for T where T: $($sub_traits +)+ {}
    };
}