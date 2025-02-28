# Introduction

This class was created after several training clients asked the same question: "How do I organize data in a large Rust application?".

*Large* is, of course, relative. It's a good idea to worry about your design even in the smallest program. After all---if you're new to the IT industry, you'd be amazed how many "I threw it together in 10 minutes" scripts wind up as vital parts of Production.

![](../images/dependency.png)

(Credit: XKCD, a wonderful web comic)

## Outline

We're going to start by looking at some of the horrible things you might run into---don't worry, we've all been there.

Then we'll start examining how this can hurt us---and alternatives to untangle your code.