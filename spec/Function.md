# Functions

Functions are defined with parameters and an optional return type.

```rk
fun main(args: String[]) int {
    return 0
}
```

A function can be used as a variable,
for example, in a lambda

```rk
let some_func: fun(int) int = |value| {return value}

let other_func = main // Resolves to fun(String[]) int
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
