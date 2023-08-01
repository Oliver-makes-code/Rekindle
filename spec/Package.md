# Packages

You can import a file from a path relative to the current file.

```rk
import "path/to/file.rk"
```

You can also qualify it under a namespace,
in case of name clashes.

```rk
import "path/to/file.rk" as SomeFile
```

For importing from libraries provided by a build tool,
you prefix with a colon.

```rk
import ":some-lib"
```

If you need to import from the library root you can do

```rk
import ":some-lib/path/to/file.rk"
```

You can also import libraries from a URL.

```rk
import "https://example.com/rekindle-lib/mod.rk"
```

You can import specific items from a file as follows

```rk
import "some_file.rk".Test

import "owo.rk".{
    SomeNamespace,
    some_func
}

import "uwu.rk".Test as Test2

import "nya.rk".{
    SomeNamespace as AnotherNamespace,
    some_func as another_func
}
```
