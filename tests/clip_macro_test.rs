#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod clip_maco_integration_tests {
    extern crate clip_macro;
    use clip_macro::AnswerFn;

    #[derive(AnswerFn)]
    struct A {
        pub x: u32,
    }

    #[test]
    fn test_clip_macro() {
        // let a = A { x: 1 };
        assert_eq!(42, answer());
        // assert_eq!(42, a.answer());
    }
}
