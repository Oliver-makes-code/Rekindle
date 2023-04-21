# Functions

Functions are defined with parameters and an optional return type.

```
fun main(args: String[]): int {
    return 0
}
```

A function can be used as a variable,
for example, in a lambda

```
let someFunc: fun(int) int = |value| {return value}

let otherFunc = main // Resolves to fun(String[]) int
```