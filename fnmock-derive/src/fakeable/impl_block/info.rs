pub struct FakeableImplFnInfo {
    /// The name of the method for which this fake is being generated (e.g. "get_user").
    pub fn_name: syn::Ident,
    /// The name of the method for which this fake is being generated (e.g. "get_user").
    pub fake_access_fn_name: syn::Ident,
    /// The name of the thread-local variable that holds the `GenericFakeStore` for this method (e.g. "GET_USER_FAKE_STORE").
    pub fake_store_name: syn::Ident,
    /// The name of the struct that provides the API for setting up and accessing the fake implementations (e.g. "GetUserFake").
    pub fake_api_name: syn::Ident,
    /// The name of the module where the fake struct is defined (e.g. "user_repository_fake").
    pub fake_module: syn::Ident,

    /// The identifiers of the parameters in the method signature (e.g. `self` and `user_id`), in the order they appear in the code.
    pub fn_param_idents: Vec<syn::Ident>,

    /// The type of the function pointer for the fake implementation (e.g. `fn(&UserRepository<T>, I) -> Option<String>`).
    pub fn_ptr_type: syn::Type,

    pub generic_info: Option<GenericFakeableImplFnInfo>,
}

pub struct GenericFakeableImplFnInfo {
    /// The identifiers of the generic parameters on the struct (e.g. `T`) and on the method (e.g. `I`), in the order they appear in the code.
    pub struct_generic_idents: Vec<syn::Ident>,
    pub fn_generic_idents: Vec<syn::Ident>,

    /// The generic parameters on the struct and method, including their bounds (e.g. `T: Display + 'static` and `I: 'static`).
    pub struct_generic_params: Vec<syn::GenericParam>,
    pub fn_generic_params: Vec<syn::GenericParam>,

    /// The `TypeId` expressions for the generic parameters on the struct and method, in the order they appear in the code (e.g. `[std::any::TypeId::of::<T>(), std::any::TypeId::of::<I>()]`).
    pub struct_generic_type_ids: Vec<proc_macro2::TokenStream>,
    pub fn_generic_type_ids: Vec<proc_macro2::TokenStream>,
}
