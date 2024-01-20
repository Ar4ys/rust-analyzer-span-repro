use quote::{quote, quote_spanned};
use rstml::{node::Node, parse};
use syn::spanned::Spanned;

#[proc_macro]
pub fn view(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let nodes = match parse(tokens) {
        Ok(value) => value,
        Err(error) => return error.into_compile_error().into(),
    };

    let Some(Node::Element(node)) = nodes.first() else {
        return proc_macro::TokenStream::new();
    };

    let name = node.name();

    let build = quote_spanned! { node.name().span() =>
        .build()
    };

    quote! {
        DOM(#name)#build
    }
    .into()
}
