#![feature(plugin,const_fn)]
#![plugin(stainless)]

describe! top_level {
    given {
        let mut foo = 1;
    }

    describe! nested {
        given {
            assert_eq!(foo, 1);
            foo += 1;
        }

        when "we check foo" {
            assert_eq!(foo, 2);
            foo += 1;
        }

        then {
            assert_eq!(foo, 3);
            foo += 1;
        }
    }

    then {
        assert_eq!(foo, 4);
    }
}
