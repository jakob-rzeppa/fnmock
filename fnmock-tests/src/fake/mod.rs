mod basic {
    mod by_value;
    mod interior_mutability;
    mod mut_reference;
    mod raw_const_pointers;
    mod raw_mut_pointers;
    mod reference;
    mod return_option;
    mod return_result;
    mod return_unit;
    mod smart_pointers;
}
mod attributes {
    mod cfg;
    mod deprecated;
    mod inline;
    mod must_use;
    mod track_caller;
}
mod generics {
    mod associated_type_bounds;
    mod associated_type_equality;
    mod bounds_generic;
    mod bounds_lifetime;
    mod bounds_mixed;
    mod higher_ranked_bounds;
    mod implicit_lifetime;
    mod multiple_generics;
    mod multiple_lifetimes;
    mod named_lifetime;
    mod return_generic;
    mod single_generic;
    mod static_lifetime;
    mod const_generics;
    mod unused_generic;
    mod unused_lifetime;
}
mod patterns {
    mod ignored;
    mod mutable_patterns;
    mod reference_patterns;
    mod struct_destructuring;
    mod tuple_destructuring;
}
mod special {
    mod async_function;
    mod const_function;
    mod extern_function;
    mod futures;
    mod unsafe_function;
}
mod trait_based {
    mod auto_traits;
    mod boxed;
    mod referenced;
    mod referenced_mut;
}
