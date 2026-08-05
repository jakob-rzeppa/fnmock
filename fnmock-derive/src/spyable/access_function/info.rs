pub struct SpyAccessFunctionInfo {
    /// Access function name for the spy interface (e.g. "get_user_spy").
    pub access_function_name: syn::Ident,

    /// The name of the spy module this access function reaches into.
    pub module_name: syn::Ident,

    /// The name of the struct that provides the API for setting up and accessing the spy implementation.
    pub interface_struct_name: syn::Ident,
}
