# Objects

Rekindle has objects, which are a raw representation of data.
They cannot implement traits, nor can they have default functions.
An object's variables are all public and follow the mutability of the object itself.

```rk
object SomeObject {
    some_var: int
    another_var: string
}
```

An object is instantiated like follows

```rk
let some_object = SomeObject {
    some_var = 16
    another_var = "owo"
}
```

Objects are unique compared to other types,
they are inheritly polymorphic with other similar objects.
This is a lot like Typescript's `type`

```rk
object SomeObject {
    a: int
    b: string
}

object AnotherObject {
    a: int
}

fun do_something(obj: AnotherObject) {
    //...
}

fun main() {
    let some_obj = SomeObject {
        a = 15
        b = "owo"
    }
    do_something(some_obj)
}
```
