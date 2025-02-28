# Downsides

This is a perfectly servicable pattern *if* you are using it inside a system that already controls its concurrency levels (constrained number of threads).

There are some downsides:

* You're writing a lot of access functions. You may have to do that anyway, but your access functions are becoming quite coupled to what you need.
* Not individually scalable vertically.
* Not obviously scalable horizontally.

You've got relatively loose coupling, and a much-better isolated API. If this system is going to be invoked only from other parts of your system that are *already* scaled---it'll be just fine. If you need more, then you need to consider other patterns.
