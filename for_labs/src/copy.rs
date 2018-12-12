#[cfg(test)]
mod tests {
    #[test]
    fn implement_copy() {
        struct Foo {
            x: u8,
            y: u8,
        }

        impl Copy for Foo {}

        impl Clone for Foo {
            fn clone(&self) -> Self {
                Foo { x: self.x, y: self.y }
            }
        }
    }

//    #[test]
//    fn implement_copy_with_string() {
//
//        struct Foo {
//            x: u8,
//            y: String,
//        }
//
//        impl Copy for Foo{}
//
//        impl Clone for Foo{
//            fn clone(&self) -> Self {
//                Foo {x:self.x, y:self.y.clone()}
//            }
//        }
//
//
//    }

}
