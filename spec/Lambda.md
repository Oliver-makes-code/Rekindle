# Lambdas

We have lambdas which can be used as parameters and to make DSLs

```
fun hasLambda(thing: int, action: fun(int) -> int) {
    action(thing)
}
```

A function with a lambda as the last parameter can have the lambda block outside of the function call

```
hasLambda(15) |value| {
    print(value)
    return 12
}
```

Otherwise, the lambda should be within the function call

```
hasLambda(15, |value| {
    print(value)
    return 12
})
```
