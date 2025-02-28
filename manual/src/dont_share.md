# Don't Share - Communicate

> Don't communicate by sharing memory, share memory by communicating" - Rob Pike

Rob Pike of Go fame preaches the proverb that you should communicate rather than directly sharing memory. This often applies to Rust, too - although Rust's locking semantics make sharing less dangerous!

Rust includes *channels* - just like Go channels - to make communicating between threads and async tasks easier. They can provide a very powerful mechanism to minimize direct memory sharing, and maximize communication. They can help with both maintaining performance and regulating performance through back-pressure. You're also using them a lot more than you might think: every time you use `tracing::info!` or similar, behind the scenes the message is passing through a channel.
