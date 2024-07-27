use proc_macro::TokenStream;

mod vfunc;
mod vmt;

#[proc_macro_attribute]
pub fn vmt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    vmt::vmt_impl(item)
}

#[proc_macro_attribute]
pub fn vfunc(attr: TokenStream, item: TokenStream) -> TokenStream {
    vfunc::vfunc_impl(attr, item)
}
