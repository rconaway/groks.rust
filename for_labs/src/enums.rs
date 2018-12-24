#[cfg(test)]
mod tests {

    #[test]
    fn a_sipmle_enum() {

        #[derive(Debug, PartialEq, Eq)]
        enum Colors { Red, Green, Yellow, Blue }

        let r = Colors::Red;
        assert_eq!(r, Colors::Red)
    }

    fn an_enum_with_data() {

        #[derive(Debug, PartialEq, Eq)]
        enum Shape {
            Point,                  // simple
            Square(u8),             // tuple
            Rectangle(u8, u8),      // tuple
            Circle {radius: u8},    // struct
        }

        let r = Shape::Rectangle(1,2);
        if let Shape::Rectangle(length, width) = r {
            assert_eq!(length, 1);
            assert_eq!(width, 2);
        }

        let c = Shape::Circle {radius: 10};
        if let Shape::Circle{radius:rad} = c {
            assert_eq!(rad, 10);
        }

        let p = Shape::Point;
        if let Shape::Point = p {
            // do something
        }

    }

    fn an_enum_with_implementation() {

        #[derive(Debug, PartialEq, Eq)]
        enum Vehicle {
            Human{wheels: u8},
            Machine(u8)
        }

        impl Vehicle {
            pub fn get_wheels(&self) -> u8 {
                match *self {
                    Vehicle::Human{wheels:w} => w,
                    Vehicle::Machine(w) => w
                }
            }
        }

        let bike = Vehicle::Human{wheels:2};
        assert_eq!(bike.get_wheels(), 2);

    }

    fn an_enum_variant_with_specialized_implementation() {

        trait Area {
            fn area(&self) -> u8;
        }

        struct Square {side:u8}
        struct Rectangle {length:u8, width:u8}

        impl Area for Square {
            fn area(&self) -> u8 {
                self.side * self.side
            }
        }

        impl Area for Rectangle {
            fn area(&self) -> u8 {
                self.length * self.width
            }
        }

        enum Shape {
            Square(Square),
            Rectangle(Rectangle),
        }

        impl Area for Shape {
            fn area(&self) -> u8 {
                (self as &Area).area()
            }
        }

        let s = Shape::Square( Square {side: 5});
        let r = Shape::Rectangle(Rectangle{length:10, width: 20});

        assert_eq!(s.area(), 25);
        assert_eq!(r.area(), 200);
    }


}