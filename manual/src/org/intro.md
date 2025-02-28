# Crate and Module Organization

Another question that comes up quite often in training is "how should I organize projects?" We've touched on this in other trainings, and it depends a lot upon your team structure - but there's some good general advice.

## Use A Workspace

Setup a workspace as the top-level structure. Setup your workspace in the top-level crate's `Cargo.toml`:

```toml
[workspace]
members = [
	"timed_lru_cache", # Cache library - used in multiple examples
	"public_global_app", "ex01_local_globals", "ex02_sharding_local", "ex03_thread_telemetry", "ex04_thread_actor_cache", "ex05_thread_flume", "ex06_async_telemetry", "ex07_async_actor", "ex08_mixed", "ex09_dataflow1", "ex10_dataflow2", "ex11_dataflow3", "ex12_dataflow4", "ex13_dataflow5", # First example application
]
```

Commenting your members is a *great* idea.

Put all your dependencies into:

```toml
[workspace.dependencies]
```

And reference dependencies at the crate level with:

```toml
[dependencies]
serde.workspace = true
```

Now *every* crate in your workspace will share dependencies, download and build them once (unless you start changing feature flags per crate). This *greatly* helps with build time, disk space and organization. It also makes it easy to see what you are depending upon, they are now all in one place.

## Use Local Crates

Rust compile time tends to get painful, and having different teams or team-members writing to the same files tends to be organizationally difficult. Fortunately, the answer for both is to divide your program into modules - and put modules into separate crates. This *also* forces you to be a bit more careful with inter-module dependencies---and that's a GOOD thing moving forward. 

> Less spaghetti = happy maintainers.

This can and should evolve over time. For example, in the data-flow examples it's all in one crate. It doesn't need to be, you could easily move each of the systems into separate crates and couple only with channels and `main` starting everything. As systems grow, that's exactly what you should do.

Reference local crates as:

```toml
[dependencies]
local_crate = { path = "../local_crate" }
```

> Don't be afraid to use directories to keep things organized; the paths are examples.

My rule of thumb is that if I add something to a personal project and find myself grumbling about compilation time, it needs to be moved into a crate.