# Levels, context and macros

Everything is written through the static `my_logger::LOGGER`. It needs no initialization and is used from any thread.

## Levels

| Method | Level |
|---|---|
| `write_info` | `Info` |
| `write_warning` | `Warning` |
| `write_error` | `Error` |
| `write_fatal_error` | `FatalError` |
| `write_debug` | `Debug` |

Each of them takes the process, the message and the context:

```rust
use my_logger::LogEventCtx;

my_logger::LOGGER.write_error(
    "process_name",
    format!("Can not execute order {}", order_id),
    LogEventCtx::new().add("orderId", order_id.to_string()),
);
```

- **The process** is where it happened: a `&'static str` or a `String`.
- **The message** is a `&str` or a `String`.
- **The context** is the keys and values of the record: `LogEventCtx::new()` when there are none.

`write_fatal_debug` is deprecated: `write_debug` writes the same.

## Context

```rust
let ctx = LogEventCtx::new()
    .add("orderId", "15")
    .add_object("result", &result);
```

| Method | |
|---|---|
| `add(key, value)` | The value is a `&str` or a `String`. |
| `add_object(key, &value)` | The value is anything with `Debug`, written as `{:?}`. |
| `LogEventCtx::from_object(key, &value)` | A context of one `add_object`. |

A key is a `&'static str` or a `String`. A key added twice keeps the last value. A `HashMap<String, String>` and a `BTreeMap<String, String>` turn into a context with `.into()`.

## Macros

The same in one line - the process, the message, then `"key" = value` pairs:

```rust
my_logger::write_log_info!("process_name", "message", "orderId" = order_id, "result" = result);
```

| Macro | Level |
|---|---|
| `write_log_info!` | `Info` |
| `write_log_warning!` | `Warning` |
| `write_log_err!` | `Error` |
| `write_log_fatal!` | `FatalError` |
| `write_log_debug!` | `Debug` |

- A value is anything with `Debug` and is written as `{:?}`, so a string comes with its quotes.
- With no pairs the comma after the message stays: `write_log_info!("process_name", "message",);`.
- The macros name the crate as `my_logger`. When it comes through another crate, bring it into scope under that name: `use service_sdk::my_logger;`.

## Logger of rust-extensions

`MyLogger` implements `rust_extensions::Logger`, so `my_logger::LOGGER.clone()` goes wherever an `Arc<dyn Logger + Send + Sync + 'static>` is asked for - `MyTimer`, `BackgroundExecutor` and the like.
