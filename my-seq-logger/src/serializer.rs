use my_json::json_writer::{JsonObjectWriter, RawJsonObject};
use my_logger_core::{MyLogEvent, PopulatedParams};

pub enum ContextType {
    String,
    Raw,
}

impl ContextType {
    pub fn new(src: &str) -> Self {
        if is_json_literal(src) || is_json_number(src.as_bytes()) {
            return ContextType::Raw;
        }

        ContextType::String
    }
}

// JSON literals are case-sensitive.
fn is_json_literal(src: &str) -> bool {
    matches!(src, "null" | "true" | "false")
}

// Strict JSON number grammar: `-? (0 | [1-9][0-9]*) (. [0-9]+)? ([eE] [+-]? [0-9]+)?`.
fn is_json_number(src: &[u8]) -> bool {
    let mut pos = 0;

    if src.first() == Some(&b'-') {
        pos += 1;
    }

    match src.get(pos) {
        Some(b'0') => pos += 1,
        Some(b'1'..=b'9') => pos += count_digits(&src[pos..]),
        _ => return false,
    }

    if src.get(pos) == Some(&b'.') {
        pos += 1;
        let digits = count_digits(&src[pos..]);
        if digits == 0 {
            return false;
        }
        pos += digits;
    }

    if matches!(src.get(pos), Some(b'e' | b'E')) {
        pos += 1;
        if matches!(src.get(pos), Some(b'+' | b'-')) {
            pos += 1;
        }
        let digits = count_digits(&src[pos..]);
        if digits == 0 {
            return false;
        }
        pos += digits;
    }

    pos == src.len()
}

fn count_digits(src: &[u8]) -> usize {
    src.iter().take_while(|c| c.is_ascii_digit()).count()
}

pub fn serialize(
    mut compile_buffer: String,
    log_event: &MyLogEvent,
    populated_params: &PopulatedParams,
) -> String {
    const LOCATION_KEY: &'static str = "Location";

    compile_buffer.clear();
    let mut json_writer = JsonObjectWriter::from_string(compile_buffer);

    let level_as_str = match &log_event.level {
        my_logger_core::LogLevel::Info => "Info",
        my_logger_core::LogLevel::Warning => "Warning",
        my_logger_core::LogLevel::Error => "Error",
        my_logger_core::LogLevel::FatalError => "Fatal",
        my_logger_core::LogLevel::Debug => "Debug",
    };

    let timestamp = log_event.dt.to_rfc3339();
    let timestamp = timestamp.get(..26).unwrap_or(timestamp.as_str());

    json_writer = json_writer
        .write("@l", level_as_str)
        .write("@t", timestamp)
        .write(
            "Process",
            crate::seq_utils::format_seq_string(log_event.process.as_str()).as_str(),
        )
        .write(
            "@m",
            crate::seq_utils::format_seq_string(log_event.message.as_str()).as_str(),
        );

    for (key, value) in populated_params.iter() {
        if key == LOCATION_KEY {
            json_writer =
                json_writer.write("@x", crate::seq_utils::format_seq_string(value).as_str());
        } else {
            json_writer =
                json_writer.write(key, crate::seq_utils::format_seq_string(value).as_str());
        }
    }

    if let Some(ctx) = &log_event.context {
        for (key, value) in ctx {
            match ContextType::new(value.as_str()) {
                ContextType::String => {
                    json_writer =
                        json_writer.write(key, crate::seq_utils::format_value(value).as_str());
                }
                ContextType::Raw => {
                    let raw_value = RawJsonObject::AsStr(value);
                    json_writer = json_writer.write(key, raw_value);
                }
            }
        }
    }

    json_writer.build()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use my_logger_core::{LogLevel, MyLogEvent, PopulatedParams};
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::{serialize, ContextType};

    fn is_raw(src: &str) -> bool {
        matches!(ContextType::new(src), ContextType::Raw)
    }

    #[test]
    fn test_valid_json_numbers_are_raw() {
        for src in ["0", "-0", "7", "1234", "-15", "0.5", "-0.25", "1e5", "1.5E-3", "2e+30"] {
            assert!(is_raw(src), "{src}");
        }
    }

    #[test]
    fn test_invalid_json_numbers_are_strings() {
        for src in [
            "0922", "00", "-01", "+1", "1.", ".5", "1e", "1e+", "-", "", "1.2.3", "1_000", " 1",
            "NaN", "Infinity",
        ] {
            assert!(!is_raw(src), "{src}");
        }
    }

    #[test]
    fn test_only_lower_case_literals_are_raw() {
        for src in ["null", "true", "false"] {
            assert!(is_raw(src), "{src}");
        }

        for src in ["NULL", "Null", "TRUE", "True", "FALSE", "False"] {
            assert!(!is_raw(src), "{src}");
        }
    }

    #[test]
    fn test_leading_zero_code_is_serialized_as_string() {
        let event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            level: LogLevel::Info,
            process: "test".to_string(),
            message: "msg".to_string(),
            context: Some(HashMap::from([
                ("code".to_string(), "0922".to_string()),
                ("amount".to_string(), "100".to_string()),
            ])),
        };

        let json = serialize(String::new(), &event, &PopulatedParams::new_empty());

        assert!(json.contains(r#""code":"0922""#), "{json}");
        assert!(json.contains(r#""amount":100"#), "{json}");
    }
}
