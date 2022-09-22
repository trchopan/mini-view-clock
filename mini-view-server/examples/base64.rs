use rand::{self, prelude::Distribution, Rng};

struct MyBase64;

impl Distribution<u8> for MyBase64 {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 12;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789_-";
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

fn main() {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&MyBase64)
        .take(16)
        .map(char::from)
        .collect();
    println!("{}", rand_string);

    assert!('٣'.is_alphanumeric());
    assert!('7'.is_alphanumeric());
    assert!('৬'.is_alphanumeric());
    assert!('¾'.is_alphanumeric());
    assert!('①'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('و'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
}
