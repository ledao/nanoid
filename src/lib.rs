extern crate core;

use rand::{RngCore};
use rand::thread_rng;

const DEFAULT_ALPHABET: &[u8] = "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz".as_bytes();
const DEFAULT_SIZE: usize = 21;

const LEN8TAB: &[u8] = "\x00\x01\x02\x02\x03\x03\x03\x03\x04\x04\x04\x04\x04\x04\x04\x04\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes();

fn len_32(mut x: u32) -> i32 {
    let mut n = 0;
    if x >= 1 << 16 {
        x >>= 16;
        n = 16;
    }
    if x >= 1 << 8 {
        x >>= 8;
        n += 8;
    }
    return n + LEN8TAB[x as usize] as i32;
}

fn leading_zeros_32(x: u32) -> i32 {
    return 32 - len_32(x);
}


type BytesGenerator = fn(size: usize) -> Vec<u8>;

fn generate_random_buffer(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    buffer.resize(size, 0);
    rng.fill_bytes(buffer.as_mut_slice());
    return buffer;
}

fn format_string(generate_random_buffer: BytesGenerator, alphabet: &[u8], size: usize) -> String {
    let mask = (2 << (31 - leading_zeros_32((alphabet.len() - 1 | 1) as u32)) as u32) - 1;
    let step = ((1.6 * (mask * size) as f64 / (alphabet.len()) as f64).ceil()) as usize;
    let mut id = String::new();
    loop {
        let random_buffer = generate_random_buffer(step);
        for i in 0..step {
            let current_index = (random_buffer[i] & mask as u8) as usize;
            if current_index < alphabet.len() {
                id.push(char::from(alphabet[current_index]));
                if id.len() == size as usize {
                    return id;
                }
            }
        }
    }
}


pub fn generate_string(alphabet: &[u8], size: usize) -> String {
    return format_string(generate_random_buffer, alphabet, size);
}

pub fn new() -> String {
    return generate_string(DEFAULT_ALPHABET, DEFAULT_SIZE);
}

#[macro_export]
macro_rules! idnano {
    () => {
        new()
    };
    ($size: expr) => {
        generate_string(DEFAULT_ALPHABET, $size)
    };
    ($size: expr, $alphabet: expr) => {
        generate_string($alphabet, $size)
    };

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{DEFAULT_ALPHABET, DEFAULT_SIZE, format_string, generate_random_buffer, generate_string, LEN8TAB, len_32, new};

    #[test]
    fn test_len_32() {
        assert_eq!(0, len_32(0));
        assert_eq!(3, len_32(4));
        assert_eq!(4, len_32(8));
        assert_eq!(5, len_32(21));
    }

    #[test]
    fn default_alphabet() {
        assert_eq!("-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz".as_bytes(), DEFAULT_ALPHABET);
    }

    #[test]
    fn default_size_is_21() {
        assert_eq!(21, DEFAULT_SIZE);
    }

    #[test]
    fn test_generate_url_friendly_ids() {
        let alphabet = DEFAULT_ALPHABET;
        let size = DEFAULT_SIZE;
        for _i in 0..100 {
            let id = generate_string(alphabet, size);
            for ch in id.bytes() {
                if !alphabet.contains(&ch) {
                    assert_eq!("contains url not frinedly chars", "");
                }
            }
        }
    }

    #[test]
    fn test_change_id_length() {
        let alphabet = DEFAULT_ALPHABET;
        let size = 10;
        let id = generate_string(alphabet, size);
        assert_eq!(size, id.len());
    }

    #[test]
    fn test_no_collisions() {
        let count = 100 * 1000;
        let mut used: HashMap<String, usize> = HashMap::new();
        for _i in 0..count {
            let id = new();
            if used.contains_key(&id) {
                assert_eq!("repeated id has been generated", "");
            }
            used.insert(id, 1);
        }
    }

    #[test]
    fn test_has_flat_distribution() {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".as_bytes();
        let size = 5;

        let count = 100 * 1000;
        let mut chars: HashMap<u8, i32> = HashMap::new();
        for _i in 0..count {
            let id = generate_string(alphabet, size);
            let id_bytes = id.as_bytes();
            for j in 0..id_bytes.len() {
                let ch = id_bytes[j];
                if !chars.contains_key(&ch) {
                    chars.insert(ch, 0);
                } else {
                    *chars.get_mut(&ch).unwrap() += 1;
                }
            }
        }

        assert_eq!(chars.len(), alphabet.len());

        let mut max = 0.0;
        let mut min = i32::MAX as f64;
        for v in chars {
            let distribution = (v.1 * alphabet.len() as i32) as f64 / (count * size) as f64;
            if distribution > max {
                max = distribution;
            }
            if distribution < min {
                min = distribution;
            }
        }
        let distribution = max - min;
        assert_eq!(true, distribution < 0.05);
    }

    #[test]
    fn test_has_options() {
        let id = generate_string("a".as_bytes(), 5);
        let target = "aaaaa".to_string();
        assert_eq!(target, id);
    }

    const SEQUENCE: [u8; 10] = [2, 255, 3, 7, 7, 7, 7, 7, 0, 1];

    fn generate_bytes_buffer(step: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut i = 0;
        loop {
            if i < step {
                for j in 0..step {
                    buffer.push(SEQUENCE[j]);
                }
            } else {
                break;
            }
            i += SEQUENCE.len();
        }
        return buffer;
    }

    #[test]
    fn test_generate_random_string() {
        let id = format_string(generate_bytes_buffer, "abcde".as_bytes(), 4);
        let target = "cdac".to_string();
        assert_eq!(target, id);
    }

    #[test]
    fn len8tab_is_256_len() {
        assert_eq!(256, LEN8TAB.len());
        assert_eq!(0, LEN8TAB[0] as i32);
        assert_eq!(8, LEN8TAB[255] as i32);
    }

    #[test]
    fn gen_random_buffer_len() {
        let buf = generate_random_buffer(DEFAULT_SIZE);
        assert_eq!(DEFAULT_SIZE, buf.len());
    }

    #[test]
    fn generate_id() {
        let _id = new();
    }

    #[test]
    fn test_macros() {
        //default id
        let id = idnano!();
        assert_eq!(id.len(), DEFAULT_SIZE);

        //set size
        let id_10 = idnano!(10);
        assert_eq!(id_10.len(), 10);

        //set size and alphabet
        let id_alphabet_10 = idnano!(10, "01234567890".as_bytes());
        assert_eq!(id_alphabet_10.len(), 10);
    }
}
