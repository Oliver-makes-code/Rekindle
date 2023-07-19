# Traits

We have traits, which are an abstract set of methods and fields.
All fields and methods defined in a trait are public.

```rk
trait SomeTrait {
    let someField: i32
    fun someFunc()
}
```

Classes and enums can implement traits,
and external code can implement traits on foreign classes,
and foreign traits on their classes.

```rk
impl SomeTrait for SomeClass {
    let someField = 15
    fun someFunc() {
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
