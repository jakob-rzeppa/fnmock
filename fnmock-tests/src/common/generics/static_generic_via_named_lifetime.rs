// TODO
// mod fake {
//     //! `T: 'a` is rejected unless `'a` is tied to `'static` (see
//     //! `non_static_lifetime_bound.unsupported.fake.rs`). Spelling the bound with a
//     //! named lifetime constrained by `where 'a: 'static` is supported, since it is
//     //! effectively `'static`.

//     #[fnmock::fakeable]
//     fn static_generic_via_named_lifetime<'a, T: 'a + std::fmt::Display>(value: &'a T) -> String
//     where
//         'a: 'static,
//     {
//         format!("{value}")
//     }

//     #[test]
//     fn test_static_generic_via_named_lifetime() {
//         let value = "Test".to_string();
//         let res = static_generic_via_named_lifetime(&value);
//         assert_eq!(res, "Test");
//     }

//     #[test]
//     fn test_static_generic_via_named_lifetime_fake() {
//         static_generic_via_named_lifetime_fake::<String>().setup(|value| format!("Fake {}", value));

//         let value = "Test".to_string();
//         let res = static_generic_via_named_lifetime(&value);
//         assert_eq!(res, "Fake Test");
//     }
// }

// mod spy {
//     #[fnmock::spyable]
//     fn static_generic_via_named_lifetime<'a, T: 'a + std::fmt::Display>(value: &'a T) -> String
//     where
//         'a: 'static,
//     {
//         format!("{value}")
//     }

//     #[test]
//     fn test_static_generic_via_named_lifetime() {
//         let spy = static_generic_via_named_lifetime_spy::<String>();
//         spy.expect(fnmock::predicate::eq("Test".to_string())).once();

//         let value = "Test".to_string();
//         let res = static_generic_via_named_lifetime(&value);

//         assert_eq!(res, "Test");
//         spy.assert();
//     }
// }
