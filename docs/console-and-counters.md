# Console and counters

## Console

Every record is printed to the console as soon as it is written, with or without consumers plugged:

```
2026-10-12T09:24:22.984955+00:00 Info
Process: process_name
Message: message
Context: {"field1": "value1", "field2": "value2"}
-------------------
```

- `Info`, `Warning` and `Debug` go to stdout, `Error` and `FatalError` go to stderr.
- The `Context` line is there when the record has a context.
- The [parameters of every record](populated-params.md) are not printed.

## Counters

The records are counted by their level from the start of the application:

```rust
let errors = my_logger::LOGGER.get_errors_amount();
```

| Method | Counts |
|---|---|
| `get_infos_amount()` | `Info` |
| `get_warnings_amount()` | `Warning` |
| `get_errors_amount()` | `Error` |
| `get_fatal_errors_amount()` | `FatalError` |
| `get_debugs_amount()` | `Debug` |
