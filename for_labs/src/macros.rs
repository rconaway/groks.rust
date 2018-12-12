macro_rules! calculate {
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // Force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};
        }

#[cfg(test)]
mod tests {

    #[test]
    fn example_macro_from_rust_by_example() {

        calculate! {
            eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
        }

        calculate! {
            eval (1 + 2) * (3 / 4)
        }
    }
}