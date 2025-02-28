# The Problem

Data management is a problem in *every* language. Bill spends a fair amount of time over in Go land offering sage advice for not tying yourself in knots; Python, C, C++, C# - you mame it, this problem comes up.

Even with the best intentions, systems often grow organically.

* You write a one-off tool and discover that its now in Production. Happens to all of us. I have one bespoke tool I threw together for a customer in 2004, and was a little shocked to discover that they still use it everyday!
* A service turns out to be more popular than you expected---so it grows features, maybe becoming distributed.
* A service has been around for a while. People leave, new people arrive. Not everyone shares your initial dedication to clean data---or you inherit someone else's terrible mess.

In other words, even---maybe especially---in IT: life happens.