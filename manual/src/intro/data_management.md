# Data Management Challenges

When I'm traveling to give trainings, "how do we manage data in huge applications" comes up time and time again. There is an element of "how long is a piece of string?" --- every company's programs and needs are different, so *any* solution has to be customized to what you need. Customize in terms of:

* Speedy, SAFE access to the data you rely on.
* Optimize your access patterns to minimize locking.
* Organize your PROGRAM to avoid painful compilation times.
* Organize your SYSTEM to fit the organization arrangement.

With all of that said, there's a lot of common ground --- and many of the larger Rust consumers have settled on a few patterns.

> Don't worry, we've all made this mistake!

Probably the *worst* decision you will ever make is to type:

```rust
pub static IMPORTANT_DATA: RwLock<MyData> = RwLock::default();
```

And then export the data from your module/crate.

Suddenly, anyone - anywhere in the system - can obtain a Read or Write lock on your important data. Rust doesn't protect against deadlocks, and `RwLock` is easier to get into trouble with than a regular `Mutex` (but you shouldn't make that a global static, either!). You have no control over which parts of the program reach in and lock the world. Debugging can be a mess --- you won't have data races (Rust sees to that), but data confusion can be just as bad. There is *nothing* like sitting in the office at 2am trying to figure out why your `IMPORTANT_DATA` is sometimes becoming invalid, or---worse---restarting deadlocked systems!

Fortunately, statics don't have to be public. So you could build an accessor API and export that. Now you've at least limited the scope, and are responible for your own locking discipline.

Increasingly, larger companies are using the *actor model*---which was the original idea behind Object Oriented Programming, modules messaging one another. It's fast, it scales, and---if you do it right---it makes migration to distributed services easy because you've already got an API. You can even automate the distribution with actor frameworks like Ractor (which powers an increasing chunk of Netflix!).
