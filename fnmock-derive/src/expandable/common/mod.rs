pub mod interface {
    pub mod interface_getter;
    pub mod interface_struct;
}

pub mod fake {
    pub mod inline_call;

    pub mod module {
        pub mod fake_store;
        pub mod implementation_getter;
        pub mod interface_impl;
        pub mod module_parts;
    }
}

pub mod mock {
    pub mod clear;
    pub mod inline_call;
    pub mod module_parts;
}

pub mod spy {
    pub mod inline_call;

    pub mod module {
        pub mod interface_impl;
        pub mod matcher;
        pub mod module_parts;
        pub mod record_call;
        pub mod spy_store;
    }
}
