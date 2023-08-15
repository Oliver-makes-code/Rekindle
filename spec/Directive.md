# Directives

Rekindle has directives,
which tell the compiler to have specific behavior.
A directive is started with a `$` and has calls a compiler function

For example,
the `when` function only applies the following statement when a certain value is met

```rk
$when(target = js)
fun some_func() {
    //...
}
```
