use my_json::json_writer::{JsonObjectWriter, RawJsonObject};
use my_logger_core::{MyLogEvent, PopulatedParams};

pub enum ContextType {
    String,
    Raw,
}

impl ContextType {
    pub fn new(src: &str) -> Self {
        if my_json::json_utils::is_null(src.as_bytes()) {
            return ContextType::Raw;
        }

        let number = my_json::json_utils::is_number(src.as_bytes());

        if number.is_double() || number.is_number() {
            return ContextType::Raw;
        }

        if my_json::json_utils::is_bool(src.as_bytes()) {
            return ContextType::Raw;
        }

        ContextType::String
    }
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

    json_writer = json_writer
        .write("@l", level_as_str)
        .write("@t", &log_event.dt.to_rfc3339()[..26])
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
