#![feature(plugin)]

#[plugin] extern crate stainless;

#[derive(Copy)]
pub struct X(i32);

#[cfg(test)]
mod test {
    // This use must be pub so that the addition sub-module can view it.
    pub use super::X;

    describe! stainless {
        it "should be able to see outer pub uses" {
            let _ = X(5);
        }
    }
}
