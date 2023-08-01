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

There's also the `repr` function,
which determines how a data structure is represented

```rk
$repr(js = class)
class SomeJsClass()

$repr(js = object)
class SomeJsObject()

let a = SomeJsClass()
let b = SomeJsObject()
```

The above code will result in (close to) the following JS code

```js
class SomeJsClass {
    constructor() {}
}

const a = new SomeJsClass()
const b = {}
```
