# FEVM integration


Error 
```
Testing solidity API
Try to call constructor on storage power actor
[src/main.rs:74] &res = ApplyRet {
    msg_receipt: Receipt {
        exit_code: ExitCode {
            value: 1,
        },
        return_data: RawBytes {  },
        gas_used: 0,
        events_root: None,
    },
    penalty: TokenAmount(0.0000001),
    miner_tip: TokenAmount(0.0),
    base_fee_burn: TokenAmount(0.0),
    over_estimation_burn: TokenAmount(0.0),
    refund: TokenAmount(0.0),
    gas_refund: 0,
    gas_burned: 0,
    failure_info: Some(
        PreValidation(
            "Send not from valid sender",
        ),
    ),
    exec_trace: [],
    events: [],
}
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', src/main.rs:76:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
