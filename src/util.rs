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

pub fn generate_password(length_range: Option<Range<usize>>) -> String {
    const DEFAULT_SIZE: usize = 16;

    let length = if let Some(range) = length_range {
        rand::random_range(range)
    } else {
        DEFAULT_SIZE
    };

    let mut result = String::new();
    for _ in 0..length {
        result.push(
            PASSWORD_CHARS[random_range(0..PASSWORD_CHARS.len())]
        );
    };

    result
}