use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Attribute, Meta};

#[proc_macro_attribute]
pub fn sync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let mut sync_fields = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let has_sync = field.attrs.iter().any(|attr| attr.path().is_ident("sync"));
                if has_sync {
                    sync_fields.push(field_name.clone());
                }
            }
        }
    }

    let state_struct_name = format_ident!("{}State", struct_name);

    let state_fields = sync_fields.iter().map(|f| {
        quote! { pub #f: std::sync::Arc<tokio::sync::watch::Sender<u32>>, }
    });

    let state_init = sync_fields.iter().map(|f| {
        quote! { 
            let (#f, _rx) = tokio::sync::watch::channel(0); 
            let #f = std::sync::Arc::new(#f);
        }
    });

    let struct_update_methods = sync_fields.iter().map(|f| {
        let setter_name = format_ident!("set_{}", f);
        quote! {
            pub fn #setter_name(&self, new_value: u32) {
                let _ = self.#f.send(new_value);
                println!("Updated {}: {}", stringify!(#f), new_value);
            }
        }
    });

    let expanded = quote! {
        #[derive(Clone)]
        pub struct #state_struct_name {
            #(#state_fields)*
        }

        impl #state_struct_name {
            pub fn new() -> Self {
                #(#state_init)*
                Self { #(#sync_fields),* }
            }

            #(#struct_update_methods)*
        }
    };

    TokenStream::from(expanded)
}
