A programming language interpreter made in Rust, written in an idiomatic Rust way with no dependencies outside of std. The method it uses for executing code is Abstract Syntax Tree walking. This is different from a compiler that converts human readable code into computer code. I revised the code to make it as fully featured as possible, while also keeping it simple compared to other interpreters I’ve seen. During the development process, I used unit tests on every part of the interpreter before putting it all together.


Most of important code can be found in the `runtime` folder

Runnable code examples can be found in the `examples` folder

Here's a short runnable code example:
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


