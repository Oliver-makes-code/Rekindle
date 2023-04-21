# Lambdas

We have lambdas which can be used as parameters and to make DSLs

```kt
fun hasLambda(thing: int, action: fun(int) int) {
    action(thing)
}

fun useLambda() {
    hasLambda(15) |value| {
        print(value)
        return 12
    }
}
```
