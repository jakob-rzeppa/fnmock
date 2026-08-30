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

mod impl_block {
    mod associated_function;
    mod async_method;
    mod basic;
    mod clear_and_is_set;
    mod module_path_isolation;
    mod multiple_methods;
    mod same_method_name_isolation;
    mod same_struct_isolation;
    mod thread_isolation;

    mod receiver {
        mod return_option_self;
        mod return_result_self;
        mod return_self;
        mod return_self_referenced;
        mod return_unit;
        mod self_boxed;
        mod self_consumed;
        mod self_mut_referenced;
        mod self_pin_mut;
        mod self_rc;
        mod self_referenced;
        mod self_referenced_with_params;
        mod self_type;
    }

    mod generics {
        mod clear_and_is_set;
        mod generic_combined;
        mod generic_combined_where;
        mod generic_instantiation;
        mod generic_instantiation_isolation;
        mod generic_instantiation_mixed;
        mod generic_method;
        mod generic_method_async;
        mod generic_method_where;
        mod generic_struct;
        mod generic_struct_where;
        mod lifetimes_and_generics;
        mod lifetimes_combined;
    }

    mod visibility {
        mod impl_method_pub;
        mod impl_method_pub_crate;
        mod impl_method_pub_in_path;
        mod impl_method_pub_super;
    }
}

mod returns {
    mod never_return_type;
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

mod traits {
    mod auto_traits;
    mod boxed;
    mod impl_trait_return;
    mod referenced;
    mod referenced_mut;
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

mod visibility {
    mod pub_crate;
    mod pub_in_path;
    mod pub_super;
    mod public;
    mod same_name_isolation;
    mod thread_isolation;
}
