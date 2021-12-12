// #[macro_export]
// macro_rules! append_attrs {
//     ($document:ident, $el:ident, $( $attr:expr ),* ) => {
//         $(
//             let attr = $document.create_attribute($attr.0)?;
//             attr.set_value($attr.1);
//             $el.set_attribute_node(&attr)?;
//         )*
//     }
// }

// #[macro_export]
// macro_rules! append_text_child {
//     ($document:ident, $el:ident, $text:expr ) => {
//         let text = $document.create_text_node($text);
//         $el.append_child(&text)?;
//     };
// }

// #[macro_export]
// macro_rules! create_element_attrs {
//     ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
//         let el = $document.create_element($type)?;
//         append_attrs!($document, el, $( $attr ),*);
//         el}
//     }
// }

// #[macro_export]
// macro_rules! append_element_attrs {
//     ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
//         let el = create_element_attrs!($document, $type, $( $attr ),* );
//         $parent.append_child(&el)?;
//     }
// }

// #[macro_export]
// macro_rules! append_text_element_attrs {
//     ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
//         let el = create_element_attrs!($document, $type, $( $attr ),* );
//         append_text_child!($document, el, $text);
//         $parent.append_child(&el)?;
//     }
// }
