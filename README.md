# Context

[Website repo](https://github.com/Rudxain/Collatz-finder)

## Usage

To install:

```sh
cargo install --git https://github.com/Rudxain/collatz_finder.git
```

To check if a number `n` is a counter-example:

```sh
colfind [n] check
```

To search a range of values:

```sh
colfind [n] search
```

Yes, the subcommand is in reverse, and IDK why.

Run this for more ℹinfo:

```sh
colfind --help
```

## etc

Just like my JS implementation, it supports negatives. ~It also supports multiple bases/radices~ (not anymore, sorry 🙁, I'll fix later).

The rationale behind the multi-base support, is that CC is more interesting and helpful to explore/experiment in **bases 2 & 3** and any other base whose factors are 2 and/or 3, and bases whose factors are +-1 offset from 2 and/or 3, so I had to add all bases from 1 to 10 (and hexdec, because it's binary in disguise lol)

I did some benchmarks and it's **EXTREMELY FASTER** than the JS counterpart at checking nums!
