# RunKV Log Matcher

RunKV log matcher for deadlock detection.

## Usage

Example:

Run RunKV as:
```bash
RUSTFLAGS="--cfg tokio_unstable" RUST_LOG=runkv_wheel::worker::raft=info,runkv_common::notify_pool=trace RUNKV_METRICS=true RUST_BACKTRACE=1 cargo run --release --features trace-notify-pool --package runkv-bench --bin bench_kv -- --groups 2 --loop 10000  --persist none
```

Run RunKV log matcher as:
```bash
cargo run --release -- ~/runkv/.run/tmp/bench-kv/log-1652943711845/runkv-tests-0.log.2022-05-19
```

Outputs:
```plain
line:        1 =====> raft: : switched to configuration raft_id=1 raft node=1 group=1 namespace=raft
line:        2 =====> raft: : became follower at term 0 raft_id=1 raft node=1 group=1 namespace=raft
line:        3 =====> raft: : newRaft raft_id=1 raft node=1 group=1 namespace=raft
line:        4 =====> raft: : RawNode created with id 1. raft_id=1 raft node=1 group=1 namespace=raft
line:        5 =====> raft: : switched to configuration raft_id=3 raft node=3 group=1 namespace=raft
line:        6 =====> raft: : became follower at term 0 raft_id=3 raft node=3 group=1 namespace=raft
line:        7 =====> raft: : newRaft raft_id=3 raft node=3 group=1 namespace=raft
line:        8 =====> raft: : RawNode created with id 3. raft_id=3 raft node=3 group=1 namespace=raft
line:        9 =====> raft: : switched to configuration raft_id=2 raft node=2 group=1 namespace=raft
line:       10 =====> raft: : became follower at term 0 raft_id=2 raft node=2 group=1 namespace=raft
line:       11 =====> raft: : newRaft raft_id=2 raft node=2 group=1 namespace=raft
line:       12 =====> raft: : RawNode created with id 2. raft_id=2 raft node=2 group=1 namespace=raft
line:       13 =====> raft: : switched to configuration raft_id=5 raft node=5 group=2 namespace=raft
line:       14 =====> raft: : became follower at term 0 raft_id=5 raft node=5 group=2 namespace=raft
line:       15 =====> raft: : newRaft raft_id=5 raft node=5 group=2 namespace=raft
line:       16 =====> raft: : RawNode created with id 5. raft_id=5 raft node=5 group=2 namespace=raft
line:       17 =====> raft: : switched to configuration raft_id=4 raft node=4 group=2 namespace=raft
line:       18 =====> raft: : became follower at term 0 raft_id=4 raft node=4 group=2 namespace=raft
line:       19 =====> raft: : newRaft raft_id=4 raft node=4 group=2 namespace=raft
line:       20 =====> raft: : RawNode created with id 4. raft_id=4 raft node=4 group=2 namespace=raft
line:       21 =====> raft: : switched to configuration raft_id=6 raft node=6 group=2 namespace=raft
line:       22 =====> raft: : became follower at term 0 raft_id=6 raft node=6 group=2 namespace=raft
line:       23 =====> raft: : newRaft raft_id=6 raft node=6 group=2 namespace=raft
line:       24 =====> raft: : RawNode created with id 6. raft_id=6 raft node=6 group=2 namespace=raft
line:       26 =====> raft: : starting a new election raft_id=6 raft node=6 group=2 namespace=raft
line:       27 =====> raft: : became pre-candidate at term 0 raft_id=6 raft node=6 group=2 namespace=raft
line:       28 =====> raft: : broadcasting vote request raft_id=6 raft node=6 group=2 namespace=raft
line:       41 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 6 [logterm: 0, index: 0] at term 0 raft_id=5 raft node=5 group=2 namespace=raft
line:       42 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 6 [logterm: 0, index: 0] at term 0 raft_id=4 raft node=4 group=2 namespace=raft
line:       67 =====> raft: : received votes response raft_id=6 raft node=6 group=2 namespace=raft
line:       68 =====> raft: : became candidate at term 1 raft_id=6 raft node=6 group=2 namespace=raft
line:       69 =====> raft: : broadcasting vote request raft_id=6 raft node=6 group=2 namespace=raft
line:       82 =====> raft: : received a message with higher term from 6 raft_id=4 raft node=4 group=2 namespace=raft
line:       83 =====> raft: : received a message with higher term from 6 raft_id=5 raft node=5 group=2 namespace=raft
line:       84 =====> raft: : became follower at term 1 raft_id=4 raft node=4 group=2 namespace=raft
line:       85 =====> raft: : became follower at term 1 raft_id=5 raft node=5 group=2 namespace=raft
line:       86 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 6 [logterm: 0, index: 0] at term 1 raft_id=4 raft node=4 group=2 namespace=raft
line:       90 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 6 [logterm: 0, index: 0] at term 1 raft_id=5 raft node=5 group=2 namespace=raft
line:      112 =====> raft: : received votes response raft_id=6 raft node=6 group=2 namespace=raft
line:      113 =====> raft: : became leader at term 1 raft_id=6 raft node=6 group=2 namespace=raft
line:      186 =====> raft: : starting a new election raft_id=3 raft node=3 group=1 namespace=raft
line:      187 =====> raft: : became pre-candidate at term 0 raft_id=3 raft node=3 group=1 namespace=raft
line:      188 =====> raft: : broadcasting vote request raft_id=3 raft node=3 group=1 namespace=raft
line:      201 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 3 [logterm: 0, index: 0] at term 0 raft_id=1 raft node=1 group=1 namespace=raft
line:      202 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 3 [logterm: 0, index: 0] at term 0 raft_id=2 raft node=2 group=1 namespace=raft
line:      227 =====> raft: : received votes response raft_id=3 raft node=3 group=1 namespace=raft
line:      228 =====> raft: : became candidate at term 1 raft_id=3 raft node=3 group=1 namespace=raft
line:      229 =====> raft: : broadcasting vote request raft_id=3 raft node=3 group=1 namespace=raft
line:      243 =====> raft: : received a message with higher term from 3 raft_id=1 raft node=1 group=1 namespace=raft
line:      244 =====> raft: : became follower at term 1 raft_id=1 raft node=1 group=1 namespace=raft
line:      245 =====> raft: : received a message with higher term from 3 raft_id=2 raft node=2 group=1 namespace=raft
line:      246 =====> raft: : became follower at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:      247 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 3 [logterm: 0, index: 0] at term 1 raft_id=1 raft node=1 group=1 namespace=raft
line:      251 =====> raft: : [logterm: 0, index: 0, vote: 0] cast vote for 3 [logterm: 0, index: 0] at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:      286 =====> raft: : received votes response raft_id=3 raft node=3 group=1 namespace=raft
line:      293 =====> raft: : became leader at term 1 raft_id=3 raft node=3 group=1 namespace=raft
line:   836847 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836848 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836849 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836863 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836864 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836865 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836879 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836880 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836881 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836896 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836897 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836898 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836912 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836913 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836914 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836928 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836929 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836930 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836945 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836946 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836947 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836961 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836962 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836963 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836977 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836978 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836979 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   836993 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   836994 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   836995 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837010 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837011 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837012 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837026 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837027 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837028 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837042 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837043 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837044 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837059 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837060 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837061 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837075 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837076 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837077 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837091 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837092 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837093 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
line:   837108 =====> raft: : starting a new election raft_id=2 raft node=2 group=1 namespace=raft
line:   837109 =====> raft: : became pre-candidate at term 1 raft_id=2 raft node=2 group=1 namespace=raft
line:   837110 =====> raft: : broadcasting vote request raft_id=2 raft node=2 group=1 namespace=raft
unmatch entries:
line:   836842 =====> raft node:   5   log: ApplyEnter
line:   836845 =====> raft node:   1   log: ApplyEnter
```