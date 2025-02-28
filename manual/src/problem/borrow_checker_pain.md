# Borrowing and Lifetimes

It's sometimes tempting to make things static, because static objects automatically have the `'static` lifetime --- which is a bit of an escape hatch, and in *most* cases allow you to pretend that you don't have a borrow checker watching over you.

There are times that a `static` is genuinely needed, but it can quickly become a really chaotic antipattern. For example:

* The `tracing` or `log` crate retains a global logging configuration. It has to---messages can be coming in from *anywhere*, and your program's registry of policies determines what to do with the messages.
* Signal handlers (`ctrl-c`, `signals`, handling `sigkill` etc.) **must** be static. The whole program can only have one, the Operating System needs to know exactly where they are to call them. Signal handlers are special, because they may be invoked at *any* time, so they really can't borrow anything!
* FFI and interaction with C/C++/other languages. Sometimes, global callbacks are required. These are *going* to be static!
* Allocators. Most of the time, if you set a global allocator (such as `jemalloc` or `mimalloc`) it *has* to be static because you want a single memory allocator for your whole system. There are exceptions, such as when you are using Bump arenas (Bumpalo) or local slab allocation---but generally, the allocator is static.

There are times when a `static` is actually pretty nice:

* Precomputed lookup tables, protected with `OnceLock` can provide a really fast way to provide a read-only data store. Static fits well, because it's inherently singular. You *can* use a more elaborate scheme---and probably should as you scale. But it's really nice when you're getting started.
* Caches (with eviction) are often static, because multiple services can share them. That usually starts well, and can---as we just saw---become a bit of a maintenance nightmare. You *can and should* use a more elaborate scheme than a global static in many cases.
