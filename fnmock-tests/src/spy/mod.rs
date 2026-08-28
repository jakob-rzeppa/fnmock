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
    mod bounds_where;
    mod expectations_per_instantiation;
    mod generic_in_container_param;
    mod generic_only_in_return;
    mod generic_reference_param;
    mod sequence_across_instantiations;
    mod sequence_within_instantiation;
}

mod lifetimes {
    mod lifetime_expectf_in_sequence;
}

mod unsupported;
