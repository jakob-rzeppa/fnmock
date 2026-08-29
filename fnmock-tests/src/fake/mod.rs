mod basic {
    mod clear_and_is_set;
    mod reentrant_fake;
}
mod generics {
    mod associated_type_equality;
    mod bounds_lifetime;
    mod clear_and_is_set;
    mod const_generics_clear_and_is_set;
    mod higher_ranked_bounds;
    mod named_lifetime;
    mod return_generic;
    mod static_lifetime;
    mod unused_lifetime;
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
}

mod visibility {
    // mod impl_method_pub;
    // mod impl_method_pub_crate;
    // mod impl_method_pub_in_path;
    // mod impl_method_pub_super;
}

// mod attributes;

mod unsupported;
