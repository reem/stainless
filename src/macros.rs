//! Stainless module for macros

/// Macro to spawn thread with speciefied name
/// or with "unnamed" name by default
/// and return `JoinHandle`
///
/// #Examples
/// ```
/// let handle = actor!("thread",
///            let a = 2;
///            let b = 3;
///            a + b
///    );
/// assert_eq!(5, handle.join().unwrap());
/// ```
#[macro_export]
macro_rules! actor {
    ( $($x:stmt);* ) => {{
        actor!("unnamed", $($x);*)
    }};
    ( $name:expr, $($x:stmt);* ) => {{
        thread::Builder::new().name($name.to_string()).spawn(move || {
            $($x);*
        }).unwrap()
    }};
}
