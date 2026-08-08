mod basic {
    mod by_ref_and_value;
    mod by_reference;
    mod by_value;
    mod function_executes_normally;
    mod no_params;
}

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
