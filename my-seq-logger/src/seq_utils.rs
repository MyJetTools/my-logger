use rust_extensions::{MaybeShortString, StrOrString};

pub fn format_seq_string<'s>(src: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
    let src: StrOrString<'s> = src.into();
    let mut has_esc_symbol = false;

    for c in src.as_str().chars() {
        let as_u8 = c as u8;
        if as_u8 < 32 {
            has_esc_symbol = true;
            break;
        }
    }

    if !has_esc_symbol {
        return src;
    }

    let mut result = MaybeShortString::new();

    for c in src.as_str().chars() {
        let as_u8 = c as u8;

        if as_u8 < 32 {
            if c == '\n' {
                result.push_str("\\n");
            } else if c == '\r' {
                result.push_str("\\r");
            }
        } else {
            result.push(c);
        }
    }

    result.into()
}

pub fn format_value<'s>(src: &'s str) -> StrOrString<'s> {
    let mut result = MaybeShortString::new();

    for b in src.chars() {
        if b as u8 >= 32 {
            result.push(b);
        }
    }

    result.into()
}
