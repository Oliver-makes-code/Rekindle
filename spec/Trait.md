# Traits

We have traits, which are an abstract set of methods and fields.
All fields and methods defined in a trait are public.

```rk
trait SomeTrait {
    let some_field: int
    fun some_func()
}
```

Classes and enums can implement traits,
and external code can implement traits on foreign classes,
and foreign traits on their classes.

```rk
impl SomeTrait for SomeClass {
    let some_field = 15
    fun some_func() {
        someField += 1
    }
}
```

Traits can also me crated as a union of two or more other traits,
containing the properties of the operands.

```rk
trait SomeUnion -> SomeTrait + AnotherTrait
```

A problem that could occur with multiple traits is the overlap of functions and methods,
this can be amended by spcifying the trait you want to operate on.

```rk
something:SomeTrait.method()
```
