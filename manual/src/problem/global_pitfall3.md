# Problem 3: I thought Rust was fast?

Rust used to be fast, and now your performance is awful. `top` and friends aren't as helpful as you'd like, showing CPUs in `wait` states a lot---you're not using much CPU, but you aren't pushing much data either.

That's because *everyone* is pounding the single, global lock.

> This is just like Python's Global Lock, the Global Lock in Linux and FreeBSD that took years to remove, etc. Once you have a global lock, they have a bad habit of being *really hard* to remove.

`RwLock` gives fast access to uncontested reads---but *writers* have to wait for every read to expire before they prevent ANYONE from reading until they are done. You carefully audit the code and find that someone (let's stop picking on Joey) has held a lock for far too long, stalling everyone else.