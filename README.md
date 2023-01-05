# FEVM integration

contract error when trying to use more complex struct.

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
    exec_trace: [
        GasCharge(
            GasCharge {
                name: "OnChainMessage",
                compute_gas: Gas(
                    38863.000,
                ),
                storage_gas: Gas(
                    176800.000,
                ),
            },
        ),
        Call {
            from: 100,
            to: Address {
                payload: ID(
                    101,
                ),
            },
            method: 2,
            params: RawBytes { 586405731c4f000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000 },
            value: TokenAmount(0.0),
        },


```
