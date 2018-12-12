#[cfg(test)]
mod test {

    #[test]
    fn box_basics() {
        let b = Box::new(5);

        let x = *b + 1;
        assert_eq!(x, 6);

        let msg = format!("{}", b);
        assert_eq!(msg, "5");

        assert_eq!(*b, 5);
    }

    #[test]
    fn box_cons_list() {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = List::Cons(1,
                              Box::new(List::Cons(2,
                                                  Box::new(List::Cons(3,
                                                                      Box::new(List::Nil))))));
    }

    #[test]
    fn smart_pointer_without_deref() {
        struct MyBox<T>(T); // tuple struct

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        let x = 5;
        let y = MyBox::new(x);

        // won't compile
//        let z = *y;
    }

    fn smart_pointer_with_deref() {
        struct MyBox<T>(T); // tuple struct

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, *y);

        // *y is equivalent to *(y.deref())
        let yd = y.deref();
        let yv = *yd;
        assert_eq!(5, yv);
    }

    fn deref_coercion() {
        struct MyBox<T>(T); // tuple struct

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let s = String::from("Rust");
        let m = MyBox::new(s);

        // deref coercion turns
        hello(&m);

        // into this:
        let deref_string: &String = m.deref();
        let deref_slice: &str = deref_string.deref();
        hello(deref_slice);
    }

    #[test]
    fn drop_trait() {
        static mut called: bool = false;

        struct SP {
            data: String,
        }

        impl Drop for SP {
            fn drop(&mut self) { unsafe { called = true; } }
        }

        {
            let sp = SP { data: String::from("foo") };
        }

        assert!(unsafe { called });
    }

    #[test]
    fn drop_manually() {
        static mut called: bool = false;

        struct SP {
            data: String,
        }

        impl Drop for SP {
            fn drop(&mut self) { unsafe { called = true; } }
        }

        {
            let mut sp = SP { data: String::from("foo") };

            // this won't work
            // sp.drop();

            // but this will
            drop(sp);

            assert!(unsafe { called });

            // drop() moves value, so this won't work
            // assert_eq!(sp.data, "");
        }
    }

    #[test]
    fn reference_counted_smart_pointer_basics() {
        use std::rc::Rc;

        let sp = Rc::new(String::from("foo"));

        {
            let r1 = sp.clone();
            assert_eq!(2, Rc::strong_count(&sp));
            {
                let r2 = sp.clone();
                assert_eq!(3, Rc::strong_count(&sp));
            }
        }
        assert_eq!(1, Rc::strong_count(&sp));
    }
}