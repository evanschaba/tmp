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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    #[derive(Stats)]
    struct TestStruct {
        a: u32,
        #[stats]
        b: f64,
        #[stats]
        c: [u8; 128],
    }

    fn capture_output<F>(func: F) -> String
    where
        F: FnOnce(),
    {
        let mut buf = Vec::new();
        let writer = io::stdout();
        let _guard = writer.lock();
        func();
        _guard.flush().unwrap();
        buf.clear(); // Reset buffer

        String::from_utf8(buf).unwrap_or_default()
    }

    #[test]
    fn test_memory_address() {
        let instance = TestStruct {
            a: 42,
            b: 3.14,
            c: [0; 128],
        };
        let output = capture_output(|| instance.print_memory_address());
        assert!(output.contains("Memory address of TestStruct:"));
    }

    #[test]
    fn test_size() {
        let instance = TestStruct {
            a: 42,
            b: 3.14,
            c: [0; 128],
        };
        let output = capture_output(|| instance.print_size());
        assert!(output.contains("Size of TestStruct:"));
    }

    #[test]
    fn test_field_stats() {
        let instance = TestStruct {
            a: 42,
            b: 3.14,
            c: [0; 128],
        };
        let output = capture_output(|| instance.print_field_stats());
        assert!(output.contains("Field `b`: memory address:"));
        assert!(output.contains("Field `c`: memory address:"));
    }

    #[test]
    fn test_non_stats_field() {
        let instance = TestStruct {
            a: 42,
            b: 3.14,
            c: [0; 128],
        };
        let output = capture_output(|| instance.print_field_stats());
        // `a` does not have the #[stats] attribute, so its stats should not be printed.
        assert!(!output.contains("Field `a`: memory address:"));
    }
}
