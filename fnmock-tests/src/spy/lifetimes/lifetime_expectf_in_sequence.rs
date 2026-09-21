//! A higher-ranked `expectf` still has to be storable as a sequence step, which
//! means the matcher stays `'static` despite the `for<'a>` in its callback.

struct Ref<'a>(&'a str);

#[fnmock::spyable]
fn lifetime_expectf_in_sequence(r: Ref<'_>) {
    let _ = r;
}

#[test]
fn test_higher_ranked_expectf_participates_in_a_sequence() {
    let spy = lifetime_expectf_in_sequence_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expectf(|r: &Ref<'_>| r.0 == "first")
        .once()
        .in_sequence(&mut seq);
    spy.expectf(|r: &Ref<'_>| r.0 == "second")
        .once()
        .in_sequence(&mut seq);

    let first = "first".to_string();
    let second = "second".to_string();
    lifetime_expectf_in_sequence(Ref(&first));
    lifetime_expectf_in_sequence(Ref(&second));

    spy.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_out_of_order_higher_ranked_expectf_panics_in_a_strict_sequence() {
    let spy = lifetime_expectf_in_sequence_spy();
    let mut seq = fnmock::Sequence::new_strict();
    spy.expectf(|r: &Ref<'_>| r.0 == "first")
        .once()
        .in_sequence(&mut seq);
    spy.expectf(|r: &Ref<'_>| r.0 == "second")
        .once()
        .in_sequence(&mut seq);

    let second = "second".to_string();
    lifetime_expectf_in_sequence(Ref(&second));
}
