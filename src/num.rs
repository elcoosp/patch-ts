//! Saturating numeric conversion helpers.
//!
//! Clippy's pedantic cast lints (rightly) flag `as` casts that may truncate,
//! wrap or lose precision. Every conversion in this crate goes through these
//! helpers instead, which either convert losslessly or saturate at the type
//! boundary. For this crate's domain (line numbers, byte offsets, column
//! indices) the saturation branch is unreachable in practice, but it keeps the
//! semantics explicit instead of implicit in a raw cast.

/// Convert a `usize` count to `f64` for ratio/score computations.
/// Values beyond `u32::MAX` saturate (lossless conversion below that).
#[must_use]
pub fn to_f64(n: usize) -> f64 {
    f64::from(u32::try_from(n).unwrap_or(u32::MAX))
}

/// Narrow a `usize` (byte offsets, tree-sitter points) to `u32`, saturating.
#[must_use]
pub fn to_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// Widen a `u64` to `usize`, saturating at `usize::MAX` on 32-bit targets.
#[must_use]
pub fn usize_from_u64(n: u64) -> usize {
    usize::try_from(n).unwrap_or(usize::MAX)
}

/// Convert a `usize` to `isize`, saturating at `isize::MAX`.
#[must_use]
pub fn to_isize(n: usize) -> isize {
    isize::try_from(n).unwrap_or(isize::MAX)
}

/// Convert an `isize` to `usize`, saturating negatives to 0.
#[must_use]
pub fn to_usize(n: isize) -> usize {
    usize::try_from(n).unwrap_or(0)
}

/// Convert an `i64` to `usize`, saturating negatives to 0.
#[must_use]
pub fn usize_from_i64(n: i64) -> usize {
    usize::try_from(n).unwrap_or(0)
}

/// Convert a `u64` to `u32`, saturating.
#[must_use]
pub fn to_u32_from_u64(n: u64) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// Signed difference of two `usize` values, saturating at `isize` bounds.
#[must_use]
pub fn isize_diff(a: usize, b: usize) -> isize {
    to_isize(a).saturating_sub(to_isize(b))
}

/// Convert a `usize` to `u8`, saturating at `u8::MAX`.
#[must_use]
pub fn to_u8(n: usize) -> u8 {
    u8::try_from(n).unwrap_or(u8::MAX)
}

/// Round a 0..=1 ratio to a 0..=100 percent, clamping out-of-range input.
///
/// Avoids a raw `f64 as u8` cast (clippy rightly flags those as truncating):
/// after rounding and clamping the value is an integer by construction, so it
/// is round-tripped through its decimal representation instead.
#[must_use]
pub fn percent(x: f64) -> u8 {
    if !x.is_finite() {
        return 0;
    }
    let scaled = (x * 100.0).round().clamp(0.0, 100.0);
    format!("{scaled}").parse::<u8>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u8_saturates() {
        assert_eq!(to_u8(99), 99);
        assert_eq!(to_u8(usize::MAX), u8::MAX);
    }

    #[test]
    fn test_percent_rounds_and_clamps() {
        assert_eq!(percent(1.0), 100);
        assert_eq!(percent(0.0), 0);
        assert_eq!(percent(0.123_4), 12);
        assert_eq!(percent(0.999), 100);
        assert_eq!(percent(-5.0), 0);
        assert_eq!(percent(42.0), 100);
        assert_eq!(percent(f64::NAN), 0);
    }

    #[test]
    fn test_isize_diff() {
        assert_eq!(isize_diff(10, 4), 6);
        assert_eq!(isize_diff(4, 10), -6);
    }

    #[test]
    fn test_to_f64_lossless_small() {
        assert_eq!(to_f64(42), 42.0);
        assert_eq!(to_f64(0), 0.0);
    }

    #[test]
    fn test_to_f64_saturates_beyond_u32() {
        assert_eq!(to_f64(usize::MAX), f64::from(u32::MAX));
    }

    #[test]
    fn test_to_u32_saturates() {
        assert_eq!(to_u32(1_000), 1_000);
        assert_eq!(to_u32(usize::MAX), u32::MAX);
    }

    #[test]
    fn test_usize_from_u64_saturates() {
        assert_eq!(usize_from_u64(7), 7);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(usize_from_u64(u64::MAX), usize::MAX);
    }

    #[test]
    fn test_to_isize_and_back() {
        assert_eq!(to_isize(9), 9);
        assert_eq!(to_isize(usize::MAX), isize::MAX);
        assert_eq!(to_usize(-3), 0);
        assert_eq!(to_usize(11), 11);
    }

    #[test]
    fn test_usize_from_i64_saturates_negatives() {
        assert_eq!(usize_from_i64(-1), 0);
        assert_eq!(usize_from_i64(500), 500);
    }

    #[test]
    fn test_to_u32_from_u64_saturates() {
        assert_eq!(to_u32_from_u64(u64::MAX), u32::MAX);
        assert_eq!(to_u32_from_u64(3), 3);
    }
}
