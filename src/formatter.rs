//! This module contains some helper functions to avoid having to call into the expensive fmt code.

pub const MAX_INT_DIGITS: usize = 11;

/// Writes ascii bytes to the buffer to represent the given int value.
///
/// Returns the slice of the buffer that was written to.
/// It can be used as a value or to determine the length of the formatting.
///
/// Panics if the buffer is less than [MAX_INT_DIGITS] long.
pub fn write_int(buffer: &mut [u8], mut value: i32) -> &mut [u8] {
    // Check in debug mode if the buffer is long enough.
    // We don't do this in release to have less overhead.
    debug_assert!(buffer.len() >= MAX_INT_DIGITS);

    let mut buffer_index = 0;
    let is_negative = value.is_negative();

    // We want a negative value because that can hold every absolute value.
    if !is_negative {
        value = -value;
    }

    // Special case for 0
    if value == 0 {
        buffer[buffer_index] = b'0';
        buffer_index += 1;
    }

    // Write the smallest digit to the buffer.
    // This will put it in there in reverse.
    while value != 0 {
        // The value is negative, so invert the smallest digit, offset it with the 0 character
        // and put it in the buffer.
        buffer[buffer_index] = b'0' + -(value % 10) as u8;
        buffer_index += 1;
        // Divide the value to get rid of the smallest digit.
        value /= 10;
    }

    if is_negative {
        // Don't forget to put the minus sign there.
        buffer[buffer_index] = b'-';
        buffer_index += 1;
    }

    // We built the buffer in reverse, so now we've got to undo that.
    buffer[0..buffer_index].reverse();

    &mut buffer[0..buffer_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_int() {
        let mut buffer = [0; 128];

        assert_eq!(write_int(&mut buffer, 0), b"0");
        assert_eq!(write_int(&mut buffer, -1), b"-1");
        assert_eq!(write_int(&mut buffer, 1), b"1");
        assert_eq!(write_int(&mut buffer, -42), b"-42");
        assert_eq!(write_int(&mut buffer, 42), b"42");
        assert_eq!(write_int(&mut buffer, -2147483648), b"-2147483648");
        assert_eq!(write_int(&mut buffer, 2147483647), b"2147483647");
    }
}
