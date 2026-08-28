mod params {
    mod by_ref_and_value;
    mod by_value;
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
