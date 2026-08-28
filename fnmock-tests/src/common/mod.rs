mod params {
    mod by_ref_and_value;
    mod by_value;
    mod implicit_lifetime;
    mod interior_mutability;
    mod mut_reference;
    mod raw_const_pointers;
    mod raw_mut_pointers;
    mod reference;
    mod reference_in_option;
    mod reference_in_slice;
    mod reference_in_tuple;
    mod reference_in_vec;
    mod slice;
    mod smart_pointers;
    mod zero_args;

    mod patterns {
        mod mutable;
        mod mutable_nested;
        mod slice_destructuring;
        mod tuple_destructuring;
        mod tuple_destructuring_nested;
    }
}

mod returns {
    mod return_option;
    mod return_result;
    mod return_unit;
}

mod special {
    mod async_function;
    mod async_generic_function;
    mod extern_function;
    mod futures;
    mod unsafe_function;
}

mod generics {
    mod associated_type_bounds;
    mod associated_type_equality;
    mod cross_type_isolation;
    mod cross_type_isolation_mixed;
    mod higher_ranked_bounds;
    mod mixed_generics;
    mod multiple_generics;
    mod non_parameter_where;
    mod return_generic;
    mod single_generic;
    mod static_generic_via_named_lifetime;
    mod unused_generic;
    mod where_and_direct_bounds;
    mod where_bounds;

    mod const_generics {
        mod cross_value_isolation;
        mod multiple_const_generics;
        mod single_const_generic;
        mod unused_const_generic;
    }

    mod lifetimes {
        mod infered_lifetime_param_type;
        mod lifetime_param_type;
        mod mixed_lifetime_and_generic;
        mod multiple_lifetimes;
        mod nested_lifetime_in_container;
        mod reference_with_named_lifetime;
        mod unused_lifetime;
    }
}
