# Aside: How Many Threads Can You Have?

On Linux, run:

```bash
cat /proc/sys/kernel/threads-max
```

On my workstation, I'm currently limited to 254,585 threads per process. That's a *lot*. Actually using that many will take a *lot* of memory, but it can work.

So don't worry too much when you need to make tens of low-utilization threads.

DO worry if you make more HIGH utilization threads than you have available CPU cores.