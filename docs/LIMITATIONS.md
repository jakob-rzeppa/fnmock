# Support matrix and limitations

Every construct fnmock has been tested against, with a column for `#[fnmock::fakeable]`, one for
`#[fnmock::spyable]` and one for `#[fnmock::mockable]`. This is the shared reference: everything
under [`fnmock-tests/src/common/`](../fnmock-tests/src/common/) is here. The feature docs only cover
what is specific to one macro — the accessor it generates and the methods you call on it:

- [FEATURES.md](FEATURES.md) — the `_mock()` accessor: `setup` / `is_set`, `expect` /
  `expectf` / `assert`, sequences and `clear`.
- [FAKE.md](FAKE.md) — the `_fake()` accessor and how it differs from a mock.
- [SPY.md](SPY.md) — the `_spy()` accessor and how it differs from a mock.

**How to read the tables.** Every cell links to the test that backs it:

| Symbol | Meaning |
| --- | --- |
| ✅ | Supported. The link is an ordinary test — a `#[test]`, or a module that only has to compile. |
| ❌ | Rejected at compile time with a dedicated error message. The link is a [`trybuild`](https://docs.rs/trybuild) compile-fail fixture (a `.cf.rs` file) whose exact error is pinned in the `.cf.stderr` snapshot next to it. |
| ⚠️ | Accepted, but restricted in a way that is not a compile error. See the note under the table. |
| — | Not applicable to this macro. |

Where one shared test file exercises all three macros it has a `mod fake`, a `mod spy` and a
`mod mock`; a single link in every column points at that file. Run the whole set with
`cargo test -p fnmock-tests`.

The fake and the spy mostly agree. They diverge only where their mechanics force it: `#[fakeable]`
has to name the return type to bound its fake closure, and `#[spyable]` has to name every parameter
to match on it. Those rows are called out as they come up.

A **mock** is a fake and a spy on the same item, so it supports the **intersection** of the two:
both halves run, and whichever rejects a construct first (the fake, then the spy) reports its own
error message. Every row where the fake and the spy disagree is therefore ❌ in the Mocks column, and
so is any construct either half rejects on its own.

## Contents

- [What you can apply the attribute to](#what-you-can-apply-the-attribute-to)
- [Function modifiers](#function-modifiers)
- [Parameter types](#parameter-types)
- [Parameter patterns](#parameter-patterns)
- [Return types](#return-types)
- [Trait objects](#trait-objects)
- [Generics](#generics)
- [Impl blocks](#impl-blocks)
- [Visibility](#visibility)
- [Other attributes on the item](#other-attributes-on-the-item)
- [Isolation](#isolation)

## What you can apply the attribute to

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| A free function | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | — |
| An inherent impl block | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) | — |
| A trait impl block (`impl Trait for Type`) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_trait_impl_block_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_trait_impl_block_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_trait_impl_block_fake.cf.rs) | Only inherent impl blocks are supported. |
| An impl block on a non-path type | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_impl_on_non_path_type_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_impl_on_non_path_type_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_impl_on_non_path_type_fake.cf.rs) | Only simple paths (plus generics) are supported. |
| Anything but a function or an impl block (a struct, …) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_on_struct_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_on_struct_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_on_struct_fake.cf.rs) | Can only be applied to functions and impl blocks. |

## Function modifiers

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| `async fn` | ✅ [test](../fnmock-tests/src/common/special/async_function.rs) | ✅ [test](../fnmock-tests/src/common/special/async_function.rs) | ✅ [test](../fnmock-tests/src/common/special/async_function.rs) | — |
| `async fn` with generics | ✅ [test](../fnmock-tests/src/common/special/async_generic_function.rs) | ✅ [test](../fnmock-tests/src/common/special/async_generic_function.rs) | ✅ [test](../fnmock-tests/src/common/special/async_generic_function.rs) | — |
| `unsafe fn` | ✅ [test](../fnmock-tests/src/common/special/unsafe_function.rs) | ✅ [test](../fnmock-tests/src/common/special/unsafe_function.rs) | ✅ [test](../fnmock-tests/src/common/special/unsafe_function.rs) | — |
| `extern "C" fn` | ✅ [test](../fnmock-tests/src/common/special/extern_function.rs) | ✅ [test](../fnmock-tests/src/common/special/extern_function.rs) | ✅ [test](../fnmock-tests/src/common/special/extern_function.rs) | — |
| `const fn` | ❌ [fixture](../fnmock-tests/src/common/special/unsupported_const_function_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/special/unsupported_const_function_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/special/unsupported_const_function_mock.cf.rs) | The code the macro injects — a `thread_local!` lookup or a call recording — cannot run in a const context. |
| A `const` method in an impl block | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_const_method_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_const_method_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/impl_block/special/unsupported_const_method_fake.cf.rs) | Same. |

## Parameter types

All three macros have to name every parameter type in generated code.

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| By-value parameters | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | — |
| Mixed by-value and by-reference parameters | ✅ [test](../fnmock-tests/src/common/params/by_ref_and_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_ref_and_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_ref_and_value.rs) | — |
| Zero-argument functions | ✅ [test](../fnmock-tests/src/common/params/zero_args.rs) | ✅ [test](../fnmock-tests/src/common/params/zero_args.rs) | ✅ [test](../fnmock-tests/src/common/params/zero_args.rs) | — |
| Shared references (`&str`) | ✅ [test](../fnmock-tests/src/common/params/reference.rs) | ✅ [test](../fnmock-tests/src/common/params/reference.rs) | ✅ [test](../fnmock-tests/src/common/params/reference.rs) | — |
| Mutable references (`&mut String`) | ✅ [test](../fnmock-tests/src/common/params/mut_reference.rs) | ✅ [test](../fnmock-tests/src/common/params/mut_reference.rs) | ✅ [test](../fnmock-tests/src/common/params/mut_reference.rs) | — |
| Slices (`&[T]`) | ✅ [test](../fnmock-tests/src/common/params/slice.rs) | ✅ [test](../fnmock-tests/src/common/params/slice.rs) | ✅ [test](../fnmock-tests/src/common/params/slice.rs) | — |
| References nested in `Option`, `Vec`, slices, tuples | ✅ [option](../fnmock-tests/src/common/params/reference_in_option.rs), [vec](../fnmock-tests/src/common/params/reference_in_vec.rs), [slice](../fnmock-tests/src/common/params/reference_in_slice.rs), [tuple](../fnmock-tests/src/common/params/reference_in_tuple.rs) | ✅ [option](../fnmock-tests/src/common/params/reference_in_option.rs), [vec](../fnmock-tests/src/common/params/reference_in_vec.rs), [slice](../fnmock-tests/src/common/params/reference_in_slice.rs), [tuple](../fnmock-tests/src/common/params/reference_in_tuple.rs) | ✅ [option](../fnmock-tests/src/common/params/reference_in_option.rs), [vec](../fnmock-tests/src/common/params/reference_in_vec.rs), [slice](../fnmock-tests/src/common/params/reference_in_slice.rs), [tuple](../fnmock-tests/src/common/params/reference_in_tuple.rs) | For a spy or a mock the nested reference has to be spelled `&'static` — an anonymous `&` inside a container cannot be named in the matcher's params, so the same signature that a fake accepts is rejected by the spy half. The linked spy and mock tests use `&'static`. |
| Smart pointers (`Box<T>`) | ✅ [test](../fnmock-tests/src/common/params/smart_pointers.rs) | ✅ [test](../fnmock-tests/src/common/params/smart_pointers.rs) | ✅ [test](../fnmock-tests/src/common/params/smart_pointers.rs) | — |
| Raw pointers (`*const T`, `*mut T`) | ✅ [const](../fnmock-tests/src/common/params/raw_const_pointers.rs), [mut](../fnmock-tests/src/common/params/raw_mut_pointers.rs) | ✅ [const](../fnmock-tests/src/common/params/raw_const_pointers.rs), [mut](../fnmock-tests/src/common/params/raw_mut_pointers.rs) | ✅ [const](../fnmock-tests/src/common/params/raw_const_pointers.rs), [mut](../fnmock-tests/src/common/params/raw_mut_pointers.rs) | — |
| Interior mutability (`RefCell`, `Cell`, …) | ✅ [test](../fnmock-tests/src/common/params/interior_mutability.rs) | ✅ [test](../fnmock-tests/src/common/params/interior_mutability.rs) | ✅ [test](../fnmock-tests/src/common/params/interior_mutability.rs) | — |
| Implicitly elided lifetimes | ✅ [test](../fnmock-tests/src/common/params/implicit_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/params/implicit_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/params/implicit_lifetime.rs) | — |
| Parameter names that collide with generated identifiers (`f`, `params`, `function`) | ✅ [test](../fnmock-tests/src/common/params/names_shadowing_generated_idents.rs) | ✅ [test](../fnmock-tests/src/common/params/names_shadowing_generated_idents.rs) | ✅ [test](../fnmock-tests/src/common/params/names_shadowing_generated_idents.rs) | — |
| `impl Trait` in argument position | ❌ [fixture](../fnmock-tests/src/common/traits/unsupported_impl_trait_param_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/traits/unsupported_impl_trait_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/traits/unsupported_impl_trait_param_fake.cf.rs) | `impl Trait` is anonymous, so neither the fake closure bound nor the spy's `Predicate<..>` can name it. Use a concrete type or a generic type parameter. |
| The inferred type `_` in a signature | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_inferred_param_type_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_inferred_param_type_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_inferred_param_type_fake.cf.rs) | Specify the type explicitly. |
| A `self` receiver on a free function | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_self_in_free_function_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_self_in_free_function_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_self_in_free_function_fake.cf.rs) | Only supported on methods inside an inherent impl block. |

## Parameter patterns

A fake reproduces the parameter pattern in its closure, so it accepts any pattern it can also
forward. A spy records one *named* value per parameter, so it needs a plain identifier and rejects
everything that destructures. This is the theme where the two macros diverge the most.

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| A plain identifier (`a: String`) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | ✅ [test](../fnmock-tests/src/common/params/by_value.rs) | — |
| `mut` bindings (`mut val: String`) | ✅ [test](../fnmock-tests/src/common/params/patterns/mutable.rs) | ✅ [test](../fnmock-tests/src/common/params/patterns/mutable.rs) | ✅ [test](../fnmock-tests/src/common/params/patterns/mutable.rs) | — |
| Tuple destructuring (`(left, right): (String, String)`) | ✅ [test](../fnmock-tests/src/common/params/patterns/tuple_destructuring.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_tuple_destructuring_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_tuple_destructuring_mock.cf.rs) | The fake reproduces the pattern in its closure; the spy has no name to match the parameter under. |
| Nested tuple destructuring | ✅ [test](../fnmock-tests/src/common/params/patterns/tuple_destructuring_nested.rs) | ❌ (as above) | ❌ (as above) | Same. |
| `mut` inside a tuple pattern | ✅ [test](../fnmock-tests/src/common/params/patterns/mutable_nested.rs) | ❌ (the tuple pattern is what is rejected) | ❌ (the tuple pattern is what is rejected) | Same. |
| Slice / array destructuring (`[a, b]: [i32; 2]`) | ✅ [test](../fnmock-tests/src/common/params/patterns/slice_destructuring.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_slice_destructuring_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_slice_destructuring_mock.cf.rs) | Same. |
| Wildcard patterns (`_: i32`) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_wildcard_param_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_wildcard_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_wildcard_param_fake.cf.rs) | Call values need a name to forward. |
| Reference patterns (`&x: &i32`) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_reference_param_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_reference_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_reference_param_fake.cf.rs) | Use a plain binding (`x: &i32`) instead. |
| `ref` bindings (`ref x`) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_ref_pattern_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_ref_pattern_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_ref_pattern_fake.cf.rs) | Use the identifier directly, without `ref`. |
| Struct destructuring (`Point { x, y }: Point`) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_struct_destructuring_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_struct_destructuring_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_struct_destructuring_fake.cf.rs) | For a fake, forwarding it would require reassembling the struct, which is not always possible. For a spy, there is no name to match it under. |
| Tuple-struct destructuring (`Wrapper(x): Wrapper`) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_tuple_struct_destructuring_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_tuple_struct_destructuring_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/patterns/unsupported_tuple_struct_destructuring_fake.cf.rs) | Same. |
| Macro-generated patterns (`m!(): i32`) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_macro_param_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_macro_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/params/unsupported_macro_param_fake.cf.rs) | Forwarding the parameter cannot be inferred in general. |

Note the distinction between reference *types* and reference *patterns*: `x: &i32` is supported by
all three macros, `&x: &i32` by none.

## Return types

A fake has to *produce* a return value, so its return type must be one the generated closure bound
can name. A spy never produces one, so it does not care what the function returns — this is the
theme where `#[spyable]` is wider.

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| `Option<T>` | ✅ [test](../fnmock-tests/src/common/returns/return_option.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_option.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_option.rs) | — |
| `Result<T, E>` | ✅ [test](../fnmock-tests/src/common/returns/return_result.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_result.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_result.rs) | — |
| `()` | ✅ [test](../fnmock-tests/src/common/returns/return_unit.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_unit.rs) | ✅ [test](../fnmock-tests/src/common/returns/return_unit.rs) | — |
| Generic return types | ✅ [test](../fnmock-tests/src/common/generics/return_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/return_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/return_generic.rs) | — |
| A generic parameter that appears **only** in the return type | ✅ [test](../fnmock-tests/src/common/generics/return_generic.rs) | ✅ [test](../fnmock-tests/src/spy/generics/generic_only_in_return.rs) | ✅ [test](../fnmock-tests/src/common/generics/return_generic.rs) | — |
| Boxed futures (`Pin<Box<dyn Future>>`) | ✅ [test](../fnmock-tests/src/common/special/futures.rs) | ✅ [test](../fnmock-tests/src/common/special/futures.rs) | ✅ [test](../fnmock-tests/src/common/special/futures.rs) | — |
| `impl Trait` in return position | ❌ [fixture](../fnmock-tests/src/common/traits/unsupported_impl_trait_return_fake.cf.rs) | ✅ [test](../fnmock-tests/src/common/traits/impl_trait_return.rs) | ❌ [fixture](../fnmock-tests/src/common/traits/unsupported_impl_trait_return_mock.cf.rs) | The fake closure's bound has to name the return type; a spy has no return type to name. |
| The never type `!` as a return type | ❌ [fixture](../fnmock-tests/src/common/returns/unsupported_never_return_type_fake.cf.rs) | ✅ [test](../fnmock-tests/src/common/returns/never_return_type.rs) | ❌ [fixture](../fnmock-tests/src/common/returns/unsupported_never_return_type_mock.cf.rs) | There is no value for a fake closure to produce; a spy does not produce one. |

## Trait objects

A `dyn Trait` behind a pointer or reference is an ordinary named type as far as all three macros are
concerned, in parameter and return position alike. The restriction is on `impl Trait` (above), which
is not a named type.

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| Boxed trait objects (`Box<dyn Trait>`) | ✅ [test](../fnmock-tests/src/common/traits/boxed.rs) | ✅ [test](../fnmock-tests/src/common/traits/boxed.rs) | ✅ [test](../fnmock-tests/src/common/traits/boxed.rs) |
| Referenced trait objects (`&dyn Trait`) | ✅ [test](../fnmock-tests/src/common/traits/referenced.rs) | ✅ [test](../fnmock-tests/src/common/traits/referenced.rs) | ✅ [test](../fnmock-tests/src/common/traits/referenced.rs) |
| Mutably referenced trait objects (`&mut dyn Trait`) | ✅ [test](../fnmock-tests/src/common/traits/referenced_mut.rs) | ✅ [test](../fnmock-tests/src/common/traits/referenced_mut.rs) | ✅ [test](../fnmock-tests/src/common/traits/referenced_mut.rs) |
| Auto traits (`Box<dyn Send>`) | ✅ [test](../fnmock-tests/src/common/traits/auto_traits.rs) | ✅ [test](../fnmock-tests/src/common/traits/auto_traits.rs) | ✅ [test](../fnmock-tests/src/common/traits/auto_traits.rs) |

## Generics

All three macros give a generic item a per-instantiation store, keyed by `TypeId` for type parameters and
by value for const parameters, so the accessor takes a turbofish.

| Construct | Fakes | Spies | Mocks | Reason given for a rejection |
| --- | --- | --- | --- | --- |
| A single type parameter | ✅ [test](../fnmock-tests/src/common/generics/single_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/single_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/single_generic.rs) | — |
| Multiple type parameters | ✅ [test](../fnmock-tests/src/common/generics/multiple_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/multiple_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/multiple_generics.rs) | — |
| Type parameters mixed with lifetimes and const generics | ✅ [test](../fnmock-tests/src/common/generics/mixed_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/mixed_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/mixed_generics.rs) | — |
| Unused type parameters | ✅ [test](../fnmock-tests/src/common/generics/unused_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/unused_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/unused_generic.rs) | — |
| Inline trait bounds, `where` clauses, and the two mixed | ✅ [where](../fnmock-tests/src/common/generics/where_bounds.rs), [mixed](../fnmock-tests/src/common/generics/where_and_direct_bounds.rs) | ✅ [where](../fnmock-tests/src/common/generics/where_bounds.rs), [mixed](../fnmock-tests/src/common/generics/where_and_direct_bounds.rs) | ✅ [where](../fnmock-tests/src/common/generics/where_bounds.rs), [mixed](../fnmock-tests/src/common/generics/where_and_direct_bounds.rs) | — |
| `where` clauses on non-parameter types | ✅ [test](../fnmock-tests/src/common/generics/non_parameter_where.rs) | ✅ [test](../fnmock-tests/src/common/generics/non_parameter_where.rs) | ✅ [test](../fnmock-tests/src/common/generics/non_parameter_where.rs) | — |
| Associated type bounds and equality (`I: Iterator<Item = u8>`) | ✅ [bounds](../fnmock-tests/src/common/generics/associated_type_bounds.rs), [equality](../fnmock-tests/src/common/generics/associated_type_equality.rs) | ✅ [bounds](../fnmock-tests/src/common/generics/associated_type_bounds.rs), [equality](../fnmock-tests/src/common/generics/associated_type_equality.rs) | ✅ [bounds](../fnmock-tests/src/common/generics/associated_type_bounds.rs), [equality](../fnmock-tests/src/common/generics/associated_type_equality.rs) | — |
| Higher-ranked bounds (`for<'a> Fn(&'a str) -> &'a str`) | ✅ [test](../fnmock-tests/src/common/generics/higher_ranked_bounds.rs) | ✅ [test](../fnmock-tests/src/common/generics/higher_ranked_bounds.rs) | ✅ [test](../fnmock-tests/src/common/generics/higher_ranked_bounds.rs) | — |
| A `'static` bound written as a named lifetime | ✅ [test](../fnmock-tests/src/common/generics/static_generic_via_named_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/static_generic_via_named_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/static_generic_via_named_lifetime.rs) | — |
| A `'static` bound reached through `<'a: 'static>`, a chain of lifetimes, or a `where` clause (`where T: 'a, 'a: 'static`) | ✅ [declaration](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_lifetime_param_declaration.rs), [chain](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_transitive_lifetime.rs), [where](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_where_predicates.rs) | ✅ [declaration](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_lifetime_param_declaration.rs), [chain](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_transitive_lifetime.rs), [where](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_where_predicates.rs) | ✅ [declaration](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_lifetime_param_declaration.rs), [chain](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_transitive_lifetime.rs), [where](../fnmock-tests/src/common/generics/lifetimes/static_bound_via_where_predicates.rs) | — |
| A trait bound naming a `'static` lifetime (`T: Into<&'a str>` with `'a: 'static`) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/bound_referencing_a_static_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/bound_referencing_a_static_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/bound_referencing_a_static_lifetime.rs) | — |
| A higher-ranked bound with the binder on the `where` predicate (`where for<'x> F: Fn(&'x str) -> String`) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/higher_ranked_where_predicate.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/higher_ranked_where_predicate.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/higher_ranked_where_predicate.rs) | — |
| A trait bound naming a non-`'static` lifetime (`T: Into<&'a str>`) | ❌ [fixture](../fnmock-tests/src/common/generics/lifetimes/unsupported_bound_references_non_static_lifetime_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/generics/lifetimes/unsupported_bound_references_non_static_lifetime_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/generics/lifetimes/unsupported_bound_references_non_static_lifetime_fake.cf.rs) | The bound is redeclared on the generated items, where the lifetime is not in scope. Constrain it with `'a: 'static`. |
| A non-`'static` lifetime bound on a type parameter (`T: 'a`), or a bare non-`'static` `T` | ❌ [fixture](../fnmock-tests/src/common/generics/unsupported_non_static_lifetime_bound_fake.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/generics/unsupported_non_static_lifetime_bound_on_type_param_spy.cf.rs) | ❌ [fixture](../fnmock-tests/src/common/generics/unsupported_non_static_lifetime_bound_fake.cf.rs) | Both stores are keyed by `TypeId`, which requires `'static`; the spy's `Expectation<M>` also requires `M: Any`. |

### Lifetimes

Lifetime parameters themselves are unconstrained for all three macros — none keys on lifetimes.
Nothing here is rejected. (For spies, some of these shapes do disable the predicate-based `.expect()`
in favour of `.expectf()`; see
[FEATURES.md](FEATURES.md#when-expect-is-unavailable).)

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| A named lifetime on a parameter type | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_param_type.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_param_type.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_param_type.rs) |
| An inferred (`'_`) lifetime on a parameter type | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/infered_lifetime_param_type.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/infered_lifetime_param_type.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/infered_lifetime_param_type.rs) |
| A reference with a named lifetime | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/reference_with_named_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/reference_with_named_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/reference_with_named_lifetime.rs) |
| Multiple lifetime parameters | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/multiple_lifetimes.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/multiple_lifetimes.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/multiple_lifetimes.rs) |
| Lifetimes mixed with type parameters | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/mixed_lifetime_and_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/mixed_lifetime_and_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/mixed_lifetime_and_generic.rs) |
| A lifetime nested inside a container | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/nested_lifetime_in_container.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/nested_lifetime_in_container.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/nested_lifetime_in_container.rs) |
| Unused lifetime parameters | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/unused_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/unused_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/unused_lifetime.rs) |
| A return type borrowing from a parameter (`-> &'a str`) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/borrowed_return.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/borrowed_return.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/borrowed_return.rs) |
| An explicit `'static` on a parameter type | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/static_reference_param.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/static_reference_param.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/static_reference_param.rs) |
| A `'static` type parameter behind a reference with its own lifetime | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/generic_behind_reference.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/generic_behind_reference.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/generic_behind_reference.rs) |
| An outlives relation between two of the item's own lifetimes (`<'a, 'b: 'a>`) | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_outlives_relation.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_outlives_relation.rs) | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/lifetimes/lifetime_outlives_relation.rs) |

⚠️ For a **fake**, an outlives relation between two of the item's own lifetimes is not carried into the closure's bound. With `-> &'a str`, the closure therefore cannot return data borrowed from the `'b` parameter; returning the `'a` parameter or a `'static` value works.

### Const generics

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| A single const parameter | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/const_generics/single_const_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/single_const_generic.rs) | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/const_generics/single_const_generic.rs) |
| Multiple const parameters | ✅ [test](../fnmock-tests/src/common/generics/const_generics/multiple_const_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/multiple_const_generics.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/multiple_const_generics.rs) |
| Unused const parameters | ✅ [test](../fnmock-tests/src/common/generics/const_generics/unused_const_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/unused_const_generic.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/unused_const_generic.rs) |
| A const parameter next to a lifetime | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/lifetimes/const_generic_with_lifetime.rs) | ✅ [test](../fnmock-tests/src/common/generics/lifetimes/const_generic_with_lifetime.rs) | ✅ ⚠️ [test](../fnmock-tests/src/common/generics/lifetimes/const_generic_with_lifetime.rs) |

Const parameters are keyed by **value**, so `foo_fake::<5>()` / `foo_spy::<5>()` does not affect a
call to `foo::<7>()`
([cross_value_isolation.rs](../fnmock-tests/src/common/generics/const_generics/cross_value_isolation.rs)).

⚠️ For a **fake**, the const value is not passed into the closure — hardcode it. Since the fake only
applies to the one value you selected on the accessor, that is not a real loss.

## Impl blocks

Applying either attribute to an inherent impl block makes every method in it fakeable / spyable,
each with its own store. What the block itself may be is under
[What you can apply the attribute to](#what-you-can-apply-the-attribute-to); `const` methods are
under [Function modifiers](#function-modifiers).

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| Methods with a receiver | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/basic.rs) |
| Associated functions (no receiver) | ✅ [test](../fnmock-tests/src/common/impl_block/associated_function.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/associated_function.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/associated_function.rs) |
| Multiple methods per block, each handled independently | ✅ [test](../fnmock-tests/src/common/impl_block/multiple_methods.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/multiple_methods.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/multiple_methods.rs) |
| `async` methods | ✅ [test](../fnmock-tests/src/common/impl_block/async_method.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/async_method.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/async_method.rs) |

### Receivers and `Self` returns

Every receiver form Rust offers is supported by all three macros. A fake is handed the receiver as its
first closure argument; a spy does not record it, and a mock does both (see
[SPY.md](SPY.md#methods) and
[FEATURES.md](FEATURES.md#impl-blocks)).

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| `&self` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced.rs) |
| `&self` with further parameters | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced_with_params.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced_with_params.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_referenced_with_params.rs) |
| `&mut self` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_mut_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_mut_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_mut_referenced.rs) |
| `self` (consuming) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_consumed.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_consumed.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_consumed.rs) |
| `self: Box<Self>` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_boxed.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_boxed.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_boxed.rs) |
| `self: Rc<Self>` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_rc.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_rc.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_rc.rs) |
| `self: Pin<&mut Self>` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_pin_mut.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_pin_mut.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_pin_mut.rs) |
| An explicitly typed receiver (`self: Type`) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_type.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_type.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/self_type.rs) |
| Returning `Self` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self.rs) |
| Returning `&Self` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self_referenced.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_self_referenced.rs) |
| Returning `Option<Self>` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_option_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_option_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_option_self.rs) |
| Returning `Result<Self, _>` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_result_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_result_self.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_result_self.rs) |
| Returning `()` | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_unit.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_unit.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/receiver/return_unit.rs) |

### Generic impl blocks

Struct generics go on the type, method generics on the accessor
(`Type::<G>::method_fake::<M>()`). The `'static` requirement from [Generics](#generics) applies.

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| Generic struct | ✅ [struct](../fnmock-tests/src/common/impl_block/generics/generic_struct.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_struct_where.rs) | ✅ [struct](../fnmock-tests/src/common/impl_block/generics/generic_struct.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_struct_where.rs) | ✅ [struct](../fnmock-tests/src/common/impl_block/generics/generic_struct.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_struct_where.rs) |
| Generic method | ✅ [method](../fnmock-tests/src/common/impl_block/generics/generic_method.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_method_where.rs) | ✅ [method](../fnmock-tests/src/common/impl_block/generics/generic_method.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_method_where.rs) | ✅ [method](../fnmock-tests/src/common/impl_block/generics/generic_method.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_method_where.rs) |
| Generic `async` method | ✅ [test](../fnmock-tests/src/common/impl_block/generics/generic_method_async.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/generics/generic_method_async.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/generics/generic_method_async.rs) |
| Generic struct *and* generic method | ✅ [combined](../fnmock-tests/src/common/impl_block/generics/generic_combined.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_combined_where.rs) | ✅ [combined](../fnmock-tests/src/common/impl_block/generics/generic_combined.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_combined_where.rs) | ✅ [combined](../fnmock-tests/src/common/impl_block/generics/generic_combined.rs), [where](../fnmock-tests/src/common/impl_block/generics/generic_combined_where.rs) |
| An impl block on a partly or fully concrete instantiation (`impl Foo<u8>`, `impl<U> Foo<u8, U>`) | ✅ [concrete](../fnmock-tests/src/common/impl_block/generics/generic_instantiation.rs), [mixed](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_mixed.rs) | ✅ [concrete](../fnmock-tests/src/common/impl_block/generics/generic_instantiation.rs), [mixed](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_mixed.rs) | ✅ [concrete](../fnmock-tests/src/common/impl_block/generics/generic_instantiation.rs), [mixed](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_mixed.rs) |
| Lifetimes on the impl block and the method | ✅ [with generics](../fnmock-tests/src/common/impl_block/generics/lifetimes_and_generics.rs), [combined](../fnmock-tests/src/common/impl_block/generics/lifetimes_combined.rs) | ✅ [with generics](../fnmock-tests/src/common/impl_block/generics/lifetimes_and_generics.rs), [combined](../fnmock-tests/src/common/impl_block/generics/lifetimes_combined.rs) | ✅ [with generics](../fnmock-tests/src/common/impl_block/generics/lifetimes_and_generics.rs), [combined](../fnmock-tests/src/common/impl_block/generics/lifetimes_combined.rs) |

## Visibility

The generated accessor inherits the visibility of the item, for all three macros, so it is reachable from
exactly the same places — no more, no less. The ✅ rows test that it *is* reachable where the item
is; the ❌ rows are compile-fail fixtures testing that it is *not* reachable where the item is not.
Those fixtures use `#[fnmock::fakeable]` (or `#[fnmock::spyable]`); the visibility handling is shared code, so the Mocks column points back at the Fakes column rather than duplicating them.

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| `pub` | ✅ [free fn](../fnmock-tests/src/common/visibility/public.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/public.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/public.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub.rs) |
| `pub(crate)` | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_crate.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_crate.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_crate.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_crate.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_crate.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_crate.rs) |
| `pub(super)` | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_super.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_super.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_super.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_super.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_super.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_super.rs) |
| `pub(in path)` | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_in_path.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_in_path.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_in_path.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_in_path.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_in_path.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_in_path.rs) |
| A private accessor is **not** reachable from outside its module | ✅ [free fn](../fnmock-tests/src/common/visibility/private_not_accessible.cf.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_private_not_accessible.cf.rs) | ✅ (shared code) | ✅ (same code as the fake; see the Fakes column) |
| A `pub(super)` accessor is **not** reachable from a sibling module | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_super_not_accessible_from_sibling.cf.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_super_not_accessible_from_sibling.cf.rs) | ✅ (shared code) | ✅ (same code as the fake; see the Fakes column) |
| A `pub(in path)` accessor is **not** reachable from outside that path | ✅ [free fn](../fnmock-tests/src/common/visibility/pub_in_path_not_accessible_from_outside.cf.rs), [method](../fnmock-tests/src/common/impl_block/visibility/impl_method_pub_in_path_not_accessible_from_outside.cf.rs) | ✅ (shared code) | ✅ (same code as the fake; see the Fakes column) |

## Other attributes on the item

Attributes you place on the item survive expansion, in either order relative to the fnmock
attribute, on free functions and on impl-block methods. Nothing here is rejected. The `.cf.rs`
fixtures deny the corresponding lint and assert that it still fires — which is what proves the
attribute was not swallowed.

| Construct | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| Doc comments (`///`), before and after | ✅ [test](../fnmock-tests/src/common/attributes/doc_comments.rs) | ✅ [test](../fnmock-tests/src/common/attributes/doc_comments.rs) | ✅ [test](../fnmock-tests/src/common/attributes/doc_comments.rs) |
| `#[deprecated]`, before or after | ✅ [before](../fnmock-tests/src/common/attributes/deprecated_before_fake.cf.rs), [after](../fnmock-tests/src/common/attributes/deprecated_after_fake.cf.rs) | ✅ [before](../fnmock-tests/src/common/attributes/deprecated_before_spy.cf.rs), [after](../fnmock-tests/src/common/attributes/deprecated_after_spy.cf.rs) | ✅ (same code as the fake; see the Fakes column) |
| `#[deprecated]` on an impl-block method | ✅ [test](../fnmock-tests/src/common/attributes/deprecated_impl_method_fake.cf.rs) | ✅ [test](../fnmock-tests/src/common/attributes/deprecated_impl_method_spy.cf.rs) | ✅ (same code as the fake; see the Fakes column) |
| `#[must_use]`, before or after | ✅ [before](../fnmock-tests/src/common/attributes/must_use_before_fake.cf.rs), [after](../fnmock-tests/src/common/attributes/must_use_after_fake.cf.rs) | ✅ [before](../fnmock-tests/src/common/attributes/must_use_before_spy.cf.rs), [after](../fnmock-tests/src/common/attributes/must_use_after_spy.cf.rs) | ✅ (same code as the fake; see the Fakes column) |

## Isolation

State lives in `thread_local!` storage for all three macros, so nothing leaks between threads or (for
generics) between instantiations.

| Rule | Fakes | Spies | Mocks |
| --- | --- | --- | --- |
| **Thread isolation.** State set on one thread is invisible to another and does not leak back out of a spawned thread. | ✅ [free fn](../fnmock-tests/src/common/visibility/thread_isolation.rs), [method](../fnmock-tests/src/common/impl_block/thread_isolation.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/thread_isolation.rs), [method](../fnmock-tests/src/common/impl_block/thread_isolation.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/thread_isolation.rs), [method](../fnmock-tests/src/common/impl_block/thread_isolation.rs) |
| **Per-instantiation isolation.** Touching one generic instantiation leaves the others untouched. | ✅ [type](../fnmock-tests/src/common/generics/cross_type_isolation.rs), [mixed](../fnmock-tests/src/common/generics/cross_type_isolation_mixed.rs), [impl block](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_isolation.rs) | ✅ [type](../fnmock-tests/src/common/generics/cross_type_isolation.rs), [mixed](../fnmock-tests/src/common/generics/cross_type_isolation_mixed.rs), [impl block](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_isolation.rs) | ✅ [type](../fnmock-tests/src/common/generics/cross_type_isolation.rs), [mixed](../fnmock-tests/src/common/generics/cross_type_isolation_mixed.rs), [impl block](../fnmock-tests/src/common/impl_block/generics/generic_instantiation_isolation.rs) |
| **Per-const-value isolation.** Same, keyed by const value. | ✅ [test](../fnmock-tests/src/common/generics/const_generics/cross_value_isolation.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/cross_value_isolation.rs) | ✅ [test](../fnmock-tests/src/common/generics/const_generics/cross_value_isolation.rs) |
| **Same-name isolation.** Two same-named functions in different modules are independent. | ✅ [free fn](../fnmock-tests/src/common/visibility/same_name_isolation.rs), [method](../fnmock-tests/src/common/impl_block/module_path_isolation.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/same_name_isolation.rs), [method](../fnmock-tests/src/common/impl_block/module_path_isolation.rs) | ✅ [free fn](../fnmock-tests/src/common/visibility/same_name_isolation.rs), [method](../fnmock-tests/src/common/impl_block/module_path_isolation.rs) |
| **Same-method-name isolation.** The same method name on two different types is independent. | ✅ [test](../fnmock-tests/src/common/impl_block/same_method_name_isolation.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/same_method_name_isolation.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/same_method_name_isolation.rs) |
| **Same-struct isolation.** Two identically named structs in different modules keep separate stores. | ✅ [test](../fnmock-tests/src/common/impl_block/same_struct_isolation.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/same_struct_isolation.rs) | ✅ [test](../fnmock-tests/src/common/impl_block/same_struct_isolation.rs) |

### Test scope

All three macros gate their injected code behind `#[cfg(test)]`, so release builds carry no fnmock
machinery at all. The flip side is a scope limitation that no fixture can express, because it is a
property of how Rust compiles test code rather than of the macro: fakes, spies and mocks can only be driven
from a `#[cfg(test)]` unit test **inside the crate that defines the item** — not from an integration
test under `tests/`, not from a doctest, and not from another crate. See
[test scope](../USAGE.md#test-scope) in USAGE.md.
