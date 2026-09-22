#[fnmock::fakeable]
fn option_ref_param(a: Option<&str>) -> String {
    format!("Real {}", a.unwrap_or("none"))
}

#[test]
fn test_option_ref_param_fake() {
    option_ref_param_fake().setup(|a| format!("Fake {}", a.unwrap_or("none")));

    let value = "Test".to_string();
    assert_eq!(option_ref_param(Some(&value)), "Fake Test");
    assert_eq!(option_ref_param(None), "Fake none");
}

#[fnmock::fakeable]
fn vec_ref_param(items: Vec<&str>) -> String {
    format!("Real {}", items.join(","))
}

#[test]
fn test_vec_ref_param_fake() {
    vec_ref_param_fake().setup(|items| format!("Fake {}", items.join("|")));

    let a = "x".to_string();
    let b = "y".to_string();
    assert_eq!(vec_ref_param(vec![&a, &b]), "Fake x|y");
}

#[fnmock::fakeable]
fn slice_param(items: &[i32]) -> i32 {
    items.iter().sum()
}

#[test]
fn test_slice_param_fake() {
    slice_param_fake().setup(|items: &[i32]| items.iter().product());

    assert_eq!(slice_param(&[2, 3, 4]), 24);
}

#[fnmock::fakeable]
fn tuple_ref_param(pair: (&str, i32)) -> String {
    format!("Real {} {}", pair.0, pair.1)
}

#[test]
fn test_tuple_ref_param_fake() {
    tuple_ref_param_fake().setup(|pair| format!("Fake {} {}", pair.0, pair.1));

    let s = "Test".to_string();
    assert_eq!(tuple_ref_param((&s, 7)), "Fake Test 7");
}

#[fnmock::fakeable]
fn option_ref_return(a: &str) -> Option<&str> {
    Some(a)
}

#[test]
fn test_option_ref_return_fake() {
    option_ref_return_fake().setup(|_| None);

    let value = "Test".to_string();
    assert_eq!(option_ref_return(&value), None);
}

mod mock {
    #[fnmock::mockable]
    fn option_ref_param(a: Option<&str>) -> String {
        format!("Real {}", a.unwrap_or("none"))
    }

    #[test]
    fn test_option_ref_param() {
        let mock = option_ref_param_mock();
        mock.setup(|a| format!("Fake {}", a.unwrap_or("none")));
        mock.expect(fnmock::predicate::eq(Some("Test".to_string())))
            .once();

        let value = "Test".to_string();
        assert_eq!(option_ref_param(Some(&value)), "Fake Test");
        mock.assert();
    }

    #[fnmock::mockable]
    fn vec_ref_param(items: Vec<&str>) -> String {
        format!("Real {}", items.join(","))
    }

    #[test]
    fn test_vec_ref_param() {
        let mock = vec_ref_param_mock();
        mock.setup(|items| format!("Fake {}", items.join("|")));
        mock.expectf(|items: &Vec<&str>| items.as_slice() == ["x", "y"]);

        assert_eq!(vec_ref_param(vec!["x", "y"]), "Fake x|y");
        mock.assert();
    }

    #[fnmock::mockable]
    fn slice_param(items: &[i32]) -> i32 {
        items.iter().sum()
    }

    #[test]
    fn test_slice_param() {
        let mock = slice_param_mock();
        mock.setup(|items: &[i32]| items.iter().product());
        mock.expect(fnmock::predicate::eq(&[2, 3, 4][..])).once();

        assert_eq!(slice_param(&[2, 3, 4]), 24);
        mock.assert();
    }

    #[fnmock::mockable]
    fn tuple_ref_param(pair: (&str, i32)) -> String {
        format!("Real {} {}", pair.0, pair.1)
    }

    #[test]
    fn test_tuple_ref_param() {
        let mock = tuple_ref_param_mock();
        mock.setup(|pair| format!("Fake {} {}", pair.0, pair.1));
        mock.expect(fnmock::predicate::eq(("Test", 7))).once();

        assert_eq!(tuple_ref_param(("Test", 7)), "Fake Test 7");
        mock.assert();
    }

    #[fnmock::mockable]
    fn option_ref_return(a: &str) -> Option<&str> {
        Some(a)
    }

    #[test]
    fn test_option_ref_return() {
        let mock = option_ref_return_mock();
        mock.setup(|_| None);
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let value = "Test".to_string();
        assert_eq!(option_ref_return(&value), None);
        mock.assert();
    }
}
