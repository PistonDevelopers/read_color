#![deny(missing_docs)]

//! A simple library for reading hex colors.

use std::str::Chars;

/// Converts a character into a u8 value.
pub fn char_to_hex(ch: char) -> Option<u8> {
    Some(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'C' | 'c' => 12,
        'D' | 'd' => 13,
        'E' | 'e' => 14,
        'F' | 'f' => 15,
        _ => { return None; }
    })
}

/// Reads a hex value from an iterator of characters.
pub fn hex(chars: &mut Chars) -> Option<u8> {
    match chars.next() {
        None => { return None; }
        Some(ch) => char_to_hex(ch)
    }
}

/// Reads a pair of hex values, joining them in value range 0-255.
pub fn hex_pair(chars: &mut Chars) -> Option<u8> {
    let h1 = match hex(chars) {
        None => { return None; }
        Some(h1) => h1
    };
    let h2 = match hex(chars) {
        None => { return None; }
        Some(h2) => h2
    };
    Some((h1 << 4) | h2)
}

/// Reads RGB colors from iterator of characters.
pub fn rgb(chars: &mut Chars) -> Option<[u8; 3]> {
    let red = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    let green = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    let blue = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    Some([red, green, blue])
}

/// Reads RGBA colors from iterator of characters.
pub fn rgba(chars: &mut Chars) -> Option<[u8; 4]> {
    let red = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    let green = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    let blue = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    let alpha = match hex_pair(chars) {
        None => { return None; }
        Some(x) => x
    };
    Some([red, green, blue, alpha])
}

/// Reads RGB with optional alpha from iterator of characters.
pub fn rgb_maybe_a(chars: &mut Chars) -> Option<([u8; 3], Option<u8>)> {
    let rgb = match rgb(chars) {
        None => { return None; }
        Some(x) => x
    };
    let a = match hex_pair(chars) {
        None => { return Some((rgb, None)); }
        Some(x) => x
    };
    Some((rgb, Some(a)))
}

