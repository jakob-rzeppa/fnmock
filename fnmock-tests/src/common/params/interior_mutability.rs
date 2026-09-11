mod fake {
    use std::{cell::RefCell, rc::Rc};

    #[fnmock::fakeable]
    fn interior_mutability(a: Rc<RefCell<String>>) {
        a.borrow_mut().push_str(" modified");
    }

    #[test]
    fn test_interior_mutability() {
        let value = Rc::new(RefCell::new("Test".to_string()));
        interior_mutability(value.clone());
        assert_eq!(value.borrow().as_str(), "Test modified");
    }

    #[test]
    fn test_interior_mutability_fake() {
        let value = Rc::new(RefCell::new("Test".to_string()));
        interior_mutability_fake().setup(|a| a.borrow_mut().push_str(" fake modified"));
        interior_mutability(value.clone());
        assert_eq!(value.borrow().as_str(), "Test fake modified");
    }
}

mod spy {
    use std::{cell::RefCell, rc::Rc};

    #[fnmock::spyable]
    fn interior_mutability(value: Rc<RefCell<String>>) {
        value.borrow_mut().push_str(" modified");
    }

    #[test]
    fn test_interior_mutability() {
        let spy = interior_mutability_spy();
        spy.expect(fnmock::predicate::eq(Rc::new(RefCell::new(
            "hi".to_string(),
        ))));

        let value = Rc::new(RefCell::new("hi".to_string()));
        interior_mutability(value.clone());

        spy.assert();
    }
}
