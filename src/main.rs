use std::{collections::BTreeMap, io::BufRead};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Log {
    SendEnter,
    SendExit,
    AppendEnter,
    AppendExit,
    ApplyEnter,
    ApplyExit,
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
            _ => Err((kind.to_string(), verb.to_string())),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut m: BTreeMap<(u64, Log), u64> = BTreeMap::new();
    let mut nmatch: BTreeMap<(u64, Log), u64> = BTreeMap::new();

    let re_log =
        regex::Regex::new(r"worker::raft: (\d+) (send|append|apply) (enter|exit)").unwrap();
    // let re_raft = regex::Regex::new(r"became leader.+raft node=(\d+)").unwrap();
    let re_raft = regex::Regex::new(r"tracing_slog_drain(.+$)").unwrap();

    println!("path: {}", &args[1]);

    let f = std::fs::File::open(&args[1]).unwrap();
    for (i, line) in std::io::BufReader::new(f).lines().enumerate() {
        let i = i as u64 + 1;
        let line = line.unwrap();
        if let Some(cap) = re_raft.captures(&line) {
            println!("line: {:8} =====> raft: {}", i, &cap[1]);
        }
        if let Some(cap) = re_log.captures(&line) {
            let raft_node: u64 = cap[1].parse().unwrap();
            let log = Log::parse(&cap[2], &cap[3]).unwrap();
            match log {
                Log::SendEnter | Log::AppendEnter | Log::ApplyEnter => {
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
