# FEVM integration


Error 
```
    failure_info: Some(
        MessageBacktrace(
            Backtrace {
                frames: [
                    Frame {
                        source: 101,
                        method: 1,
                        code: ExitCode {
                            value: 23,
                        },
                        message: "EVM execution error: InvalidMemoryAccess",
                    },
                    Frame {
                        source: 1,
                        method: 3,
                        code: ExitCode {
                            value: 23,
                        },
                        message: "constructor failed: send to f0101 method 1 aborted with code 23",
                    },
                    Frame {
                        source: 10,
                        method: 3,
                        code: ExitCode {
                            value: 23,
                        },
                        message: "send to f01 method 3 aborted with code 23",
                    },
                ],
                cause: None,
            },
        ),
    ),

```
