use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Expr, ExprLit, FnArg, ItemFn, Lit, ReturnType, Type, TypePath,
};

pub fn vfunc_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let index = parse_macro_input!(attr as VirtualFunctionIndex).0;

    let mut func: ItemFn = syn::parse(item.clone()).unwrap();

    let vis = &func.vis;
    let attrs = &func.attrs;

    let sig = &func.sig;

    let inputs = &sig.inputs;
    let output = &sig.output;

    let (arg_idents, arg_types) = get_args(inputs);

    let (converted_args, vfunc_types) = convert_to_c_args(arg_idents, arg_types);

    let vfunc = get_vfunction(index, vfunc_types, output);

    let ret_type = extract_return_type_type(output);

    let vfunc_call = if is_type_ref(&ret_type) {
        func.sig.output = ReturnType::parse
            .parse2(quote! { -> Option<#ret_type> })
            .expect("could not convert reference result type to option");

        Expr::parse
            .parse2(quote! {
                #vfunc(self, #(#converted_args),*).as_ref()
            })
            .expect("could not create vfunction call")
    } else {
        Expr::parse
            .parse2(quote! {
                #vfunc(self, #(#converted_args),*)
            })
            .expect("could not create vfunction call")
    };

    let sig = &func.sig;

    quote! {
        #(#attrs)* #vis #sig {
            unsafe {
                #vfunc_call
            }
        }
    }
    .into()
}

struct VirtualFunctionIndex(isize);

impl Parse for VirtualFunctionIndex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let index: ExprLit = input.parse()?;

        if let Lit::Int(lit) = &index.lit {
            Ok(Self(lit.base10_parse()?))
        } else {
            Err(syn::Error::new(
                index.span(),
                "invalid virtual function index",
            ))
        }
    }
}

fn get_vfunction(index: isize, types: Vec<Type>, ret_type: &ReturnType) -> TokenStream2 {
    let mut ret_type = extract_return_type_type(ret_type);

    if let Type::Reference(type_ref) = ret_type {
        let elem = type_ref.elem;

        ret_type = Type::parse
            .parse2(quote! { *const #elem })
            .expect("could not convert return type to option");
    }

    quote! {
        std::mem::transmute::<_, extern "fastcall" fn(*const Self, #(#types),*) -> #ret_type> (
           (*std::mem::transmute::<_, *const *const usize>(self)).offset(#index).read()
        )
    }
}

fn extract_return_type_type(ret_type: &ReturnType) -> Type {
    match ret_type {
        ReturnType::Default => Type::parse
            .parse2(quote! { () })
            .expect("could not extract function return type"),
        ReturnType::Type(_, ty) => *ty.clone(),
    }
}

fn get_args(args: &Punctuated<FnArg, Comma>) -> (Vec<Ident>, Vec<Type>) {
    args.iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => Some((
                match *pat_type.pat.clone() {
                    syn::Pat::Ident(ident) => ident.ident,
                    _ => return None,
                },
                *pat_type.ty.clone(),
            )),
        })
        .unzip()
}

fn convert_to_c_args(idents: Vec<Ident>, types: Vec<Type>) -> (Vec<Expr>, Vec<Type>) {
    idents
        .iter()
        .zip(types)
        .map(|(ident, ty)| {
            if let Type::Reference(refr) = ty {
                match *refr.elem {
                    Type::Path(path) => {
                        let ref_type = &path
                            .path
                            .segments
                            .last()
                            .expect("could not get reference type")
                            .ident;

                        if ref_type == "str" {
                            convert_to_c_string(ident)
                        } else {
                            convert_to_c_ptr(ident, &path)
                        }
                    }
                    _ => panic!("unexpected reference type"),
                }
            } else {
                (
                    Expr::parse
                        .parse2(quote! { #ident })
                        .expect("could not create arg expression"),
                    ty,
                )
            }
        })
        .unzip()
}

fn convert_to_c_string(ident: &Ident) -> (Expr, Type) {
    (
        Expr::parse
            .parse2(quote! { std::ffi::CString::new(#ident).unwrap().as_ptr() })
            .expect(
                "could not convert create a &str to *const std::ffi::c_char conversion expression",
            ),
        Type::parse
            .parse2(quote! { *const std::ffi::c_char })
            .expect("could not convert &str type to *const std::ffi::c_char"),
    )
}

fn convert_to_c_ptr(ident: &Ident, ref_type: &TypePath) -> (Expr, Type) {
    (
        Expr::parse
            .parse2(quote! { #ident })
            .expect("could not create a c ptr expression"),
        Type::parse
            .parse2(quote! { *const #ref_type })
            .expect("could not convert reference type to c pointer"),
    )
}

fn is_type_ref(ty: &Type) -> bool {
    matches!(ty, Type::Reference(_))
}
