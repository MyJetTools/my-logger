# Consumers

A consumer is a destination of the records. Every record written after a consumer is plugged is handed to it; any number of consumers can be plugged. The [Seq logger](seq-logger.md) is a consumer.

```rust
use std::sync::Arc;

use my_logger::{MyLogEvent, MyLoggerConsumer};

pub struct MyConsumer;

impl MyLoggerConsumer for MyConsumer {
    fn write_log(&self, log_event: Arc<MyLogEvent>) {
        // put the record into a queue of your own
    }
}

my_logger::LOGGER.plug_consumer(Arc::new(MyConsumer));
```

`write_log` is called on the thread which writes the record, before the write returns. It must not block and has no Tokio runtime to rely on: put the record into a queue and do the work in the background.

## MyLogEvent

| Field | |
|---|---|
| `dt` | When the record is written. |
| `level` | `Info`, `Warning`, `Error`, `FatalError` or `Debug`. |
| `process` | The process of the record. |
| `message` | The message of the record. |
| `context` | The keys and values of the record: `None` when there are none. |

The [parameters of every record](populated-params.md) are not a part of the event: a consumer takes them with `my_logger::LOGGER.get_populated_params()`.
