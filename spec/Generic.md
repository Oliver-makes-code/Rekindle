# Generics

We have a meta-type that points to a type,
allowing developers to include them as parameters.

The `typeof` operator creates a meta-type,
that consists of the proceeding traits

```
class Example(type: typeof SomeTrait)
```

The meta-type can be used as a type,
just as normal types can

```
class Example(type: typeof SomeTrait, value: type)
```

To reference something that uses meta-type parameters in an annotation,
you include it in the parenthesis.

```
let something: Example(Foo)
```

Due to this, meta-type parameters can only be at the beginning of arguments.

```
class Example(a: int, b: typeof SomeTrait) // Fails to compile!
```