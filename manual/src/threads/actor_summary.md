# Threaded Actors Summary

The actor model isn't the answer to *every* situation, but it can be a really helpful addition to your toolkit when you are building large Rust programs. It provides:

* Clean division of responsibility for developers.
* Clean code.
* The ability to scale loads, and scale out if you need to.

There are some downsides:

* Actors aren't suitable for times when a threaded context switch is too long to wait (16 microseconds on Linux by defalt).
* Over-reliance on sychronizing with oneshot can have the same effect as a Mutex---slowing you down to effectively synchronous access at bottlenecks. You can alleviate that with multiple lanes.

Before we talk about other patterns, let's look at the same thing - in async.
