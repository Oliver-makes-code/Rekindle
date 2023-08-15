# Functions

Functions are defined with parameters and an optional return type.

```rk
fun main(args: Array(String)) int {
    return 0
}
```

A function can be used as a variable,
for example, in a lambda

```rk
let some_func: fun(int) int = |value| {return value}

let other_func = main // Resolves to fun(Array(String)) int
```

A function can specify a scope,
which allows you to call it on a class, enum, or trait

```rk
fun some_func(this: SomeClass) {}
```

If you're within a class, enum, or trait,
scope args don't need a type.

```rk
class SomeClass {
    fun some_func(this) {
        //...
    }
}
```

Functions with an array type as the last argument are treated as variable length argument.
They can be called using an array or a list of parameters.

```rk
fun some_func(something: Array(int)) {
    //...
}

fun main() {
    some_func(1, 2, 3)
    some_func([1, 2, 3])
    let a: fun(Array(int)) = some_func
    a(1, 2, 3)
    a([1, 2, 3])
}
```
