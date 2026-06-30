use std::ops::Range;

use rand::random_range;

const PASSWORD_CHARS: [char; 94] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
    ':', ';', '<', '=', '>', '?', '@',
    '[', '\\', ']', '^', '_', '`',
    '{', '|', '}', '~',
];

pub fn generate_password(length: Option<usize>) -> String {
    const DEFAULT_SIZE: Range<usize> = 16..25;

    let length = length.unwrap_or_else(|| rand::random_range(DEFAULT_SIZE));

    let mut result = String::new();
    for _ in 0..length {
        result.push(
            PASSWORD_CHARS[random_range(0..PASSWORD_CHARS.len())]
        );
    };

    result
}