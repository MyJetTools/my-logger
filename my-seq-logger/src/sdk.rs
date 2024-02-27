use std::{collections::HashMap, sync::Arc};

use flurl::{FlUrl, FlUrlError};
use my_json::json_writer::{JsonObjectWriter, RawJsonObject};
use my_logger_core::MyLogEvent;
use rust_extensions::*;

const NULL_PARAM: Option<&str> = None;

pub async fn push_logs_data(
    url: &str,
    api_key: Option<&String>,
    fields_to_populate: Option<&HashMap<String, String>>,
    data: &[Arc<MyLogEvent>],
    seq_debug: bool,
) -> Result<(), FlUrlError> {
    let body = compile_body(fields_to_populate, data);

    if seq_debug {
        println!("Sending log: {}", std::str::from_utf8(&body).unwrap());
    }

    let mut fl_url = FlUrl::new(url)
        .append_path_segment("api")
        .append_path_segment("events")
        .append_path_segment("raw")
        .with_header("Accept", "*/*")
        .with_header("Content-Type", "application/vnd.serilog.clef");

    if let Some(api_key) = api_key {
        fl_url = fl_url.with_header("X-Seq-ApiKey", api_key);
    };

    let mut result = fl_url
        .append_query_param("clef", NULL_PARAM)
        .post(Some(body))
        .await?;

    if seq_debug {
        println!("Result: {}", result.get_status_code());
        let body = result.get_body_as_slice().await?;
        println!("Body: {}", std::str::from_utf8(&body).unwrap());
    }

    Ok(())
}

fn compile_body(
    fields_to_populate: Option<&HashMap<String, String>>,
    data: &[Arc<MyLogEvent>],
) -> Vec<u8> {
    let mut result = Vec::new();

    for log_data in data {
        let mut json_writer = JsonObjectWriter::new();

        let level_as_str = match &log_data.level {
            my_logger_core::LogLevel::Info => "Info",
            my_logger_core::LogLevel::Warning => "Warning",
            my_logger_core::LogLevel::Error => "Error",
            my_logger_core::LogLevel::FatalError => "Fatal",
            my_logger_core::LogLevel::Debug => "Debug",
        };

        json_writer.write("@l", level_as_str);
        json_writer.write("@t", &log_data.dt.to_rfc3339()[..26]);
        json_writer.write(
            "Process",
            format_seq_string(log_data.process.as_str()).as_str(),
        );
        json_writer.write("@m", format_seq_string(log_data.message.as_str()).as_str());

        if let Some(fields_to_populate) = fields_to_populate {
            if let Some(ex) = fields_to_populate.get("Location") {
                json_writer.write("@x", format_seq_string(ex).as_str());
            }

            for (key, value) in fields_to_populate {
                json_writer.write(key, format_seq_string(value).as_str());
            }
        }

        if let Some(ctx) = &log_data.context {
            for (key, value) in ctx {
                match get_context_type(value.as_str()) {
                    ContextType::String => {
                        json_writer.write(key, format_value(value).as_str());
                    }
                    ContextType::Raw => {
                        let raw_value = RawJsonObject::AsSlice(value.as_bytes());
                        json_writer.write(key, raw_value);
                    }
                }
            }
        }

        if result.len() > 0 {
            result.push(13);
            result.push(10);
        }

        result.extend(json_writer.build());
    }

    result
}

fn format_value<'s>(src: &'s str) -> StrOrString<'s> {
    let mut result = MaybeShortString::new();

    for b in src.chars() {
        if b as u8 >= 32 {
            result.push(b);
        }
    }

    result.into()
}

pub enum ContextType {
    String,
    Raw,
}

fn get_context_type(src: &str) -> ContextType {
    if my_json::json_utils::is_null(src.as_bytes()) {
        return ContextType::Raw;
    }

    if my_json::json_utils::is_number(src.as_bytes()) {
        return ContextType::Raw;
    }

    if my_json::json_utils::is_bool(src.as_bytes()).is_some() {
        return ContextType::Raw;
    }

    ContextType::String
}

fn format_seq_string<'s>(src: impl Into<StrOrString<'s>>) -> StrOrString<'s> {
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

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use my_logger_core::MyLogEvent;
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    #[tokio::test]
    async fn test() {
        let mut ctx = HashMap::new();

        ctx.insert("HostPort".to_string(), "10.0.0.3:5125".to_string());

        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            level: my_logger_core::LogLevel::Error,
            process: "Process".to_string(),
            message: "Process".to_string(),
            context: Some(ctx),
        };

        super::push_logs_data(
            "http://192.168.1.67:5345",
            None,
            None,
            &vec![Arc::new(log_event)],
            true,
        )
        .await
        .unwrap();
    }
}
