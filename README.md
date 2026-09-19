# rust-for-systems-starter

The starter workspace for **Rust for Systems: Read a SQLite File** — a
[SourceBoot](https://sourceboot.com) course where you learn Rust by building one
real thing: a reader for the SQLite file format, from the 100-byte file header to
walking a b-tree and printing a table's rows.

This repo is exactly what a learner's workspace starts as: a plain cargo workspace
holding one library crate, `sqlkit`, with a module per topic — most of them empty,
one of them deliberately broken. The lessons, the per-lab tests and the grader are
deliberately **not** in here — they live on
[sourceboot.com](https://sourceboot.com) and arrive through the `sboot` CLI, into a
separate cache directory. A repo created from this template stays your code and
nothing else, which is what makes it worth showing people.

> Renamed 2026-09-01 (was `rust-core-starter`, when the course id was `rust-core`). GitHub
> redirects renamed repos, so a template link you already have keeps working.

## Use it

Two ways in; both give you the same tree.

**With GitHub** — your reader starts life as a private repo you own:

```sh
gh repo create my-sqlite-reader --private --template sourceboot/rust-for-systems-starter --clone
cd my-sqlite-reader
```

Then install `sboot` and work from inside the clone:

```sh
curl -fsSL https://sourceboot.com/install.sh | sh
sboot login                   # connects this machine, in your browser
sboot test 00-welcome         # fetches the lab's tests + grader, runs them, grades
```

`sboot` recognises the repo by its `sboot.toml` and downloads each lab's tests on
first use (`sboot where` prints where they live — outside this repo). Running
`sboot start rust-for-systems` inside the clone is safe: it puts back anything
that is missing and never touches a file you have edited. With the template you
already have the tree, so you don't need it.

**Without GitHub:**

```sh
sboot start rust-for-systems
```

materialises this same tree into `./sqlite-reader-sb/` — named after what you
build rather than after the course — makes it a git repository and commits it,
asking once for the name and email git stamps on your commits. No `gh` and no
template involved; `--dir <name>` picks a different folder. When you want it on
GitHub, `sboot repo` creates the private repo and pushes it.

## What's in the tree

```
db/                     the cargo workspace you own
  sqlkit/               the library you write, one lab at a time
    src/                a module per topic — bytes, header, page, cells,
                        record, btree, dump, error. Most start EMPTY, and
                        lib.rs declares them as the labs tell you to
    src/bin/            sqlite_info and sqlite_dump — the two programs the
                        course builds up to; placeholders until then
    tests/              your tests, plus the real SQLite files to read them
                        against (tests/fixtures/)
.cargo/                 the build flags every compile here uses (no `unsafe`, and
                        the short list of calls the library may not make — read
                        clippy.toml); the same file the grader compiles under
rust-toolchain.toml     pinned stable Rust plus clippy — that is the whole toolchain
sboot.toml              tells the sboot CLI which course this repo is for
```

Stable Rust, on your own machine, with **no external crates, ever** — writing the
byte readers, the varints, the error type and the b-tree walk yourself is the
course; reaching for `rusqlite` or `byteorder` solves a different problem. Every
program here runs straight on your own machine, so a fresh clone is ready in
seconds:

```sh
cd db && cargo test -p sqlkit
```

## One file does not compile, and that is lab 01

`sqlkit/src/warmup.rs` ships with six deliberate errors in it. A fresh clone still
builds, because `lib.rs` does not declare the module yet — and lab 01's very first
instruction is to add `pub mod warmup;`, which breaks the build on purpose. Fixing
what rustc then tells you, in the two rounds it tells you, *is* the lab: four errors
on the first pass, two more about ownership once those are gone.

So: if your first build after starting lab 01 fails, nothing is wrong with this
template. That is the course starting.

## The course

https://sourceboot.com — lessons, labs and grading. This template is just the
starting tree.

## License

MIT — see [LICENSE](LICENSE). The scaffold is yours to build on and publish.
The course prose, tests and grader are not in this repo and are not covered by
it.
