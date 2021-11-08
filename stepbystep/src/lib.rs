use proc_macro::TokenStream;
use quote::{quote, format_ident};

#[proc_macro_attribute]
pub fn export_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = syn::parse_macro_input!(item);
    let ident = &input.sig.ident;
    let poll_ident = format_ident!("{}_poll", ident);
    let init_ident = format_ident!("{}_init", ident);
    let tokens = quote!{
        #input

        type Task = std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>;

        thread_local! {
            static RUNNER: std::cell::RefCell<std::collections::HashMap<usize, Task>> = std::cell::RefCell::new(std::collections::HashMap::new());
        }

        #[no_mangle]
        pub fn #init_ident(idx: usize) {
            RUNNER.with(|runner| {
                runner.borrow_mut().insert(idx, std::boxed::Box::pin(#ident(idx)));
            })
        }

        #[no_mangle]
        pub fn #poll_ident(idx: usize) -> bool {
            RUNNER.with(|runner| {
                let waker = futures::task::noop_waker();
                let mut cx = std::task::Context::from_waker(&waker);
                runner.borrow_mut().get_mut(&idx).unwrap().as_mut().poll(&mut cx).is_ready()
            })
        }
        
    };
    println!("Tokens: {}", tokens);
    TokenStream::from(tokens)
}