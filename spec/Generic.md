# Generics

Generics are defined with the `type` keyword.
They allow things to take in types as parameters.

```rk
fun some_func(type T; t: T) {
    // do stuff with t
}
```

They can also be bounded,
allowing you to specify what you need to call it

```rk
fun default(type T: Default) T {
    return T.default()
}
```

You can specify them outside parameters to use them as a type alias

```rk
type SomeType: Array(int)
```

Type aliases can also have parameters

```rk
type Holder(Element): Either(Map(string, Element), Array(Element))
```

They can be used in functions and similar like such

```rk
fun get_values(type Element; holder: Holder(Element)) Array(Element) {
    when holder {
        First(map) {
            return map.values()
        }
        Second(arr) {
            return arr
        }
    }
}
```
