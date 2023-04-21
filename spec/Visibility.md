# Visability

We go for an immutable-by-default and private-by-default structure,
making developers consciously decide what they what public and mutable.

You can make something mutable by adding it in the type,
and you make something public by adding it in the declaration

```
pub let something: mut = 15 // Type inferred to int

class SomeClass(pub child: mut SomeOtherClass)
```
