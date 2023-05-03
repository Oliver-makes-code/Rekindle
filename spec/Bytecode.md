# Bytecode and Virtual Machine

We choose to use a custom bytecode format and virtual machine,
in order to create an easy way to run, distribute, and target code

A binary rekindle file has the extension `.rkbin`

Rekindle is Little Endian

## Header

An example header for a binary goes as follows
```
72 6B 64 6C ; Magic number
01 00 ; Rekindle version
xx xx xx xx ; Dictionary location
```

The first four bytes of a rekindle file spell out `rkdl` (`72 6B 64 6C`)

The next two bytes provide the version of rekindle used to compile/run the file

The next four bytes point to the location of the dictionary in the file

## Dictionary

The dictionary points to functions,
variable, classes, and traits in a binary.

An entry in the dictionary consists of a type, a name,
a null terminator for the name, and a file location

```
01 ; Type
6e 61 6d 65 00 ; Name
xx xx xx xx ; location in file
```

01 is a trait, 02 is a class, 03 is a function, and 04 is a variable.
A null byte in the location of the type ends the dictionary