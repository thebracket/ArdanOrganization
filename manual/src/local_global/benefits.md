# Benefits

Flexibility
Controlled Access
Protection from Abuse

In a lot of programs that won't grow into behemoths---this is enough. I'd encourage *everyone* to use this pattern whenever you have a singleton (other than allocators and signal handlers). Isolate, control access. Program defensively. Assume that poor Joey (to keep picking on our fictional new guy) is going to haplessly accidentally try to misuse any interface you create.

That's all "normal" programming advice.