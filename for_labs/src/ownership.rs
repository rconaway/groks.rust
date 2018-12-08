#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
mod tests {
//    use super::*;

    #[test]
    fn drop_is_called_when_var_goes_out_of_scope() {
        static mut DROPS: isize = 0;
        pub struct Empty {}

        impl Drop for Empty {
            fn drop(&mut self) {
                unsafe { DROPS += 1; }
            }
        }

        // wrap variable declaration in scope so it can go out of scope
        { let _e = Empty {}; }

        unsafe { assert_eq!(DROPS, 1); }
    }

    #[test]
    fn cannot_use_variable_after_value_is_moved_from_it() {
        let s1 = String::from("one");

        // this is a 'move', not a shallow copy
        let s2 = s1;

        // "not on my watch", says the compiler
        // let  x = s1;
    }

    #[test]
    fn a_type_cannot_be_both_droppable_and_copyable() {
        struct S {
            a: isize
        }
        impl Drop for S { fn drop(&mut self) {} }

        // Compiler fails
        // impl Copy for S {}
    }

    #[test]
    fn a_user_type_can_be_placed_on_the_stack_and_use_copy_semantics() {
        #[derive(Copy, Clone)]
        struct S {
            a: isize,
        }

        let s1 = S { a: 42 };
        let s2 = s1;

        // This is possible because of Copy trait
        let x = s1.a;
    }

    #[test]
    fn ownership_transfers_into_functions_as_though_parameters_are_assigned() {
        fn takes_ownership(_s: String) {}
        fn makes_copy(_i: isize) {}

        let i1 = 10;
        makes_copy(i1);
        // allowed because simple types are copied
        let i2 = i1;

        let s1 = String::from("foo");
        takes_ownership(s1);
        // not allowed because String is moved
        // let  s2 = s1;;
    }

    #[test]
    fn ownership_transfers_out_of_functions() {
        fn gives_ownership() -> String {
            let some_string = String::from("foo");
            some_string
        }

        let s1 = gives_ownership();
        let s2 = s1;
    }

    #[test]
    fn a_reference_can_be_borrowed() {
        fn calculate_length(s: &String) -> usize {

            // Can't because reference is immutable
            // s.push_str("foo");

            // OK, because s has borrowed "hello"
            s.len()
        }

        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        // ownership of "hello" never left s1
        let s2 = s1;
    }

    #[test]
    fn borrowing_occurs_in_assignments_too() {
        let s = String::from("hello");

        // s never loses ownership
        let s1 = &s;
        let s2 = &s;
        let s3 = s2;
    }

    #[test]
    fn there_can_be_only_one_mutable_reference_at_a_time() {
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r4 = r1;

        let r2 = &mut s;

        // r2 "owns" the mutable reference
        // so this compiles
        let r3 = r2;
        // but this doesn't
        // let r4 = r1;
    }

    #[test]
    fn there_cannot_be_an_immutable_and_mutable_reference() {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;

        // compiler error
        // let r4 = r1;
    }

    #[test]
    fn no_dangling_for_you() {
        // nope
//        fn dangle() -> &String {
//            let s = String::from("hello");
//            &s
//        }
//
//        let reference_to_nothging = dangle();

        // yep
        fn no_dangle() -> String {
            let s = String::from("hello");
            s
        }

        let copied = no_dangle();
    }

    #[test]
    fn recap() {

        // At any given time, you can have either (but not both of) ...

        // one mutable reference
        {
            let mut mutable = String::from("foo");

            let r1 = &mut mutable;
            let r2 = &mut mutable;

            // this compiles
            let r3 = r2;

            // but this doesn't
            // let r3 = r1;

            // I think r2 "takes" the mutable reference, although the book doesn't say that
            // I don't think r1 is being optimized away because optimization is set to 0
        }

        // or any number of immutable references
        {
            let mut foo = String::from("foo");

            let r1 = &foo;
            let r2 = &foo;

            // this compiles
            let r3 = r2;
            let r3 = r1;
        }

        // references must always be valid
        {
            let mut r: &String;

            {
                    let mut foo = String::from("foo");
                    r = &foo;
            }

            // doesn't compile
//            let r2 = r;
        }
    }

    #[test]
    fn string_slices_are_taken_from_references() {

        let s = String::from("hello, world");
        let w1 = &s[0..5];
        assert_eq!(w1, "hello");

        // doesn't compile
        // let w1 = s[0..5];

    }

    #[test]
    fn changing_a_sliced_string_recovers_ownership() {
        let mut s = String::from("Hello, world");

        // immutable borrow occurs here
        let hello = &s[0..5];

        // immutable borrow moved to x
        let x = hello;

        // mutable borrow reclaims ownership to s
        s.clear();

        // so x is not valid
        // let z = x;
    }

    #[test]
    fn str_slice_is_a_more_general_parameter_type() {
        fn takes_a_reference(s: &String) {}
        fn takes_a_slice(s: &str) {}

        let slice = "Hello, world";
        let string = String::from(slice);

        takes_a_reference(&string);
        // doesn't compile
        //takes_a_string_reference(slice);

        // slice parameter can be used like reference
        takes_a_slice(&string);
        takes_a_slice(slice);

    }

    #[test]
    fn non_string_slices() {

        let a = [1,2,3,4,5];

        assert_eq!(&a[0..2], [1,2]);

        assert_eq!(&a[..], [1,2,3,4,5]);

        assert_eq!(&a[..3], [1,2,3]);

        assert_eq!(&a[3..], [4,5]);
    }

    



}

