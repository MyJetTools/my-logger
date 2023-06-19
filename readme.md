## Usage of my_logger


### How to use it in your application


Plug it

```toml
[dependencies]
my-logger = { tag = "max_version", git = "https://github.com/MyJetTools/my-logger.git" }
```


Put one of these lines at any place in your code
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

### How to configure
Some context values can be pre-populated by adding the line in the **fn main()**
```rust
#[tokio::main]
async fn main() {
   my_logger::LOGGER.populate_params("key","value");
   my_logger::LOGGER.populate_params("key2","value2");   
}
```
To configure application name and version method can be used
```rust
#[tokio::main]
async fn main() {
   my_logger::LOGGER.populate_app_and_version(env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_VERSION"));
}
```
