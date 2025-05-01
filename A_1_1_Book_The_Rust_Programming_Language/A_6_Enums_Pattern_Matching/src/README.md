Important Notes
---

The syntax if let takes a pattern and an expression separated by an equal sign.
It works the same way as a match, where the expression is given to the match
and the pattern is its first arm. In this case, the pattern is Some(max),
and the max binds to the value inside the Some. We can then use max in the
body of the if let block in the same way we used max in the corresponding match
arm. The code in the if let block isn’t run if the value doesn’t match the pattern.

Using if let means less typing, less indentation, and less boilerplate code. However,
you lose the exhaustive checking that match enforces. Choosing between match 
and if let depends on what you’re doing in your particular situation 
and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of if let as syntax sugar for a match that 
runs code when the value matches one pattern and then ignores all other values.