use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

#[proc_macro_attribute]
pub fn trace_and_log(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let params = input_fn.sig.inputs.iter().map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(ident) = &*pat_type.pat {
                let param_name = ident.ident.to_string();
                let param_value = quote!(#ident);
                let param_type = &pat_type.ty;
                quote!(format!("{}: {}({:?})", #param_name, stringify!(#param_type), #param_value))
            } else {
                quote!("")
            }
        } else {
            quote!("self")
        }
    });

    // Handle function return type to decide if we log it
    let output_logging = if input_fn.sig.output.is_empty() {
        quote! {
            // No output to log
            let _ = (|| #fn_block)();
        }
    } else {
        quote! {
            // Execute the original function and capture output
            let output = (|| #fn_block)();
            let output_mem_ptr = &output as *const _ as usize;
            let output_mem_size = std::mem::size_of_val(&output);

            // Generate log entry for this function
            let log_data = format!(
                "[INFO][{}][{}ms] {}({}) -> result ({}({}@0x{:X}&{} bytes))",
                timestamp,
                duration.as_millis(),
                stringify!(#fn_name),
                param_log_str,
                stringify!(output),
                output,
                output_mem_ptr,
                output_mem_size
            );

            // Print trace data to stdout
            println!("{}", log_data);
            // Append trace data to file
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("target/logs/traces.log")
                .expect("Unable to open log file.");
            writeln!(file, "{}", log_data).expect("Unable to write log file.");

            output
        }
    };

    let expanded = quote! {
        #fn_sig {
            use std::time::Instant;
            use chrono::Local;
            use std::fs::OpenOptions;
            use std::io::Write;

            let start_time = Instant::now();
            let param_log: Vec<String> = vec![#(#params),*];
            let param_log_str = param_log.join(", ");
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            #output_logging
        }
    };

    TokenStream::from(expanded)
}
