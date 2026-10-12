# my-logger

A logger for Rust applications. Every record is printed to the console and handed to the plugged consumers. The Seq logger is such a consumer: it sends the records to [Seq](https://datalust.co/seq) or to my-logger-server.

## Plug it

```toml
[dependencies]
my-logger = { tag = "x.x.x", git = "https://github.com/MyJetTools/my-logger.git" }
```

`tag` is the latest of [the tags](https://github.com/MyJetTools/my-logger/tags).

Write a record at any place of the code:

```rust
use my_logger::LogEventCtx;

my_logger::LOGGER.write_info(
    "process_name",
    "message",
    LogEventCtx::new()
        .add("field1", "value1")
        .add("field2", "value2"),
);
```

## Plug the Seq logger

To send the records to Seq or to my-logger-server, turn the `my-seq-logger` feature on:

```toml
[dependencies]
my-logger = { tag = "x.x.x", git = "https://github.com/MyJetTools/my-logger.git", features = [
    "my-seq-logger",
] }
async-trait = "*"
tokio = { version = "*", features = ["full"] }
```

Give it the connection string and enable it once, at the start of the application:

```rust
use std::sync::Arc;

use my_logger::my_seq_logger::{SeqLogger, SeqSettings};

pub struct SettingsReader;

#[async_trait::async_trait]
impl SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        "url=http://127.0.0.1:5341;apikey=my-api-key;product=my-product".to_string()
    }
}

#[tokio::main]
async fn main() {
    my_logger::LOGGER.populate_app_and_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    SeqLogger::enable_from_connection_string(Arc::new(SettingsReader)).await;
}
```

A service built on service-sdk needs none of this: `ServiceContext::new` makes both calls itself and takes the connection string from `seq_conn_string` of the settings.

## Features

| Feature | In short |
|---|---|
| [Levels, context and macros](docs/writing-logs.md) | `write_info`, `write_warning`, `write_error`, `write_fatal_error` and `write_debug`, each with a context of keys and values; the `write_log_*!` macros. |
| [Parameters of every record](docs/populated-params.md) | `Application`, `Version`, `EnvInfo` and the ones of your own, added to each record which is sent. |
| [Console and counters](docs/console-and-counters.md) | Every record is printed to the console and counted by its level. |
| [Consumers](docs/consumers.md) | A destination of your own for the records. |
| [Seq logger](docs/seq-logger.md) | Sends the records to Seq or to my-logger-server in the background and writes a panic as a fatal error. |
| [Connection string](docs/seq-connection-string.md) | `url`, `apikey`, `product`, `timeout` and `queuesize` of the Seq logger, read again before every request. |
| [Product scope](docs/seq-connection-string.md#product-scope) | `product=my-product` names the product scope of my-logger-server the records go to. |
