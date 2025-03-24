# Named optional parameters

Named optional parameters are parameters for logging macros to configure this log record.

Each parameter is optional and no order is required. It can appear at the beginning or end of a log macro argument list, but cannot appear among the formatting arguments.

| Name   | Type / Basic Syntax       | Description                                                                       |
|--------|---------------------------|-----------------------------------------------------------------------------------|
| logger | `Arc<Logger>` or `Logger` | If specified, the given logger will be used instead of the global default logger. |
| kv     | `{ key = value }`         | Key-value pairs for structured logs. See documentation of module [`kv`] for more. |

[`kv`]: crate::kv
