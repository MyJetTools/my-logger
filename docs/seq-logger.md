# Seq logger

A [consumer](consumers.md) which sends the records to [Seq](https://datalust.co/seq) or to my-logger-server. It comes with the `my-seq-logger` feature of `my-logger` as `my_logger::my_seq_logger`. How to plug it: [readme](../README.md#plug-the-seq-logger). What it is set up with: [connection string](seq-connection-string.md).

## Start

`SeqLogger::enable_from_connection_string(settings).await` is called once, at the start of the application, inside a Tokio runtime. It:

1. Sets the panic hook - see [Panics](#panics).
2. Reads the connection string. Without `url` in it the call does not return: it prints `Can not parse Logs settings` and reads the string again every 5 seconds. A string which can not be parsed otherwise [panics](seq-connection-string.md#a-string-which-can-not-be-parsed).
3. Starts the task which sends the records. It runs on the runtime the call is awaited on and sends for as long as that runtime lives.
4. Plugs itself into `my_logger::LOGGER` and prints `Seq Logs writer is started`.

A second call plugs a second sender, and every record is sent twice.

Writing a record needs no runtime: it is put into a queue in memory, from any thread.

## How the records are sent

- The task takes everything there is in the queue and sends it as one request. What is written while a request is on its way goes with the next one.
- A request carries up to 2 MB of records; the rest goes in the requests after it.
- With `queuesize` in the connection string the queue holds that many records, and a record written into a full queue is dropped. Without it the queue has no limit.

The request is `POST {url}/api/events/raw?clef`:

| Header | |
|---|---|
| `Content-Type` | `application/vnd.serilog.clef` |
| `X-Seq-ApiKey` | `apikey` of the connection string, when it is there. |
| `scope` | `product` of the connection string, when it is there - see [Product scope](seq-connection-string.md#product-scope). |

The body is a record per line, the lines separated by `\r\n`:

```json
{"@l":"Info","@t":"2026-10-12T09:24:22.984955","Process":"process_name","@m":"message","Application":"my-app","Version":"0.1.0","orderId":15,"comment":"hello"}
```

| Field | |
|---|---|
| `@l` | `Info`, `Warning`, `Error`, `Fatal` or `Debug`. |
| `@t` | When the record is written, UTC. |
| `Process` | The process of the record. |
| `@m` | The message of the record. |
| the rest | The [parameters of every record](populated-params.md), then the context of the record. A context value which reads as a number, `true`, `false` or `null` is written as such, any other one as a string. |

## When a request fails

A request is tried 4 times, then its records are dropped:

- **No response** - the server is not reachable, or does not answer within `timeout`. Each attempt prints `Attempt: N Error while uploading logs to seq` to stderr, the attempts are a second apart.
- **A status other than 200-209.** The attempts follow one another with no pause and print nothing.

## Panics

`enable_from_connection_string` replaces the panic hook of the process. A panic anywhere in the application is written as a `FatalError` record with the process `Panic Handler`, the panic message as the message, and `Location` and `PanicInfo` in the context.

The record is printed to the console at once and sent in the background as any other one, so a panic which ends the process may not get to the server.

## HTTPS

An `https://` url needs a TLS provider feature of [flurl](https://github.com/MyJetTools/fl-url) - `with-ring-tls` or `with-rust-tls` - turned on by the application; service-sdk has the features of the same names. Without one every request fails with `FlUrl does not support https` in stderr and the records are dropped.

## Environment variables

| Variable | |
|---|---|
| `SEQ_DEBUG` | Set to anything - every request and the response to it are printed to the console. |
| `SEQ_COMPRESS` | Set to anything - a request over 1 MB is sent gzipped. |
| `ENV_INFO` | Sent as `EnvInfo` with every record - see [Parameters of every record](populated-params.md). |
