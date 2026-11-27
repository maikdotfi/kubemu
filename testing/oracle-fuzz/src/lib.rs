//! Differential fuzzing against the Go oracle (README §10).
//!
//! Inputs come from a seeded [`Xoshiro256StarStar`], one seed per case, so
//! any mismatch replays with `--seed <case> --cases 1`. Each case goes to
//! both kubemu and the oracle process; results (or error texts) must agree.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout};

use kubemu_env::Xoshiro256StarStar;
use serde_json::{Map, Value, json};

/// A running `oracle-go` process speaking line-delimited JSON.
#[derive(Debug)]
pub struct Oracle {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Oracle {
    pub fn spawn(binary: &Path) -> std::io::Result<Self> {
        let mut child = kubemu_env_real::spawn_piped(binary, &[])?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = BufReader::new(child.stdout.take().expect("piped stdout"));
        let mut o = Self {
            child,
            stdin,
            stdout,
        };
        match o.call("ping", Value::Null)? {
            Ok(v) if v == json!("pong") => Ok(o),
            other => Err(std::io::Error::other(format!(
                "oracle ping failed: {other:?}"
            ))),
        }
    }

    /// Sends one request. The outer error is transport; the inner one is
    /// the oracle's own error text.
    pub fn call(&mut self, op: &str, payload: Value) -> std::io::Result<Result<Value, String>> {
        let req = json!({"op": op, "payload": payload});
        writeln!(self.stdin, "{req}")?;
        self.stdin.flush()?;
        let mut line = String::new();
        if self.stdout.read_line(&mut line)? == 0 {
            return Err(std::io::Error::other("oracle closed its stdout"));
        }
        let resp: Value = serde_json::from_str(&line).map_err(std::io::Error::other)?;
        Ok(match resp.get("error").and_then(Value::as_str) {
            Some(e) => Err(e.to_owned()),
            None => Ok(resp.get("result").cloned().unwrap_or(Value::Null)),
        })
    }
}

impl Drop for Oracle {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Random JSON shaped to make merges interesting: few distinct keys so
/// document and patch collide, plenty of nulls, nested objects in arrays.
#[derive(Debug)]
pub struct Gen(Xoshiro256StarStar);

const KEYS: [&str; 5] = ["a", "b", "c", "d", "e"];
const STRINGS: [&str; 5] = ["", "x", "<&>", "é\"\\", "null"];

impl Gen {
    pub fn new(seed: u64) -> Self {
        Self(Xoshiro256StarStar::seed_from_u64(seed))
    }

    fn below(&mut self, n: u64) -> u64 {
        // Modulo bias is irrelevant for input generation.
        self.0.next_u64() % n
    }

    pub fn value(&mut self, depth: u32) -> Value {
        let kinds = if depth == 0 { 5 } else { 8 };
        match self.below(kinds) {
            0 | 1 => Value::Null,
            2 => Value::Bool(self.below(2) == 1),
            3 => match self.below(3) {
                0 => json!(self.below(100)),
                1 => json!(-(self.below(100) as i64)),
                _ => json!(self.below(100) as f64 + 0.5),
            },
            4 => json!(STRINGS[self.below(STRINGS.len() as u64) as usize]),
            5 => Value::Array((0..self.below(4)).map(|_| self.value(depth - 1)).collect()),
            _ => Value::Object(self.object(depth - 1)),
        }
    }

    pub fn object(&mut self, depth: u32) -> Map<String, Value> {
        (0..self.below(5))
            .map(|_| {
                let k = KEYS[self.below(KEYS.len() as u64) as usize].to_owned();
                (k, self.value(depth))
            })
            .collect()
    }

    /// Mostly objects at the top, like real documents and patches.
    pub fn top(&mut self, depth: u32) -> Value {
        if self.below(10) < 8 {
            Value::Object(self.object(depth))
        } else {
            self.value(depth)
        }
    }
}

/// One merge-patch case; `Err` describes the mismatch.
pub fn merge_case(oracle: &mut Oracle, seed: u64) -> std::io::Result<Result<(), String>> {
    let mut g = Gen::new(seed);
    let doc = g.top(3);
    let patch = g.top(3);
    let ours = kubemu_patch::merge_patch(&doc, &patch).map_err(|e| e.to_string());
    let theirs = oracle.call("merge", json!({"original": doc, "patch": patch}))?;
    Ok(if ours == theirs {
        Ok(())
    } else {
        Err(format!(
            "merge mismatch\n  doc:    {doc}\n  patch:  {patch}\n  kubemu: {ours:?}\n  oracle: {theirs:?}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_is_seeded() {
        let a: Vec<Value> = (0..50).map(|s| Gen::new(s).top(3)).collect();
        let b: Vec<Value> = (0..50).map(|s| Gen::new(s).top(3)).collect();
        assert_eq!(a, b);
        assert!(a.iter().any(|v| v.to_string().contains("null")));
    }
}
