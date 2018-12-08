// Cannot put this here - it goes in main.rs
// #[macro_use] extern crate hex_literal;

mod tests {
    #[test]
    fn sha256_example() {
        use sha2::{Sha256, Sha512, Digest};

        let mut hasher = Sha256::new();
        hasher.input(b"hello world");
        let result = hasher.result();
        let expected:[u8;32] = hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

        assert_eq!(result[..], expected[..]);

        let mut hasher = Sha512::new();
        hasher.input(b"hello world");
        let result = hasher.result();

        assert_eq!(result[..],
                   hex!("309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f")[..]);
    }


    #[test]
    fn next_16_bits_of_hash() {
        fn next16(remainder: &[u8]) -> (u16, &[u8]){
            let low = remainder[0] as u16;
            let hi = remainder[1] as u16;
            (low | hi << 8, &remainder[2..])
        }

        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.input(b"hello world");
        let result = hasher.result();
        let expected:[u8;32] = hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

        assert_eq!(result[..], expected[..]);

        let (one, remainder) = next16(&result[..]);
        assert_eq!(0x4db9, one);
        let (two, remainder) = next16(remainder);
        assert_eq!(0xb927, two);
        let (three, _) = next16(remainder);
        assert_eq!(0x4d93, three);

    }



}