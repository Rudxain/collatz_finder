# Context
[Website repo](https://github.com/Rudxain/Collatz-finder)

# Usage
To install:
```sh
cargo install --git https://github.com/Rudxain/collatz_finder.git
```
To check if a number `n` is a counter-example:
```sh
collatz_finder [n]
```

Just like my JS implementation, it supports negatives. It also supports multiple bases/radices, run `collatz_finder --help` for more info.

I did some benchmarks and it's **EXTREMELY FASTER** than the JS counterpart at checking nums!
