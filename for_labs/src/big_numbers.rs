#[cfg(test)]
mod tests {
    #[test]
    fn square_root_example() {
        use num::FromPrimitive;
        use num::bigint::BigInt;
        use num::rational::{Ratio, BigRational};

        fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
            let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
            let mut approx = start.clone();

            for _ in 0..iterations {
                approx = (&approx + (&start / &approx)) /
                    Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
            }

            approx
        }

        let sqrt = approx_sqrt(10, 4); // prints 4057691201/1283082416

        let numerator = FromPrimitive::from_u128(4057691201).unwrap();
        let denominator = FromPrimitive::from_u128(1283082416).unwrap();
        let expected = Ratio::new(numerator, denominator);

        assert_eq!(sqrt, expected);
    }

    #[test]
    fn big_fibonacci() {
        use num::bigint::BigUint;
        use num::traits::{Zero, One};
        use std::mem::replace;

        // Calculate large fibonacci numbers.
        fn fib(n: usize) -> BigUint {
            let mut f0: BigUint = Zero::zero();
            let mut f1: BigUint = One::one();
            for _ in 0..n {
                let f2 = f0 + &f1;
                // This is a low cost way of swapping f0 with f1 and f1 with f2.
                f0 = replace(&mut f1, f2);
            }
            f0
        }

        let f = fib(1000);
        let expected = BigUint::new(vec![1556111435, 190401991, 2256560071, 1284402514, 2151428395, 154187752, 1008558256, 775229480, 2751115457, 671514929, 4284660124, 1929785112, 1297430915, 3063393570, 2118306451, 1920627562, 1771810074, 2968009996, 1865167802, 1462942481, 129331906, 2218187]);
            assert_eq!(f, expected);

    }

    #[test]
    fn true_random() {
        use num::bigint::{ToBigInt, RandBigInt};

        let mut rng = rand::thread_rng();
        let a = rng.gen_bigint(1000);

        let low = -10000.to_bigint().unwrap();
        let high = 10000.to_bigint().unwrap();
        let b = rng.gen_bigint_range(&low, &high);

//        let expected = BigInt::new(Sign::NoSign, vec![1681]);
//        assert_eq!(b, expected);
    }

    #[test]
    fn pseudo_random() {
        use num::bigint::{ToBigInt, RandBigInt, Sign, BigInt};
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        let seed: [u8;32] = [0;32];
        let mut rng:StdRng = SeedableRng::from_seed(seed);
        let a = rng.gen_bigint(1000);

        let low = -10000.to_bigint().unwrap();
        let high = 10000.to_bigint().unwrap();
        let b = rng.gen_bigint_range(&low, &high);

        let expected = BigInt::new(Sign::Minus, vec![1440]);
        assert_eq!(b, expected);
    }


    #[test]
    fn pseudo_random_big_range() {
        use num::bigint::{RandBigInt, Sign, BigInt};
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        fn permutations(n:u16, r:u16) -> BigInt {

            fn factorial(n:u16) -> BigInt {
                let mut f:BigInt = BigInt::from(1);
                for x  in 1 ..= n {
                    f *= x;
                }
                f
            }

            factorial(n) / factorial(n - r)
        }

        let seed: [u8;32] = [0;32];

        let range = permutations(1024, 21);

        let mut rng:StdRng = SeedableRng::from_seed(seed);
        let b = rng.gen_bigint_range(&BigInt::from(0u8), &range);

        let expected = BigInt::new(Sign::Minus, vec![1440]);
        assert_eq!(b, expected);
    }

    #[test]
    fn to_and_from_str() {
        use num::bigint::BigInt;
        use std::str::FromStr;

        let x: BigInt =  FromStr::from_str("12345").unwrap();

        assert_eq!(x.to_string(), "12345");
    }

    #[test]
    fn to_and_from_primitives() {
        use num::bigint::BigInt;
        use num::FromPrimitive;
        use num::ToPrimitive;

        let x: BigInt =  FromPrimitive::from_u64(42).unwrap();

        assert_eq!(x.to_i64().unwrap(), 42);

    }


}