# FEVM integration

Contract is being reverted when `value` is ot null.

Error 
```
    failure_info: Some(
        MessageBacktrace(
            Backtrace {
                frames: [
                    Frame {
                        source: 101,
                        method: 2,
                        code: ExitCode {
                            value: 33,
                        },
                        message: "contract reverted",
                    },
                ],
                cause: None,
            },
        ),
    ),

```
