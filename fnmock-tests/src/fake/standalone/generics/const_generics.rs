// #[fnmock::fakeable]
// fn const_generics<const C: usize>(a: String) -> String {
//     format!("{} {}", a, C)
// }

// #[test]
// fn test_const_generics() {
//     let res = const_generics::<5>("Test".to_string());
//     assert_eq!(res, "Test 5");
// }

// #[test]
// fn test_const_generics_fake() {
//     const_generics_fake::<5>().setup(|a| format!("Fake {}", a, C));
//     let res = const_generics::<5>("Test".to_string());
//     assert_eq!(res, "Fake Test 5");
// }
