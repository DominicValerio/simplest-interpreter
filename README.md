Probably the simplest AST walking interpreter that still has a decent amount of features.

code example
```go
var x = 0
fn add(x, y) {
  x = 2
  return x + y
}

# this is a scope
{
  var y = 2
  x = add(x, y)
}
print(x) # prints 4
```

bugs
- Token.ln doesn't update when a newline is in a string literal
- a string literal can't have escape characters
- When a program is interpreted, the line in the current token isn't updated
- not every valid operation is valid
