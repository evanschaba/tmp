extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields /*Meta*/};

// Define the #[derive(Stats)] macro for structs
#[proc_macro_derive(Stats, attributes(stats))]
pub fn stats_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Field info generation
    let mut field_info = quote! {}; // To store the generated code for field stats

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in fields_named.named.iter() {
                let field_name = &field.ident;

                // Check if the field has a #[stats] attribute
                let mut has_stats_attr = false;
                for attr in &field.attrs {
                    if is_stats_attr(attr) {
                        has_stats_attr = true;
                        break;
                    }
                }

                // Generate code for fields with #[stats]
                if has_stats_attr {
                    field_info.extend(quote! {
                        println!(
                            "Field `{}`: memory address: {:p}, size: {} bytes",
                            stringify!(#field_name),
                            &self.#field_name,
                            std::mem::size_of_val(&self.#field_name)
                        );
                    });
                }
            }
        }
    }

    // Generate methods for struct-level stats and field-level stats
    let expanded = quote! {
        impl #struct_name {
            pub fn print_memory_address(&self) {
                // Print the memory address of the struct
                println!("Memory address of {}: {:p}", stringify!(#struct_name), self);
            }

            pub fn print_size(&self) {
                // Print the size of the struct
                let size = std::mem::size_of_val(self);
                println!("Size of {}: {} bytes", stringify!(#struct_name), size);
            }

            pub fn print_field_stats(&self) {
                // Print memory stats for fields with #[stats]
                println!("Memory stats for fields of struct {}:", stringify!(#struct_name));
                #field_info
            }
        }
    };

    TokenStream::from(expanded)
}

// Helper function to check if the attribute is #[stats]
fn is_stats_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("stats")
}
