mod expectations {
    mod describe;
    mod expectf;
    mod global_times;
    mod multiple_independent_expectations;
    mod times;
    mod unexpected_call_is_not_an_error;
}

mod sequences {
    mod advancable_range;
    mod basic_order;
    mod chaining_order;
    mod cross_function;
    mod multiple_independent_sequences;
    mod out_of_order_lenient;
    mod strict_sequence_in_order;
    mod unsequenced_expectation_independent;
}

mod generics {
    mod assert_scoped_to_instantiation;
    mod associated_type_bounds;
    mod bounds_generic;
    mod bounds_mixed;
    mod bounds_where;
    mod const_generics;
    mod const_generics_mixed;
    mod const_generics_multiple;
    mod cross_type_isolation;
    mod expectations_per_instantiation;
    mod generic_in_container_param;
    mod generic_only_in_return;
    mod generic_reference_param;
    mod multiple_generics;
    mod sequence_across_instantiations;
    mod sequence_within_instantiation;
    mod single_generic;
    mod thread_isolation;
    mod unused_generic;
}

mod lifetimes {
    mod elided_lifetime_param_type;
    mod lifetime_expectf_in_sequence;
    mod lifetime_param_type;
    mod mixed_lifetime_and_generic;
    mod multiple_lifetimes;
    mod nested_lifetime_in_container;
    mod reference_with_named_lifetime;
}

mod unsupported;
