#![feature(plugin,const_fn)]
#![plugin(stainless)]

#[macro_use]
extern crate stainless;

pub use std::thread;

describe! simple_concurrent_test {

    it "should create thread which return 1" {
        let handle = actor!(1);
        assert_eq!(handle.join().ok(), Some(1));
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
        assert_eq!(handle.join().ok(), Some(5));
    }

    it "should create bunch of threads with default format names" {
        let handles = actors!(5, ());
        assert_eq!(handles[0].thread().name(), Some("unnamed-0"));
        assert_eq!(handles[1].thread().name(), Some("unnamed-1"));
        assert_eq!(handles[2].thread().name(), Some("unnamed-2"));
        assert_eq!(handles[3].thread().name(), Some("unnamed-3"));
        assert_eq!(handles[4].thread().name(), Some("unnamed-4"));
    }

    it "should create bunch of threads with formated names" {
        let handles = actors!("thread-{}", 5, ());
        assert_eq!(handles[0].thread().name(), Some("thread-0"));
        assert_eq!(handles[1].thread().name(), Some("thread-1"));
        assert_eq!(handles[2].thread().name(), Some("thread-2"));
        assert_eq!(handles[3].thread().name(), Some("thread-3"));
        assert_eq!(handles[4].thread().name(), Some("thread-4"));
    }

    it "should create bunch of threads with calculated results" {
        const NUMBER_OF_THREADS: usize = 10;
        let handles = actors!(NUMBER_OF_THREADS, 
            let a = 10;
            let b = 15;
            a + b);
        for h in handles {
            assert_eq!(h.join().ok(), Some(25));
        }
    }
}