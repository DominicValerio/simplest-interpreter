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

var count = 0
while count < 10000 {
	count = count + 1
}
```


