# Parameters of every record

Parameters populated once are added to every record the [Seq logger](seq-logger.md) sends.

```rust
my_logger::LOGGER.populate_app_and_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
my_logger::LOGGER.populate_params("Region", "eu");
```

| Call | Adds |
|---|---|
| `populate_app_and_version(name, version)` | `Application` and `Version`. With the `ENV_INFO` environment variable set - `EnvInfo` with its value as well. |
| `populate_params(key, value)` | A parameter of your own. The key is a `&'static str`, the value is a `&'static str` or a `String`. |

- Populate them at the start of the application: a record sent before that goes without them.
- Every call adds. A key populated twice is sent twice, so each of them is called once.
- They are not printed to the console.
- `my_logger::LOGGER.get_populated_params()` returns what is populated.

`Application` is also what my-logger-server finds the product scope of a record by, when the connection string names no [product](seq-connection-string.md#product-scope).
