use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parser, parse_macro_input, Attribute, Field, Fields::Named, FieldsNamed, ItemStruct,
};

pub fn vmt_impl(item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);

    add_vmt_field(&mut item);
    add_repr_c(&mut item);

    item.into_token_stream().into()
}

fn add_vmt_field(item: &mut ItemStruct) {
    let fields = match &mut item.fields {
        Named(fields) => fields,
        _ => panic!("#[vmt] can only be applied to structs with named fields"),
    };

    if has_vmt_field(fields) {
        panic!("this struct already has a VMT field")
    }

    let vmt_field = Field::parse_named
        .parse2(quote! {
            __vmt: usize
        })
        .expect("could not add VMT field");

    fields.named.insert(0, vmt_field)
}

fn has_vmt_field(fields: &FieldsNamed) -> bool {
    fields
        .named
        .iter()
        .any(|field| field.ident.clone().map_or(false, |ident| ident == "__vmt"))
}

fn add_repr_c(item: &mut ItemStruct) {
    let mut repr_c = Attribute::parse_outer
        .parse2(quote! {
            #[repr(C)]
        })
        .expect("could not add #[repr(C)]");

    item.attrs.append(&mut repr_c);
}
