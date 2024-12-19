use std::{sync::Arc, time::Duration};

use flurl::{FlUrl, FlUrlResponse};
use my_logger_core::{MyLogEvent, PopulatedParams};

const NULL_PARAM: Option<&str> = None;

const MAX_CHUNK_SIZE: usize = 1024 * 1024 * 2;

pub async fn upload_log_events_chunk(
    url: &str,
    api_key: Option<String>,
    populated_params: PopulatedParams,
    data: Vec<Arc<MyLogEvent>>,
    seq_debug: bool,
) {
    let mut chunk_to_upload = Vec::new();
    let mut payload_buffer = Vec::new();

    for log_event in data.iter() {
        payload_buffer = super::serialize(payload_buffer, log_event, &populated_params);

        let merge_slice = chunk_to_upload.len() == 0
            || chunk_to_upload.len() + payload_buffer.len() < MAX_CHUNK_SIZE;

        if merge_slice {
            if chunk_to_upload.len() > 0 {
                chunk_to_upload.push(13);
                chunk_to_upload.push(10);
            }
            chunk_to_upload.extend_from_slice(payload_buffer.as_slice());
        } else {
            upload_current_chunk(url, chunk_to_upload.as_slice(), api_key.as_ref(), seq_debug)
                .await;
            chunk_to_upload.clear();
            chunk_to_upload.extend_from_slice(payload_buffer.as_slice());
        }
    }
}

async fn upload_current_chunk(
    url: &str,
    chunk_to_upload: &[u8],
    api_key: Option<&String>,
    seq_debug: bool,
) {
    let mut attempt_no = 0;
    loop {
        attempt_no += 1;
        if seq_debug {
            println!(
                "Sending log: {}",
                std::str::from_utf8(&chunk_to_upload).unwrap()
            );
        }

        let mut fl_url = FlUrl::new(url)
            .set_timeout(Duration::from_secs(10))
            .append_path_segment("api")
            .append_path_segment("events")
            .append_path_segment("raw")
            .with_header("Accept", "*/*")
            .with_header("Content-Type", "application/vnd.serilog.clef");

        if let Some(api_key) = api_key {
            fl_url = fl_url.with_header("X-Seq-ApiKey", api_key);
        };

        let response = fl_url
            .append_query_param("clef", NULL_PARAM)
            .post(Some(chunk_to_upload.to_vec()))
            .await;

        match response {
            Ok(mut response) => {
                if seq_debug {
                    print_fl_url_response(&mut response).await;
                }

                if is_status_code_ok(response.get_status_code()) {
                    return;
                }
            }
            Err(err) => {
                println!(
                    "Attempt: {} Error while uploading logs to seq. Err: {:?}",
                    attempt_no, err
                );

                if attempt_no > 10 {
                    return;
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

fn is_status_code_ok(status_code: u16) -> bool {
    status_code >= 200 && status_code < 210
}

async fn print_fl_url_response(response: &mut FlUrlResponse) {
    println!("Result: {}", response.get_status_code());
    let body = response.get_body_as_slice().await;
    let body = match body {
        Ok(body) => body,

        Err(err) => {
            println!("Error while reading response body. Err: {:?}", err);
            return;
        }
    };

    match std::str::from_utf8(&body) {
        Ok(body_as_str) => {
            println!("Body: {}", body_as_str);
        }
        Err(_) => {
            println!("Body is not a valid utf8 string. Len: {}", body.len());
        }
    }
}

/*
fn compile_body(populated_params: &PopulatedParams, data: &[Arc<MyLogEvent>]) -> Vec<u8> {
    const LOCATION_KEY: &'static str = "Location";
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
            crate::seq_utils::format_seq_string(log_data.process.as_str()).as_str(),
        );
        json_writer.write(
            "@m",
            crate::seq_utils::format_seq_string(log_data.message.as_str()).as_str(),
        );

        if let Some(ex) = populated_params.get(LOCATION_KEY) {
            json_writer.write("@x", crate::seq_utils::format_seq_string(ex).as_str());
        }

        for (key, value) in populated_params.iter() {
            if key != LOCATION_KEY {
                json_writer.write(key, crate::seq_utils::format_seq_string(value).as_str());
            }
        }

        if let Some(ctx) = &log_data.context {
            for (key, value) in ctx {
                match get_context_type(value.as_str()) {
                    ContextType::String => {
                        json_writer.write(key, format_value(value).as_str());
                    }
                    ContextType::Raw => {
                        let raw_value = RawJsonObject::AsStr(value);
                        json_writer.write(key, raw_value);
                    }
                }
            }
        }

        if result.len() > 0 {
            result.push(13);
            result.push(10);
        }

        result.extend_from_slice(json_writer.build().as_slice());
    }

    result
}
 */
/*
#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use my_logger_core::{MyLogEvent, PopulatedParams};
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

        upload_logs_chunk(
            "http://192.168.1.67:5345",
            None,
            &PopulatedParams::new_empty(),
            &vec![Arc::new(log_event)],
            true,
        )
        .await
        .unwrap();
    }
}
 */
