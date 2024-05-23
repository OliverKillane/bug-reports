fn check_parse2<T: syn::parse::Parse>(tokens: proc_macro2::TokenStream) {
    syn::parse2::<T>(tokens).expect("Failed to parse");
}

#[test]
fn expr_let_from_quote() {
    check_parse2::<syn::ExprLet>(quote::quote! {
        let x = 1
    });
    check_parse2::<syn::ExprLet>(quote::quote! {
        let x = {}
    });
    check_parse2::<syn::ExprLet>(quote::quote! {
        let x = { Foo {} }
    });
}

#[test]
fn expr_let_from_quote_fail() {
    // failure!
    check_parse2::<syn::ExprLet>(quote::quote! {
        let x = Foo {}
    });
}

#[test]
fn expr_let_from_syn_fail() {
    let let_expr = syn::ExprLet {
        attrs: Vec::new(),
        let_token: syn::token::Let::default(),
        pat: Box::new(syn::Pat::Ident(syn::PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: syn::Ident::new("x", proc_macro2::Span::call_site()),
            subpat: None,
        })),
        eq_token: syn::token::Eq::default(),
        expr: Box::new(syn::Expr::Struct(syn::ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: {
                    let mut path = syn::punctuated::Punctuated::new();

                    // if the path is empty ✅, but if there is any struct type then it fails
                    path.push_value(syn::PathSegment {
                        ident: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });

                    path
                },
            },
            brace_token: syn::token::Brace::default(),
            fields: syn::punctuated::Punctuated::new(),
            dot2_token: None,
            rest: None,
        })),
    };

    use quote::ToTokens;
    println!("Tokens: {}", let_expr.clone().into_token_stream());
    let tks = let_expr.into_token_stream();

    // fail!
    check_parse2::<syn::ExprLet>(tks);
}
