#[cfg(test)]
mod tests {
    #[test]
    fn iterate_over_hash() {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.input(b"hello world");

        let result = hasher.result();
        let expected: [u8; 32] = hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(result[..], expected[..]);

        let mut it = result.iter();
        let x = it.next().unwrap();
        let y = it.next().unwrap();

        assert_eq!(*x, 0xb9);
        assert_eq!(*y, 0x4d);
    }

    #[test]
    fn return_iterator_over_array() {
        fn iter(a: &[u8; 4]) -> std::slice::Iter<'_, u8> {
            let i = a.iter();
            i
        }

        let a = [0u8, 1u8, 2u8, 3u8];
        let mut it = iter(&a);
        assert_eq!(*it.next().unwrap(), 0);
        assert_eq!(*it.next().unwrap(), 1);
    }





}