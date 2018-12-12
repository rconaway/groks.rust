#[cfg(test)]
mod tests {

    #[test]
    fn overload_op() {
        use std::ops::Add;

        #[derive(Debug)]
        struct Foo {
            x: u8,
            y: u8,
            z: String,
        }

        impl Add for &Foo {
            type Output = Foo;

            fn add(self, rhs: &Foo) -> Foo {
                type Output = Foo;
                Foo {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: {
                        let mut s = self.z.to_owned();
                        s.push_str(&rhs.z);
                        s
                    },
                }
            }
        }

        let a = Foo { x: 1, y: 2, z:String::from("one")};
        let b = Foo { x: 3, y: 4, z:String::from("two")};
        let c = &a + &b;

        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
        assert_eq!(c.z, String::from("onetwo"));

        assert_eq!(a.x, 1);
    }

}