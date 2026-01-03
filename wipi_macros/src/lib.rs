use quote::TokenStreamExt;

#[proc_macro_attribute]
pub fn wipi_main(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_ts = proc_macro2::TokenStream::from(input);

    let output_ts = transform_wipi_main(input_ts);

    proc_macro::TokenStream::from(output_ts)
}

fn transform_wipi_main(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    enum State {
        None = 0,
        Fn,
        MainFnName,
    }

    let mut state = State::None;

    for item in input.into_iter() {
        if let proc_macro2::TokenTree::Ident(_) = &item {
            match state {
                State::None => {
                    if item.to_string() == "fn" {
                        state = State::Fn;
                    }
                }
                State::Fn => {
                    state = State::MainFnName;
                    output.extend(quote::quote! { __app_main });
                    continue;
                }
                State::MainFnName => {
                    state = State::None;
                }
            }
        }

        output.append(item);
    }

    quote::quote! {
        #output

        #[unsafe(no_mangle)]
        fn __wipi_main() -> alloc::boxed::Box<dyn wipi::app::App> {
            alloc::boxed::Box::new(__app_main())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_wipi_main() {
        let input = quote::quote! {
            fn main() {
                println!("Hello, world!");
            }
        };

        let output = transform_wipi_main(input);

        let expected_output = quote::quote! {
            fn __app_main() {
                println!("Hello, world!");
            }

            #[unsafe(no_mangle)]
            fn __wipi_main() -> alloc::boxed::Box<dyn wipi::app::App> {
                alloc::boxed::Box::new(__app_main())
            }
        };

        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
