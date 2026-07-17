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
    mod const_generics;
    mod higher_ranked_bounds;
    mod implicit_lifetime;
    mod multiple_generics;
    mod multiple_lifetimes;
    mod named_lifetime;
    mod non_parameter_where;
    mod return_generic;
    mod single_generic;
    mod static_lifetime;
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
    mod impl_traits;
    mod referenced;
    mod referenced_mut;
}

mod impl_block {
    mod associated_function;
    mod basic;
    mod multiple_methods;
    mod same_method_name_isolation;

    mod attributes {
        mod cfg;
    }

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
        mod generic_combined;
        mod generic_combined_where;
        mod generic_method;
        mod generic_method_where;
        mod generic_struct;
        mod generic_struct_where;
        mod lifetimes_and_generics;
        mod lifetimes_combined;
    }
}
