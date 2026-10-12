# Connection string

What the [Seq logger](seq-logger.md) is set up with. The application gives it through `SeqSettings::get_conn_string` - see [readme](../README.md#plug-the-seq-logger).

```
url=http://127.0.0.1:5341;apikey=my-api-key;product=my-product;timeout=10
```

`key=value` pairs separated by `;`. The keys are in lower case, with no spaces around them and no `;` after the last pair. Only the first `=` of a pair separates, so a value may have `=` in it - but not `;`.

| Key | | Default | |
|---|---|---|---|
| `url` | required | | Where the records are sent: `{url}/api/events/raw`. An `https://` one needs [TLS turned on](seq-logger.md#https). |
| `apikey` | optional | none | Sent as the `X-Seq-ApiKey` header. |
| `product` | optional | none | The product scope of the records - see [Product scope](#product-scope). |
| `timeout` | optional | `10` | Seconds a request waits for the response. |
| `queuesize` | optional | no limit | How many records wait to be sent; a record over that is dropped. |
| `flushlogschunk` | optional | `50` | A number; read and not used. |
| `flushdelay` | optional | `1` | A number; read and not used. |

## Product scope

my-logger-server keeps and searches the records by product scope. `product` names the scope of everything the application writes: it is sent as the `scope` header with every request.

```
url=http://my-logger-server:8000;product=my-product
```

- A product is `a-z`, `0-9`, `-` and `_`, 64 characters at most. Spaces around it are cut off and upper case is turned to lower case: `product=My-Product` is sent as `my-product`.
- Anything else - an empty value as well - [panics](#a-string-which-can-not-be-parsed). The server ignores a scope it can not use without a word, so the logger does not let one through.
- Without `product` no header is sent. The server then finds the scope by the `Application` of a record, and takes its default scope when none matches.
- The server does not ask for a scope to be set up beforehand: a product with a typo in it becomes a scope of its own.
- Seq itself has no product scopes and takes no notice of the header.

## Changes on the fly

The string is read again before every request, so a change needs no restart:

- `url`, `apikey`, `product` and `timeout` apply from the next request.
- `queuesize` is taken at the start only.

## A string which can not be parsed

| What is wrong | What happens |
|---|---|
| No `url` | Not a panic: `Can not parse Logs settings` is printed to stderr and the string is read again every 5 seconds. Nothing is sent until then. |
| An unknown key - `Url`, `ApiKey` in upper case as well | panic |
| A pair without `=` - a `;` after the last pair as well | panic |
| `timeout`, `queuesize`, `flushlogschunk` or `flushdelay` which is not a number | panic |
| A `product` which can not name a scope | panic |

At the start a panic stops the application, with the reason in stderr.

Do not let the string turn into one which panics while the application is running: nothing is sent and the records written meanwhile are lost until it is fixed.
