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
