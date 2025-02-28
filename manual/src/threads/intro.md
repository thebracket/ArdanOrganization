# Part 1: Theads

A lot of Rust programs never touch async, and just use threading. Rust 1.0 didn't even have async, and was still a great language.

When you are making a threaded program, you are typically either using threads for:

* A background task that runs separately from the main task.
* Dividing workload to maximize CPU usage in a calculation.

Channels can help with both. You can use channels to send instructions to background channels (for example, submitting results of calculations) - or the other way, use a channel to launch calculation tasks.

You can also use channels to submit data to a side-channel (for example, telemetry) without interrupting the flow of your program.

Channels also lend themselves to a data-flow architecture, which we'll talk about later.