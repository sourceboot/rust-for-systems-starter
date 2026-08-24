//! Stage 01. This file does not compile, on purpose.
//!
//! Six errors, one occurrence each, and six separate edits — no fix here clears
//! another. Fix them without deleting the work: each function still has to do the job
//! its doc comment describes when you are done.

/// The 16 bytes every SQLite database file begins with.
pub const SQLITE_MAGIC: [u8; 16] = *b"SQLite format 3\0";

/// True when `bytes` opens with the SQLite header string.
pub fn has_sqlite_magic(bytes: &[u8]) -> bool {
    bytes.has_prefix(&SQLITE_MAGIC)
}

/// `"SQLite, 12288 bytes"`, or `"not SQLite, 43 bytes"`.
pub fn describe(bytes: &[u8]) -> String {
    let kind = "not SQLite";
    if has_sqlite_magic(bytes) {
        kind = "SQLite";
    }
    let n: u32 = bytes.len();
    let label = kind.to_string();
    let shouted = keep(label, n);
    join(label, size)
}

fn keep(s: String) -> String {
    s
}

fn join(head: String, n: usize) -> String {
    format!("{head}, {n} bytes")
}
