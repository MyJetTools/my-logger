use std::{collections::HashMap, sync::Arc};

use flurl::{FlUrl, FlUrlError};
use my_json::json_writer::JsonObjectWriter;
use my_logger_core::MyLogEvent;
use rust_extensions::StrOrString;

const CLEF_PARAM: Option<StrOrString<'_>> = None;

pub async fn push_logs_data(
    url: String,
    api_key: Option<&String>,
    fields_to_populate: Option<&HashMap<String, String>>,
    data: Vec<Arc<MyLogEvent>>,
) -> Result<(), FlUrlError> {
    let body = compile_body(fields_to_populate, data);

    #[cfg(feature = "debug-http")]
    println!("Sending log: {}", std::str::from_utf8(&body).unwrap());

    let mut fl_url = FlUrl::new(url)
        .append_path_segment("api")
        .append_path_segment("events")
        .append_path_segment("raw");

    if let Some(api_key) = api_key {
        fl_url = fl_url.with_header("X-Seq-ApiKey", api_key);
    };

    let mut result = fl_url
        .append_query_param("clef", CLEF_PARAM)
        .post(Some(body))
        .await?;

    if std::env::var("SEQ_DEBUG").is_ok() {
        println!("Result: {}", result.get_status_code());
        let body = result.get_body_as_slice().await?;
        println!("Body: {}", std::str::from_utf8(&body).unwrap());
    }

    Ok(())
}

fn compile_body(
    fields_to_populate: Option<&HashMap<String, String>>,
    data: Vec<Arc<MyLogEvent>>,
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

        json_writer.write_value("@l", level_as_str);
        json_writer.write_value("@t", log_data.dt.to_rfc3339().as_str());
        json_writer.write_value("Process", log_data.process.as_str());
        json_writer.write_value("@m", &log_data.message);

        if let Some(fields_to_populate) = fields_to_populate {
            if let Some(ex) = fields_to_populate.get("Location") {
                json_writer.write_value("@x", ex);
            }

            for (key, value) in fields_to_populate {
                json_writer.write_value(key, value);
            }
        }

        if let Some(ctx) = &log_data.context {
            for (key, value) in ctx {
                match get_context_type(value.as_str()) {
                    ContextType::String => {
                        json_writer.write_value(key, format_value(value));
                    }
                    ContextType::Raw => {
                        json_writer.write_raw_value(key, value.as_bytes());
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

fn format_value(src: &str) -> String {
    let mut result = String::with_capacity(src.len());

    for b in src.as_bytes() {
        if *b >= 32 {
            result.push(*b as char);
        }
    }

    result
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

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use my_logger_core::MyLogEvent;
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::compile_body;

    #[test]
    fn test() {
        let mut ctx = HashMap::new();

        ctx.insert("HostPort".to_string(), "10.0.0.3:5125".to_string());

        let log_event = MyLogEvent {
            dt: DateTimeAsMicroseconds::now(),
            level: my_logger_core::LogLevel::Error,
            process: "Process".to_string(),
            message: "Process".to_string(),
            context: Some(ctx),
        };
        let body = compile_body(None, vec![Arc::new(log_event)]);

        println!("{}", std::str::from_utf8(&body).unwrap());
    }
}
