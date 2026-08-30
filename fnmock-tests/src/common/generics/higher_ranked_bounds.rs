mod fake {
    #[fnmock::fakeable]
    fn higher_ranked_bounds<F>(f: F, s: &str) -> String
    where
        F: for<'a> Fn(&'a str) -> String + 'static,
    {
        f(s)
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_bounds() {
        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_bounds(f, "test");
        assert_eq!(res, "TEST");
    }

    #[test]
    fn test_higher_ranked_bounds_fake() {
        higher_ranked_bounds_fake::<fn(&str) -> String>().setup(|_f, s| format!("Fake {}", s));

        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_bounds(f, "test");
        assert_eq!(res, "Fake test");
    }
}

// TODO
// mod spy {
//     #[fnmock::spyable]
//     fn higher_ranked_bounds<F>(f: F, s: &str) -> String
//     where
//         F: for<'a> Fn(&'a str) -> String + 'static,
//     {
//         f(s)
//     }

//     fn uppercase(s: &str) -> String {
//         s.to_uppercase()
//     }

//     #[test]
//     fn test_higher_ranked_bounds() {
//         let spy = higher_ranked_bounds_spy::<fn(&str) -> String>();
//         spy.expectf(|_f, s: &str| s == "test").once();

//         let f: fn(&str) -> String = uppercase;
//         let res = higher_ranked_bounds(f, "test");

//         assert_eq!(res, "TEST");
//         spy.assert();
//     }
// }
