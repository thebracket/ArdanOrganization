# Async Wrap-Up

We're not going to belabor the point by doing the multi-lane as well - it's the same, minus using `send_async` and `recv_async` instead of `send` and `recv`. Flume is native to both async and synchronous code.

If you are building a network-connected async system, channels can be an excellent way to divide your program up - with even less overhead than threading.

> Async actors are widely used, and follow this pattern - or use a framework (we'll discuss those later). Netflix, in particular, have been giving great talks at RustConf about the amazing performance they are seeing with this pattern.
