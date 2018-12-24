
#[allow(dead_code)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

struct WithTests {
    x:u8,
    y:u8,

    #[cfg(test)]
    test:i32,
    #[cfg(test)]
    xtest: i32,
}

#[allow(dead_code)]
fn use_tests() {
    let x = WithTests {
        x: 10,
        y: 20,

        #[cfg(test)]
        test: 100,
        #[cfg(test)]
        xtest: 200,
    };
    println!("{}, {}", x.x, x.y);

    #[cfg(test)]
    println!("{}, {}", x.test, x.xtest);

}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate macros;
    use macros::grok;

    #[grok(foo)]
    fn test() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        //assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_conditional_code() {
        let w = WithTests {
            x: 100,
            y: 200,
            test: 1000,
            xtest: 2000,
        };

        assert_eq!(w.x, 100);
    }
}