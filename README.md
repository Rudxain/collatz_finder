# About

[JS impl](https://github.com/Rudxain/Collatz-finder)

## Archive

[Reason](https://youtube.com/watch?v=jlh21U2texo&lc=UgzLFcfKTav59WvOB0Z4AaABAg).

> [!note]
> This repo is in maintenance hiatus/stasis.

## Usage
> [!important]
> This repo is just for learning purposes.
> Expect **breaking changes at literally any time**.
> 
> I won't provide support for anything, but you can ask questions if you're curious about stuff.
> 
> This isn't meant for production environments, **at all**.
> 
> I won't publish it to any package repository.

Install:
```sh
cargo install --git https://github.com/Rudxain/collatz_finder.git
```

ℹinfo:
```sh
colfind help
```

## etc
> [!note]
> Both JS & Rs assume that numbers don't diverge to +♾️, they only search cycles. So they could enter an infinite loop that allocates increasingly more memory, until an [OOM](https://en.wikipedia.org/wiki/Out_of_memory) panic happens.

Just like my JS implementation, it supports negatives. ~It also supports multiple bases/radices~ (not anymore, sorry 🙁, I might fix it later).

The rationale behind the multi-base support, is that CC is more interesting and helpful to explore/experiment in **bases 2 & 3** and any other base whose factors are 2 and/or 3, and bases whose factors are +-1 offset from 2 and/or 3, so I had to add all bases from 1 to 10 (and hexdec, because it's binary in disguise)
