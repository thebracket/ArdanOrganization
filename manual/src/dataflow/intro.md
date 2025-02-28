# Part 4: Data Flow and Data Driven Design

We've been leading up to this. Many high-performance systems are best modeled around the natural *flow* of data through the system, using types that closely mirror the tasks at hand. Channels and async tasks make connecting the dots quite easy --- and allow for breaking up the monolith at a later date if you need to. You keep each section compartmentalized, making it easy to divide into teams - and you don't sacrifice performance.

So let's model a common workflow:

1. At the top, multiple "ingestors" receive data. They might be receiving events from a Kafka queue, receiving data over the network, or reacting to events from a database.
    * The ingestors have variable input rates.
    * A goal is to ingest as fast as possible and not "block" further ingestion.
2. Ingestors seek to "blast" messages as fast as possible into (one or more) processing queues.
    * The processing queues make batches for calculation - say moving averages.
    * Messages are logged as they are processed.
    * Having a large backlog queue is ok, but the goal is to not stall ingestors.
3. A sidecar processor receives message summaries and logs them in batches.
4. A process prints moving averages.

While this isn't a perfect representation of a real data flow, it's a good example - and many of the common pieces are there.

In this section, we'll build the system - and make some performance notes. We'll also talk a lot about *back pressure*.