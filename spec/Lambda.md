# Lambdas

We have lambdas which can be used as parameters and to make DSLs

```rk
fun has_lambda(thing: int, action: fun(int) int) {
    action(thing)
}
```

A function with a lambda as the last parameter can have the lambda block outside of the function call

```rk
has_lambda(15) |value| {
    print(value)
    return 12
}
```

Otherwise, the lambda should be within the function call

```rk
has_lambda(15, |value| {
    print(value)
    return 12
})
```
