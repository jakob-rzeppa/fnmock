/// Information about a fake module that is being generated for a struct method or a free function.
#[derive(Clone)]
pub struct FakeableInfo {
    /// Access function name for the fake interface (e.g. "get_user_fake").
    pub access_function_name: syn::Ident,

    /// The name of the module that will be generated (e.g. "get_user_fake").
    /// For struct method fakes, this is the struct name + method name with "_fake" appended (e.g. "user_repository_get_user_fake").
    /// For free function fakes, this is the function name with "_fake" appended (e.g. "handle_user_fake").
    pub module_name: syn::Ident,

    /// The name of the thread-local static variable that will store the fake implementation (e.g. "GET_USER_FAKE_STORE").
    pub store_name: syn::Ident,
    /// The display name for error messages.
    pub display_name: String,

    /// The name of the struct that provides the API for setting up and accessing the fake implementations (e.g. "GetUserFake").
    pub interface_struct_name: syn::Ident,

    /// The trait bound of the function closure for the fake implementation (e.g. `Fn(&UserRepository<T>, I) -> Option<String>`).
    pub fn_closure_trait: syn::TraitBound,

    /// Information about the generic parameters for this fake module, if any. For struct method fakes, this includes both the generic parameters from the struct and the method, in the order they appear in the code.
    pub generic_info: Option<FakeableGenericInfo>,
}

/// Information about the generic parameters for a fake module.
///
/// If it is a struct method fake, the generic parameters from the struct come first, followed by the generic parameters from the method.
#[derive(Clone)]
pub struct FakeableGenericInfo {
    pub generic_count: usize,

    /// The types of the generic parameters.
    pub generic_types: Vec<syn::Type>,

    /// The generic parameters, including their bounds (e.g. `T: Display + 'static` and `I: 'static`).
    pub generic_params: Vec<syn::TypeParam>,

    /// The `TypeId` expressions for the generic parameters, in the order they appear in the code (e.g. `[std::any::TypeId::of::<T>(), std::any::TypeId::of::<I>()]`).
    pub generic_type_ids: Vec<syn::Expr>,
}
