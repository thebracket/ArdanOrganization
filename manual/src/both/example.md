# Example

Let's take a look at `ex08_mixed`. It's very much a tale of two cities:

* The `web` module launches an Axum webserver.
    * It serves `OK` to `/health`.
    * It stores a channel.
    * It sends the channel back to the threaded program.
    * If you connect to `/command/message` - it sends that to the sycnhronous world for processing, and sends the reply back.
* The `main` section is all threaded - so no need to worry about the intricacies of async.

Best of all, performance is great. The async world will sit silenty while there's nothing to do - as will the channel. When needed, it comes to life. The threaded system will only handle one task at a time - which is sometimes what you want (you can use `flume` and lanes like we did before if you need more). Often, though - for really heavy lifting, it's more a job server model.