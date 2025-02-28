# Local Globals

In a traditional application (not distributed, networked and maybe single threaded), this is a very traditional application organization pattern. It can work as you scale up, and gives you integration points for future scaling. You don't gain easy rate limiting or resource use management---but it's a good start.

Let's take our cache disaster from the `problems` section, and turn it into a "local global" pattern. We'll put `master_cache` into its own crate---possibly representing that you/your team now have "ownership" of it---and build a client application.
