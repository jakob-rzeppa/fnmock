# Features

What `#[fnmock::fakeable]` currently supports for **fakes**. Every entry below is backed by a
test in [fnmock-tests/src/fake/](fnmock-tests/src/fake/); unsupported cases are backed by
`trybuild` snapshots in [unsupported/compile_fail/](fnmock-tests/src/fake/unsupported/compile_fail/).

## How it works

Applying the attribute to a function or an inherent impl block:

1. injects a `#[cfg(test)]` fake lookup at the top of the original body — if a fake is set, it is
   called and its result returned; otherwise the real body runs,
2. generates an accessor named `<fn_name>_fake()`,
3. generates a hidden module holding the fake in `thread_local!` storage.

```rust
#[fnmock::fakeable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}

#[test]
fn test() {
    greet_fake().setup(|name| format!("Fake {}", name));
    assert_eq!(greet("Test".to_string()), "Fake Test");
}
```

The lookup is `#[cfg(test)]`-gated, so non-test builds carry no fake machinery and no runtime
overhead. The accessor is emitted as `#[cfg(test)] pub(crate)`, so it can be called from a
different module than the one defining the item ([visibility.rs](fnmock-tests/src/fake/visibility.rs)).

## Accessor API

| Method | Behaviour |
| --- | --- |
| `setup(closure)` | Install a fake. Calling it again overwrites the previous fake. |
| `clear()` | Remove the fake; subsequent calls run the real implementation again. |
| `is_set()` | Whether a fake is currently installed. |

The closure mirrors the function signature: same parameters, same return type. For methods the
receiver is passed as the **first** closure argument (`|_, a, b|`). For `async` functions the
closure is a plain synchronous closure returning the output type — not a future.

## Free functions

### Parameter and return types

| Supported | Test |
| --- | --- |
| By-value parameters | [by_value.rs](fnmock-tests/src/fake/basic/by_value.rs) |
| Shared references (`&str`) | [reference.rs](fnmock-tests/src/fake/basic/reference.rs) |
| Mutable references (`&mut String`) | [mut_reference.rs](fnmock-tests/src/fake/basic/mut_reference.rs) |
| References nested in containers (`Option<&T>`, `Vec<&T>`, `&[T]`, tuples of refs) | [reference_in_container.rs](fnmock-tests/src/fake/basic/reference_in_container.rs) |
| Smart pointers (`Box<T>`) | [smart_pointers.rs](fnmock-tests/src/fake/basic/smart_pointers.rs) |
| Raw pointers (`*const T`, `*mut T`) | [raw_const_pointers.rs](fnmock-tests/src/fake/basic/raw_const_pointers.rs), [raw_mut_pointers.rs](fnmock-tests/src/fake/basic/raw_mut_pointers.rs) |
| Interior mutability (`RefCell`, `Cell`, …) | [interior_mutability.rs](fnmock-tests/src/fake/basic/interior_mutability.rs) |
| Returning `Option<T>` / `Result<T, E>` / `()` | [return_option.rs](fnmock-tests/src/fake/basic/return_option.rs), [return_result.rs](fnmock-tests/src/fake/basic/return_result.rs), [return_unit.rs](fnmock-tests/src/fake/basic/return_unit.rs) |
| Boxed / referenced trait objects, auto traits (`Box<dyn Send>`) | [trait_based/](fnmock-tests/src/fake/trait_based/) |
| Returning boxed futures (`Pin<Box<dyn Future>>`) | [futures.rs](fnmock-tests/src/fake/special/futures.rs) |

### Parameter patterns

| Supported | Test |
| --- | --- |
| Tuple destructuring `(left, right): (String, String)` | [tuple_destructuring.rs](fnmock-tests/src/fake/patterns/tuple_destructuring.rs) |
| Nested tuple destructuring | [nested_tuple_destructuring.rs](fnmock-tests/src/fake/patterns/nested_tuple_destructuring.rs) |
| Slice destructuring | [slice_destructuring.rs](fnmock-tests/src/fake/patterns/slice_destructuring.rs) |
| `mut` bindings, incl. inside tuple patterns | [mutable_patterns.rs](fnmock-tests/src/fake/patterns/mutable_patterns.rs) |

The pattern is reproduced in the fake closure, so `setup(|(left, right)| …)` destructures the
same way the real function does.

### Function modifiers

`async fn`, `unsafe fn`, and `extern "C" fn` are all supported — see
[special/](fnmock-tests/src/fake/special/).

## Generics

Generic functions get a per-instantiation fake store, so the accessor takes a turbofish and each
combination of generic arguments is faked independently:

```rust
#[fnmock::fakeable]
fn identity<T: 'static>(a: T) -> T { a }

identity_fake::<String>().setup(|a| format!("Fake {}", a));
```

| Supported | Test |
| --- | --- |
| Single and multiple type parameters | [single_generic.rs](fnmock-tests/src/fake/generics/single_generic.rs), [multiple_generics.rs](fnmock-tests/src/fake/generics/multiple_generics.rs) |
| Generic return types | [return_generic.rs](fnmock-tests/src/fake/generics/return_generic.rs) |
| Unused type parameters | [unused_generic.rs](fnmock-tests/src/fake/generics/unused_generic.rs) |
| Trait bounds, inline and in `where` clauses, mixed | [bounds_generic.rs](fnmock-tests/src/fake/generics/bounds_generic.rs), [bounds_mixed.rs](fnmock-tests/src/fake/generics/bounds_mixed.rs) |
| `where` clauses on non-parameter types | [non_parameter_where.rs](fnmock-tests/src/fake/generics/non_parameter_where.rs) |
| Associated type bounds and equality (`I: Iterator<Item = u8>`) | [associated_type_bounds.rs](fnmock-tests/src/fake/generics/associated_type_bounds.rs), [associated_type_equality.rs](fnmock-tests/src/fake/generics/associated_type_equality.rs) |
| Higher-ranked bounds (`for<'a> Fn(&'a str) -> &'a str`) | [higher_ranked_bounds.rs](fnmock-tests/src/fake/generics/higher_ranked_bounds.rs) |
| Lifetimes: named, implicit, `'static`, multiple, unused | [named_lifetime.rs](fnmock-tests/src/fake/generics/named_lifetime.rs), [implicit_lifetime.rs](fnmock-tests/src/fake/generics/implicit_lifetime.rs), [static_lifetime.rs](fnmock-tests/src/fake/generics/static_lifetime.rs), [multiple_lifetimes.rs](fnmock-tests/src/fake/generics/multiple_lifetimes.rs), [unused_lifetime.rs](fnmock-tests/src/fake/generics/unused_lifetime.rs) |
| Const generics, incl. multiple and mixed with types | [const_generics.rs](fnmock-tests/src/fake/generics/const_generics.rs), [const_generics_multiple.rs](fnmock-tests/src/fake/generics/const_generics_multiple.rs), [const_generics_mixed.rs](fnmock-tests/src/fake/generics/const_generics_mixed.rs) |

Type parameters must be `'static` — fakes are keyed by `TypeId`. Const parameters are keyed by
value, so `foo_fake::<5>()` does not affect a call to `foo::<7>()`. The const value is not
accessible inside the fake closure; since the fake only applies to that one value, hardcode it.

Because the fake is selected by generic arguments, **always specify them explicitly** on both
`setup` and the call site. If the compiler infers different arguments than you expected, the fake
silently will not apply and the real implementation runs. See [USAGE.md](USAGE.md).

## Impl blocks

Applying the attribute to an inherent impl block makes every method in it fakeable, each with its
own store. The accessor is an associated function:

```rust
#[fnmock::fakeable]
impl BasicStruct {
    fn basic(&self) -> i32 { 42 }
}

BasicStruct::basic_fake().setup(|_| 5);
```

| Supported | Test |
| --- | --- |
| Methods and associated functions (no receiver) | [basic.rs](fnmock-tests/src/fake/impl_block/basic.rs), [associated_function.rs](fnmock-tests/src/fake/impl_block/associated_function.rs) |
| Multiple methods per block, each faked independently | [multiple_methods.rs](fnmock-tests/src/fake/impl_block/multiple_methods.rs) |
| Same method name on different types, kept isolated | [same_method_name_isolation.rs](fnmock-tests/src/fake/impl_block/same_method_name_isolation.rs) |
| `async` methods | [async_method.rs](fnmock-tests/src/fake/impl_block/async_method.rs) |
| Receivers: `&self`, `&mut self`, `self`, `Box<Self>`, `Rc<Self>`, `Pin<&mut Self>`, explicit `self: Type` | [receiver/](fnmock-tests/src/fake/impl_block/receiver/) |
| Returning `Self`, `&Self`, `Option<Self>`, `Result<Self, _>`, `()` | [receiver/](fnmock-tests/src/fake/impl_block/receiver/) |
| Generic structs, generic methods, and both combined (incl. `where` clauses and lifetimes) | [impl_block/generics/](fnmock-tests/src/fake/impl_block/generics/) |

Generic arguments are split across the two positions they belong to — struct generics on the
type, method generics on the accessor:

```rust
GenericCombined::<String>::combine_fake::<i32>().setup(|_, other| ("Fake".to_string(), other * 2));
```

## Isolation semantics

- **Thread isolation.** Fakes live in `thread_local!` storage. A fake set on one thread is invisible
  to another, and a fake set inside a spawned thread does not leak back to the caller. Tests running
  in parallel therefore do not interfere ([thread_isolation.rs](fnmock-tests/src/fake/basic/thread_isolation.rs)).
- **Per-instantiation isolation.** For generics, setting or clearing one instantiation leaves the
  others untouched ([cross_type_isolation.rs](fnmock-tests/src/fake/generics/cross_type_isolation.rs)).
- **No automatic reset.** A fake stays installed for the rest of the thread's life unless you call
  `clear()`.

## Not supported

Each of these fails at compile time with a dedicated error message.

| Not supported | Reason given |
| --- | --- |
| `const fn` / `const` methods | The fake lookup fnmock injects cannot run in a const context. |
| Trait impl blocks (`impl Trait for Type`) | Only inherent impl blocks are supported. |
| Impl blocks on non-path types | Only simple paths (+generics) are supported. |
| The attribute on anything but a function or impl block | Can only be applied to functions and impl blocks. |
| `impl Trait` in argument or return position | Use a concrete type or a generic type parameter instead. |
| The inferred type `_` in a signature | Specify the type explicitly. |
| The never type `!` | — |
| Non-`'static` lifetime bounds on generic parameters | Only `'static` is supported in generic parameters. |
| `self` receiver on a free function | Only supported on methods inside an inherent impl block. |
| Wildcard parameter patterns (`_: i32`) | Fake call values need a name to forward. |
| Reference patterns (`&x: &i32`) | Use a plain binding (`x: &i32`) instead. |
| `ref` bindings (`ref x`) | Use the identifier directly without `ref`. |
| Struct and tuple-struct destructuring patterns | — |
| Macro-generated parameter patterns | — |

Note the distinction between reference *types* and reference *patterns*: `x: &i32` is supported,
`&x: &i32` is not.
