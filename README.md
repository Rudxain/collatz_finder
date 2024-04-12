# About

## Archive
[Reason](https://youtube.com/watch?v=jlh21U2texo&lc=UgzLFcfKTav59WvOB0Z4AaABAg).

> [!note]
> This repo is in maintenance hiatus/stasis.

## Intro
This allows any human to aid in the search for a counterexample that disproves the Collatz Conjecture.

However, the current purpose of this repo is just for learning purposes.

## Usage
> [!important]
> Expect **breaking API changes at literally any time**.
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

## FAQ

> "What to do if I find a counter-example?"

This is not a stupid question! I would also not know what to do in one of the most epic moments in the history of math and CS!

I suggest you to post your results everywhere! [except Wikipedia](https://en.wikipedia.org/wiki/Wikipedia:No_original_research). To make people take you seriously, you must post the numeral that disproves CC. It'll be  **H U G E** ,  so I recommend posting a [gist](https://gist.github.com) containing the full numeral, and then share links to that gist.

Tip for smaller size: hexadecimal is more compact than dec, raw-binary is more compact than hex. If you use hex or raw, disambiguate endianness.

Another tip, **ensure nobody else takes credit for it.** I know that sounds egotistical, but nobody wants to give countless hours of computing power for free, am I right?

## etc
> [!note]
> The program assumes that numbers don't diverge to +♾️, it only searches cycles. So it could enter an infinite loop that allocates increasingly more memory, until an [OOM](https://en.wikipedia.org/wiki/Out_of_memory) panic happens.

Just like my [JS implementation](https://github.com/Rudxain/Collatz-finder), it supports negatives. ~It also supports multiple bases/radices~ (not anymore, sorry 🙁, I might fix it later).

The rationale behind the multi-base support, is that CC is more interesting and helpful to explore/experiment in **bases 2 & 3** and any other base whose factors are 2 and/or 3, and bases whose factors are +-1 offset from 2 and/or 3, so I had to add all bases from 1 to 10 (and hexdec, because it's binary in disguise)
