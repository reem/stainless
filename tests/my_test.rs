#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! a_panic {
   it "fails to parse" () {
       assert!(true);
   }
}