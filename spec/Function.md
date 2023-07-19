# Functions

Functions are defined with parameters and an optional return type.

```rk
fun main(args: String[]) i32 {
    return 0
}
```

A function can be used as a variable,
for example, in a lambda

```rk
let someFunc: fun(i32) i32 = |value| {return value}

let otherFunc = main // Resolves to fun(String[]) i32
```

A function can specify a scope,
which allows you to call it on a class, enum, or trait

```rk
fun someFunc(this: SomeClass) {}
```

If you're within a class, enum, or trait,
scope args don't need a type.

```rk
class SomeClass {
    fun someFunc(this) {
        //...
    }
}
```
