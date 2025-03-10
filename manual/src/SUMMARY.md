# Summary

- [Introduction](./intro/intro.md)
    - [Who Am I?](./intro/whoami.md)
    - [Data Management Challenges](./intro/data_management.md)
- [The Problem](./problem/intro.md)
    - [The Temptation of Global State](./problem/global_temptation.md)
    - [The Pitfalls of Global State](./problem/global_pitfall.md)
        - [More Pitfalls](./problem/global_pitfall2.md)
        - [Even More Pitfalls](./problem/global_pitfall3.md)
        - [Pitfalls Everywhere](./problem/global_pitfall4.md)
    - [Borrowing and Lifetimes](./problem/borrow_checker_pain.md)
    - [Singletons](./problem/singletons.md)
    - [Division of Responsiblities](./problem/divisions.md)
- [Local Globals](./local_global/intro.md)
    - [Localize the Global](./local_global/localize.md)
    - [Localize the Global](./local_global/localize2.md)
    - [Localize the Global](./local_global/localize3.md)
    - [Sharding the Cache](./local_global/sharding.md)
    - [Benefits](./local_global/benefits.md)
    - [Downsides](./local_global/downsides.md)
- [Don't Share - Communicate](./dont_share.md)
- [Part 1: Threads](./threads/intro.md)
    - [Aside: How Many Threads Can You Have?](./threads/limits.md)
    - [A Telemetry Sidecar](./threads/logging_sidecar.md)
    - [Shared Data Actor](./threads/actor_cache.md)
    - [Benefits and Downsides of the Actor](./threads/actor_benefits.md)
    - [Multi-Lane Actor](./threads/flume.md)
    - [Multi-Lane Benefits/Downsides](./threads/flume_up_downside.md)
    - [Threaded Actors Summary](./threads/actor_summary.md)
- [Part 2: Async]()
    - [Async and Channels](./async/intro.md)
    - [An Async Telemetry Sidecare](./async/telemetry_sidecar.md)
    - [An Async Single-Lane Cache](./async/actor_cache.md)
    - [Async Wrap-Up](./async/wrap.md)
- [Part 3: Async AND Threads!](./both/intro.md)
    - [Example](./both/example.md)
- [Part 4: Data Flow and Data Driven Design](./dataflow/intro.md)
    - [A Simple Ingestor](./dataflow/ingestor.md)
    - [Starting the Batcher](./dataflow/batcher1.md)
    - [Add Some Processing](./dataflow/processor1.md)
    - [Let's Tweak Parameters](./dataflow/tweaking.md)
    - [Telemetry Channel](./dataflow/telemetry1.md)
    - [Web Telemetry Channel](./dataflow/telemetry2.md)
- [Crate and Module Organization](./org/intro.md)
- [Wrap-Up]()
