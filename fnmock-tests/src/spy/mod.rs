mod clear {
    mod basic;
    mod generics;
    mod impl_block;
    mod sequences;
}

mod expectations {
    mod describe;
    mod expect_availability;
    mod expectf;
    mod global_times;
    mod multiple_independent_expectations;
    mod predicates;
    mod times;
    mod unexpected_call_is_not_an_error;
}

mod sequences {
    mod advancable_range;
    mod basic_order;
    mod chaining_order;
    mod cross_function;
    mod expectation_in_multiple_sequences;
    mod greedy_matching;
    mod last_step_stays_current;
    mod multiple_independent_sequences;
    mod never_step;
    mod out_of_order_lenient;
    mod sequence_default;
    mod strict_sequence_in_order;
    mod unsequenced_expectation_independent;
}

mod generics {
    mod assert_scoped_to_instantiation;
    mod generic_only_in_return;
    mod generic_reference_param;
    mod sequence_across_instantiations;
}

mod lifetimes {
    mod lifetime_expectf_in_sequence;
}
