use proc_macro::TokenStream;

mod with_instance;

/// A macro to inject the current request URI into a handler.
///
/// This is useful for logging or other purposes where the request URI is
/// needed. It is not fully functional yet.
#[proc_macro_attribute]
pub fn with_instance(attr: TokenStream, item: TokenStream) -> TokenStream {
    with_instance::with_instance_impl(attr, item)
}
