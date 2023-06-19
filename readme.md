## Usage of my_logger


### How to use it in your application

Just write such kinds of codes at any places inside your code
```rust

// writing information line
my_logger::LOGGER.write_info("process_name", 
  "message", 
  LogEventCtx::new()
    .add("field1","value1")
    .add("field2","value2")
    .add("field3","value3")
);

// writing warning line
my_logger::LOGGER.write_warning("process_name",
  "message",
  LogEventCtx::new()
    .add("field1","value1")
    .add("field2","value2")
    .add("field3","value3")
);

// writing error line
my_logger::LOGGER.write_error("process_name",
  "message",
  LogEventCtx::new()
    .add("field1","value1")
    .add("field2","value2")
    .add("field3","value3")
);

// writing fatal error line
my_logger::LOGGER.write_fatal_error("process_name",
  "message",
  LogEventCtx::new()
    .add("field1","value1")
    .add("field2","value2")
    .add("field3","value3")
);

// writing fatal debug error line
my_logger::LOGGER.write_fatal_debug("process_name",
  "message",
  LogEventCtx::new()
    .add("field1","value1")
    .add("field2","value2")
    .add("field3","value3")
);

```
