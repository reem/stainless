#![feature(plugin,const_fn)]
#![plugin(stainless)]

#[macro_use]
extern crate stainless;

pub use std::thread;

describe! simple_concurrent_test {

    it "should create thread which return 1" {
        let handle = actor!(1);
        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    it "should create thread with name 'thread-1'" {
        let handle = actor!("thread-1", ());
        assert_eq!(handle.thread().name(), Some("thread-1"));
    }

    it "should create thread with multiple line of code" {
        let handle = actor!("thread",
            let a = 2;
            let b = 3;
            a + b
        );
        assert_eq!(handle.join().unwrap(), 5);
    }
}