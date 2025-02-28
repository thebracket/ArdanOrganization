# Let's Tweak Parameters

Let's tweak a few parameters. You have a *lot* of control over how the program performs (and remember: steps can be asynchronous or synchronous!).

Let's start by reducing the channel sizes. Notice how all of a sudden, output is slower? When you define a *bounded* channel, the default behavior is to *block until you can send*. Especially in async land, this is essentially free. This produces *back pressure*. A data-flow of channels will self-regulate itself to the speed of the slowest link. The rest of the chain will be paused, waiting for the ability to send.

(You can use `try_send` and some deadline options in Flume to not block).

Now start increasing channels, and remove the "sleep" option. Suddenly, it's really fast - channels are incredibly fast, and can handle potentially billions of messages per second.