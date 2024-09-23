mod alphabet;

use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::alphabet::DEFAULT_ALPHABET;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
struct TempoId {
    inner: String
}

impl TempoId {
    fn generate() -> Self {
        TempoId {
            inner: tempo_id(None),
        }
    }

    fn generate_with_alphabet(alphabet: &'static str) -> Self {
        TempoId {
            inner: tempo_id(Some(TempoIdOptions {
                alphabet,
                ..Default::default()
            })),
        }
    }

    fn generate_custom(options: TempoIdOptions) -> Self {
        TempoId {
            inner: tempo_id(Some(options)),
        }
    }

    fn parse(id: &str) -> Self {
        TempoId {
            inner: id.to_owned(),
        }
    }

    fn to_string(&self) -> String {
        self.inner.to_owned()
    }
}

impl std::fmt::Display for TempoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Clone)]
pub struct TempoIdOptions {
    pub time_length: usize,
    pub random_length: usize,
    pub time: Option<u64>,
    pub start_time: Option<u64>,
    pub pad_left: bool,
    pub alphabet: &'static str,
}

impl Default for TempoIdOptions {
    fn default() -> Self {
        TempoIdOptions {
            time_length: 8,
            random_length: 13,
            time: None,
            start_time: None,
            pad_left: true,
            alphabet: DEFAULT_ALPHABET,
        }
    }
}

fn tempo_id(options: Option<TempoIdOptions>) -> String {
    let options = options.unwrap_or_default();

    let random_part = generate_random_string(options.random_length, &options.alphabet);

    let time_part = generate_time(
        options.time_length,
        options.time,
        options.start_time,
        options.pad_left,
        &options.alphabet,
    );

    format!("{}{}", time_part, random_part)
}

fn generate_random_string(length: usize, alphabet: &str) -> String {
    let characters = alphabet;
    let alphabet_size = characters.len();
    let mut random_string = String::with_capacity(length);

    let exponent = ((alphabet_size - 1) as f64).log2().floor() as usize;
    let mask = (1 << (exponent + 1)) - 1;

    let mut random_bytes: Vec<u8> = Vec::new();
    let mut random_index: usize = 0;

    for _ in 0..length {
        let mut character_index: usize;
        loop {
            if random_bytes.is_empty() || random_index >= random_bytes.len() {
                random_bytes = get_random_bytes(length * 2);
                random_index = 0;
            }
            character_index = (random_bytes[random_index] as usize) & mask;
            random_index += 1;

            if character_index < alphabet_size {
                break;
            }
        }
        random_string.push(characters.chars().nth(character_index).unwrap());
    }

    random_string
}

const POOL_SIZE_MULTIPLIER: usize = 64;

static POOL_CURSOR: LazyLock<Mutex<usize>> =
    LazyLock::new(|| Mutex::new(0));

static POOL: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn get_random_bytes(size: usize) -> Vec<u8> {
    let mut pool = POOL.lock().unwrap();
    let mut pool_cursor = POOL_CURSOR.lock().unwrap();

    if pool.is_empty() || pool.len() < size {
        *pool = vec![0u8; size * POOL_SIZE_MULTIPLIER];
        OsRng.fill_bytes(&mut pool[..]);
        *pool_cursor = 0;
    } else if *pool_cursor + size > pool.len() {
        OsRng.fill_bytes(&mut pool[..]);
        *pool_cursor = 0;
    }

    let bytes = pool[*pool_cursor..*pool_cursor + size].to_vec();
    *pool_cursor += size;

    bytes
}

fn generate_time(
    time_length: usize,
    time: Option<u64>,
    start_time: Option<u64>,
    pad_left: bool,
    alphabet: &str,
) -> String {
    if time_length == 0 {
        return "".to_string();
    }

    let mut time = if let Some(t) = time {
        t
    } else {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

        if let Some(start) = start_time {
            since_epoch - start
        } else {
            since_epoch
        }
    };

    let max_value = get_max_value_of_fixed_length(time_length, alphabet);
    time = time % (max_value + 1);

    let millis_encoded = encode_number(time, alphabet);

    if pad_left {
        let padding_char = alphabet.chars().nth(0).unwrap();
        let padding = std::iter::repeat(padding_char)
            .take(time_length.saturating_sub(millis_encoded.len()))
            .collect::<String>();
        format!("{}{}", padding, millis_encoded)
    } else {
        millis_encoded
    }
}

fn get_max_value_of_fixed_length(length: usize, alphabet: &str) -> u64 {
    let base = alphabet.len() as u64;
    base.pow(length as u32) - 1
}

fn encode_number(mut number: u64, alphabet: &str) -> String {
    let base = alphabet.len() as u64;
    if number == 0 {
        return alphabet.chars().nth(0).unwrap().to_string();
    }

    let mut encoded = String::new();
    while number > 0 {
        let remainder = (number % base) as usize;
        encoded.insert(0, alphabet.chars().nth(remainder).unwrap());
        number /= base;
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tempo_id() {
        let id = TempoId::generate();
        println!("Generated TempoId: {}", id);
        assert_eq!(id.inner.len(), 8 + 13);
    }

    #[test]
    fn test_with_alphabet() {
        let id = TempoId::generate_with_alphabet(alphabet::UPPERCASE);
        println!("Generated TempoId with uppercase characters: {}", id);
        assert!(id.inner.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_custom_options() {
        let options = TempoIdOptions {
            time_length: 10,
            random_length: 15,
            time: None,
            start_time: None,
            pad_left: true,
            alphabet: alphabet::NUMBERS,
        };
        let id = TempoId::generate_custom(options);
        println!("Generated TempoId with custom options: {}", id);
        assert_eq!(id.inner.len(), 10 + 15);
        assert!(id.inner.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_no_pad_time() {
        let options = TempoIdOptions {
            time_length: 16,
            random_length: 1,
            time: Some(0),
            start_time: None,
            pad_left: false,
            alphabet: alphabet::NUMBERS,
        };
        let id = TempoId::generate_custom(options);
        println!("Generated TempoId with no padding: {}", id);
        assert_eq!(id.inner.len(), 2);
        assert!(id.inner.chars().all(|c| c.is_numeric()));
    }
}
