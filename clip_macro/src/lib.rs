#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// #[cfg(test)]
// mod tests {

//     #[derive(AnswerFn)]
//     struct A {}

//     #[test]
//     fn it_works() {
//         let a = A {};

//         assert_eq!(42, a.answer());
//     }
// }
