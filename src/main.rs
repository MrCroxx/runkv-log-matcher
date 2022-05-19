use std::{collections::BTreeMap, io::BufRead};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Log {
    // runkv_wheel::worker::raft
    SendEnter,
    SendExit,
    AppendEnter,
    AppendExit,
    ApplyEnter,
    ApplyExit,
    FsmApplyEnter,
    FsmApplyExit,

    PoolEnter,
    PoolExit,

    // runkv_wheel::components::fsm
    EntriesEnter,
    EntriesExit,
    MutableEnter,
    MutableExit,
    ImmutableEnter,
    ImmutableExit,

    StoreEnter,
    StoreExit,
    LoadEnter,
    LoadExit,

    // runkv_storage::raft_log_store::store
    LogAppendEnter,
    LogAppendExit,
    StatePutEnter,
    StatePutExit,
}

impl Log {
    fn parse(kind: &str, verb: &str) -> Result<Self, (String, String)> {
        match (kind, verb) {
            ("send", "enter") => Ok(Self::SendEnter),
            ("send", "exit") => Ok(Self::SendExit),
            ("append", "enter") => Ok(Self::AppendEnter),
            ("append", "exit") => Ok(Self::AppendExit),
            ("apply", "enter") => Ok(Self::ApplyEnter),
            ("apply", "exit") => Ok(Self::ApplyExit),
            ("fsmapply", "enter") => Ok(Self::FsmApplyEnter),
            ("fsmapply", "exit") => Ok(Self::FsmApplyExit),

            ("pool", "enter") => Ok(Self::PoolEnter),
            ("pool", "exit") => Ok(Self::PoolExit),

            ("entries", "enter") => Ok(Self::EntriesEnter),
            ("entries", "exit") => Ok(Self::EntriesExit),
            ("mutable", "enter") => Ok(Self::MutableEnter),
            ("mutable", "exit") => Ok(Self::MutableExit),
            ("immutable", "enter") => Ok(Self::ImmutableEnter),
            ("immutable", "exit") => Ok(Self::ImmutableExit),

            ("store", "enter") => Ok(Self::StoreEnter),
            ("store", "exit") => Ok(Self::StoreExit),
            ("load", "enter") => Ok(Self::LoadEnter),
            ("load", "exit") => Ok(Self::LoadExit),

            ("logappend", "enter") => Ok(Self::LogAppendEnter),
            ("logappend", "exit") => Ok(Self::LogAppendExit),
            ("stateput", "enter") => Ok(Self::StatePutEnter),
            ("stateput", "exit") => Ok(Self::StatePutExit),

            _ => Err((kind.to_string(), verb.to_string())),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut m: BTreeMap<(u64, Log), u64> = BTreeMap::new();
    let mut nmatch: BTreeMap<(u64, Log), u64> = BTreeMap::new();

    let re_log = regex::Regex::new(
        r"(runkv_wheel::worker::raft:|runkv_wheel::components::fsm:|runkv_storage::raft_log_store::store:) (\d+) (send|append|apply|fsmapply|entries|mutable|immutable|pool|store|load|logappend|stateput) (enter|exit)",
    )
    .unwrap();
    // let re_raft = regex::Regex::new(r"became leader.+raft node=(\d+)").unwrap();
    let re_raft = regex::Regex::new(r"tracing_slog_drain: (.+$)").unwrap();

    println!("path: {}", &args[1]);

    let f = std::fs::File::open(&args[1]).unwrap();
    for (i, line) in std::io::BufReader::new(f).lines().enumerate() {
        let i = i as u64 + 1;
        let line = line.unwrap();
        if let Some(cap) = re_raft.captures(&line) {
            println!("line: {:8} =====> raft: {}", i, &cap[1]);
        }
        if let Some(cap) = re_log.captures(&line) {
            let raft_node: u64 = cap[2].parse().unwrap();
            let log = Log::parse(&cap[3], &cap[4]).unwrap();
            match log {
                Log::SendEnter
                | Log::AppendEnter
                | Log::ApplyEnter
                | Log::FsmApplyEnter
                | Log::EntriesEnter
                | Log::MutableEnter
                | Log::ImmutableEnter
                | Log::PoolEnter
                | Log::StoreEnter
                | Log::LoadEnter
                | Log::LogAppendEnter
                | Log::StatePutEnter => {
                    if let Some(l) = m.insert((raft_node, log), i) {
                        nmatch.insert((raft_node, log), l);
                    }
                }
                Log::SendExit => {
                    if m.remove(&(raft_node, Log::SendEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::AppendExit => {
                    if m.remove(&(raft_node, Log::AppendEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::ApplyExit => {
                    if m.remove(&(raft_node, Log::ApplyEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::FsmApplyExit => {
                    if m.remove(&(raft_node, Log::FsmApplyEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::EntriesExit => {
                    if m.remove(&(raft_node, Log::EntriesEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::MutableExit => {
                    if m.remove(&(raft_node, Log::MutableEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::ImmutableExit => {
                    if m.remove(&(raft_node, Log::ImmutableEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::PoolExit => {
                    if m.remove(&(raft_node, Log::PoolEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::StoreExit => {
                    if m.remove(&(raft_node, Log::StoreEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::LoadExit => {
                    if m.remove(&(raft_node, Log::LoadEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::LogAppendExit => {
                    if m.remove(&(raft_node, Log::LogAppendEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
                Log::StatePutExit => {
                    if m.remove(&(raft_node, Log::StatePutEnter)).is_none() {
                        nmatch.insert((raft_node, log), i);
                    }
                }
            }
        }
    }

    println!("unmatch entries:");
    let mut total = Vec::new();
    for entry in m {
        total.push(entry);
    }
    for entry in nmatch {
        total.push(entry);
    }
    total.sort_by_key(|entry| entry.1);
    for ((raft_node, log), l) in total {
        println!(
            "line: {:8} =====> raft node: {:3}   log: {:?}",
            l, raft_node, log
        );
    }
}
