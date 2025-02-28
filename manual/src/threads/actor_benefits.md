# Benefits and Downsides of the Actor

> For this to be performant, remove the "reply" part of the "set" call!

There's an immediate obvious benefit: There is no need for locking! Because you never have multiple accessors, you can *never* have concurrent access - so there's no locking at all.

There's a second, hidden benefit: the calls into the actor closely mimic the setup for a gRPC service. So (follow the Services class!) it's relatively easy to transparently scale horizontally now.

There is, however, a downside as well - and its the same as the upside. It's single access, so it can easily become the bottleneck.