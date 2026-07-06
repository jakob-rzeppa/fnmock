# Usage

## Generics

Since you need to specify fakes for each combination of generics, make sure to always specify the generics when using the fake. The compiler might infer the wrong types and you are left debugging.

It is also recommended to specify the generics on calls of the faked function, to be sure, the fakes generics match the used ones. For simple functions this might be unnecessary, but with complexity it is more likely the fake implementation will not fake for the used generics.
