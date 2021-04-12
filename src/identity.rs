pub fn gen_identity<T>() -> impl Fn(T) -> T {
    |x| x
}
