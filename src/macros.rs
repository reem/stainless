//! Stainless module for macros

#[macro_export]
macro_rules! actor {
    ( $($x:expr);* ) => {{
        thread::spawn(move || {
            $($x)*
        })
    }};
    ( $name:expr, $($x:expr);* ) => {{
        thread::Builder::new().name($name).spawn(move ||{
            $($x)*
        }).unwrap()
    }};
}
