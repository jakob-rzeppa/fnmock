mod first {
    #[fnmock::fakeable]
    pub fn fetch(a: i32) -> i32 {
        a + 1
    }
}

mod second {
    #[fnmock::fakeable]
    pub fn fetch(a: i32) -> i32 {
        a + 100
    }
}

mod third {
    #[fnmock::spyable]
    pub fn fetch(a: i32) -> i32 {
        a + 1000
    }
}

mod fourth {
    #[fnmock::spyable]
    pub fn fetch(a: i32) -> i32 {
        a + 10000
    }
}

#[test]
fn test_real_bodies_are_independent() {
    assert_eq!(first::fetch(1), 2);
    assert_eq!(second::fetch(1), 101);
    assert_eq!(third::fetch(1), 1001);
    assert_eq!(fourth::fetch(1), 10001);
}

#[test]
fn test_fakes_are_independent() {
    first::fetch_fake().setup(|a| a + 1000);
    assert_eq!(first::fetch(1), 1001);
    assert_eq!(second::fetch(1), 101);

    second::fetch_fake().setup(|a| a + 2000);
    assert_eq!(first::fetch(1), 1001);
    assert_eq!(second::fetch(1), 2001);
}

#[test]
fn test_spies_are_independent() {
    let spy1 = third::fetch_spy();
    spy1.expect_once();

    let spy2 = fourth::fetch_spy();
    spy2.expect_once();

    assert_eq!(third::fetch(1), 1001);
    assert_eq!(fourth::fetch(1), 10001);

    spy1.assert();
    spy2.assert();
}
