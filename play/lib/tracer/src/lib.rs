use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

#[proc_macro_attribute]
pub fn trace_and_log(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    // Capture function parameters and names
    let params = input_fn.sig.inputs.iter().map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(ident) = &*pat_type.pat {
                let param_name = ident.ident.to_string();
                let param_value = quote!(#ident);
                let param_type = quote!(#pat_type.ty);
                quote!(format!("{} {} = {:?}", #param_name, stringify!(#param_type), #param_value))
            } else {
                quote!("")
            }
        } else {
            quote!("self")
        }
    });

    let expanded = quote! {
        #fn_sig {
            use std::time::Instant;
            use chrono::Local;
            use std::fs::OpenOptions;
            use std::io::Write;
            use std::mem::size_of_val;

            let start_time = Instant::now();

            // Capture function arguments and build param log
            let param_log: Vec<String> = vec![#(#params),*];
            let param_log_str = param_log.join(", ");

            // Execute the original function and capture output
            let output = (|| #fn_block)();
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // Memory information of the output
            let output_mem_ptr = &output as *const _ as usize;
            let output_mem_size = size_of_val(&output);

            // Generate log entry for this function
            let log_data = format!(
                "[INFO][{}][{}ms] {}({}) -> result: {:?} (mem @ 0x{:X}, {} bytes)",
                timestamp,
                duration.as_millis(),
                stringify!(#fn_name),
                param_log_str,
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

    TokenStream::from(expanded)
}
