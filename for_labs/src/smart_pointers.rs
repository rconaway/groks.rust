#[cfg(test)]
mod test {
    use super::*;

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
}