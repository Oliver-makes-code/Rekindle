# Lambdas

We have lambdas which can be used as parameters and to make DSLs

```rk
fun hasLambda(thing: i32, action: fun(i32) i32) {
    action(thing)
}
```

A function with a lambda as the last parameter can have the lambda block outside of the function call

```rk
hasLambda(15) |value| {
    print(value)
    return 12
}
```

Otherwise, the lambda should be within the function call

```rk
hasLambda(15, |value| {
    print(value)
    return 12
})
```
