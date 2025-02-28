# Multi-Lane Benefits/Downsides

So we've *gained* the ability to keep submitting messages and divide the load between threads. A cache isn't the best choice for this - you'd need locking for a shared cache. HOWEVER - if fetching the items you are caching is resource intensive, that can outweigh the downsides.

This pattern can also work for sidecar telemetry when you need to handle a *lot* of data.

It also lends itself to having multiple teams divide the work. The actor is entirely self-contained, and can easily be shunted to a separate crate. And if necessary in the future, you've got a strong interface for scaling horizontally. You can also control the number of threads (lanes) - giving you configurable performance.

> LibreQoS uses this pattern to handle messages sent from a kernel ringbuffer. Blocking the ringbuffer degrades performance, but there are too many to handle in just one thread.

For telemetry, this is also a great opportunity to coalesce and *batch* data for use further down the pipeline.
