use rust_extensions::{MaybeShortString, StrOrString};

pub fn format_seq_string<'s>(src: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
    let src: StrOrString<'s> = src.into();
    let s = src.as_str();

    // ASCII control bytes (<32) never appear inside a UTF-8 continuation,
    // so a byte index found this way is also a valid char boundary.
    let first = match s.as_bytes().iter().position(|&b| b < 32) {
        Some(idx) => idx,
        None => return src,
    };

    let mut result = MaybeShortString::new();
    result.push_str(&s[..first]);

    for c in s[first..].chars() {
        if (c as u32) < 32 {
            match c {
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                _ => {}
            }
        } else {
            result.push(c);
        }
    }

    result.into()
}

pub fn format_value<'s>(src: &'s str) -> StrOrString<'s> {
    let first = match src.as_bytes().iter().position(|&b| b < 32) {
        Some(idx) => idx,
        None => return src.into(),
    };

    let mut result = MaybeShortString::new();
    result.push_str(&src[..first]);

    for c in src[first..].chars() {
        if (c as u32) >= 32 {
            result.push(c);
        }
    }

    result.into()
}
