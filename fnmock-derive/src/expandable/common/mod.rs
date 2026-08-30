pub mod fake {
    pub mod inline_call;

    pub mod module {
        pub mod fake_store;
        pub mod implementation_getter;
        pub mod interface_getter;
        pub mod interface_impl;
        pub mod interface_struct;
    }
}

pub mod spy {
    pub mod inline_call;

    pub mod module {
        pub mod interface_getter;
        pub mod interface_impl;
        pub mod interface_struct;
        pub mod matcher;
        pub mod record_call;
        pub mod spy_store;
    }
}
