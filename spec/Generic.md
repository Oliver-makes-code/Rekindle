# Generics

We have a meta-type that points to a type,
allowing developers to include them as parameters.

The `typeof` operator creates a meta-type,
that consists of the proceeding traits

```rk
class Example(type: typeof SomeTrait)
```

The meta-type can be used as a type,
just as normal types can.
Meta-types intended to be used in type annotations need to be separated from other parameters by a semicolon.

```rk
class Example(type: typeof SomeTrait; value: type)
```

To reference something that uses meta-type parameters in an annotation,
you include it in the parenthesis.

```rk
let something: Example(Foo)
```

Meta-types can be used as return parameters,
for a way to create type generators.

```rk
fun create() typeof SomeTrait {
    return class() {
        impl SomeTrait {
            //...
        }
    }
}
```

The compiler will not infer meta-types,
as it can cause unintended bugs where you expected an instance.

```rk
let thing = SomeClass // Doesn't compile

let thing: typeof SomeTrait = SomeClass // Compiles
```
