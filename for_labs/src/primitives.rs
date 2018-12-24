#[cfg(test)]
mod tests {

    #[test]
    fn primitives_are_copied_to_functions() {
        fn foo(f: f64) {

        }

        let f = 0.0;
        foo(f);
        foo(f);
    }

    #[test]
    fn non_primitives_are_moved() {
        fn foo(s: String) {

        }

        let s = String::from("foo");
        foo(s);

        // doesn't work because s was moved in previous step
        // foo(s);
    }

    #[test]
    fn fp_nan_is_not_comparable() {
        assert_ne!(std::f32::NAN, std::f32::NAN);

    }


}