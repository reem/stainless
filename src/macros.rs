//! Stainless module for macros

/// Macro to spawn thread with speciefied name
/// or with "unnamed" name by default
/// and return `JoinHandle`
///
/// #Examples
///
/// ```
/// let handle = actor!(1);
/// assert_eq!(handle.join().ok(), Some(1));
/// ```
///
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

/// Macro to spawn bunch of threads with speciefied name format
/// or with "unnamed-{}" format by default
/// and return Vec of `JoinHandle`s
///
/// #Examples
/// ```
/// const NUMBER_OF_THREADS: usize = 10;
/// let handles = actors!(NUMBER_OF_THREADS, 
///     let a = 10;
///     let b = 15;
///     a + b);
/// for h in handles {
///     assert_eq!(h.join().ok(), Some(25));
/// }
/// ```
#[macro_export]
macro_rules! actors {
    ( $num:expr, $($st:stmt);* ) => {{
        actors!("unnamed-{}", $num, $($st);*)
    }};
    ( $name:expr, $num:expr, $($x:stmt);* ) => {{
        let mut handles = Vec::with_capacity($num);
        for i in 0..$num {
            let handle = thread::Builder::new().name(format!($name, i)).spawn(move || {
                $($x);*
            }).unwrap();
            handles.push(handle)
        }
        handles
  }};
}
