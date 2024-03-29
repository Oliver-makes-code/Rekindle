# Classes

We use the concept of classes to store and access data.
A class at it's most basic defines a name

```rk
class Example()
```

A class can define methods and fields.

```rk
class Example() {
    let someVar = 15

    pub fun some_func() {
        print("owo")
    }
}
```

A class can implement a trait,
allowing functions requiring a specific trait to use it.

```rk
class Example() {
    impl ExampleTrait {
        fun log() {
            print("owo")
        }
    }
}
```

This might be a bit too many indents for some,
so we allow the use of `impl x for y` and `defer x for y`

We also allow developers to implement traits on foreign classes,
But you're only allowed access to public fields and methods.

```rk
impl ExampleTrait for Example {
    fun log() {
        print("owo")
    }
}

defer AnotherTrait for Example -> child
```

We do not have a standard inheritance structure,
instead deffering traits to children,
which are defined in the constructor.

If you defer a trait to a child,
you cannot override any methods in said trait.

Children are much like fields, however,
fields cannot be deferred to traits,
like children can.

```rk
class Example(child: ChildClass) {
    defer ExampleTrait -> child
}
```

You can also defer specific methods/fields in a trait to children

```rk
class Example(child: ChildClass) {
    impl ExampleTrait {
        defer let some_var -> child
        defer fun some_func() -> child
    }
}
```

If you only want to implement a couple functions in a trait,
and want to defer the rest to a child, you can `defer * -> x`

```rk
class Example(child: ChildClass) {
    impl ExampleTrait {
        let some_var = 15
        defer * -> child
    }
}
```

You can create inline classes as well.
This acts the same as normal classes,
but with the ability to dynamically generate impls.

```rk
let Example = class() {
    //...
}
```

Classes can define a constructor override,
allowing more intricate behaviour in class initialization.

```rk
class Example(some_child: AnotherClass) {
    new(some_child: AnotherClass, thing: int) this(someChild) {
        print("awoo! " + thing)
    }
}
```

This overwrites the behaviour from the original constructor,
meaning you have to use the override

```rk
Example(AnotherClass()) // Fails to compile

Example(AnotherClass(), 15) // Compiles
```

Casting takes two main forms,
direct and indirect.
Direct casting fails if the type is not assignable.
Indirect casting returns `None` when the type is not assignable.

```rk
// Direct
value:Type
value as Type

// Indirect
value:?Type
value as? Type
```

Classes can declare static variables by using the `global` keyword

```rk
class SomeClass {
    global let some_global
}
```

Accessing a static value (function, field, etc) from yourself,
you need to use `This` (capital T)

```rk
class SomeClass {
    fun some_fun() {
        //stuff
    }

    fun another_fun() {
        This.some_fun()
    }
}
```
