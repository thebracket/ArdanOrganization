# Telemetry Channel

Let's add another actor for telemetry. Since we have a highly configurable pipeline, it's important to be able to see what's going on.

Let's take a look at `ex12_dataflow4`.

We've:

* Added a `telemetry` actor with a channel to send telemetry events. Currenty, it just prints.
* Modified the ingestor to print telemetry to the console.
* Removed the `sleep` in the ingestor to go as fast as we can.

As you can see, we're hitting the system hard:

```
Ingestor capacity: 100.00%
Ingestor 1 messages per second: 114206.85
Ingestor 0 messages per second: 114514.52
Ingestor 4 messages per second: 113193.05
Ingestor 6 messages per second: 112546.85
Ingestor 5 messages per second: 113573.09
Ingestor 3 messages per second: 112278.95
Ingestor 2 messages per second: 113427.27
Ingestor 7 messages per second: 114066.41
```
