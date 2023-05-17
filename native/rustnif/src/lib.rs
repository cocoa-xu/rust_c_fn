extern "C" fn my_add_rs(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn pointer_to_my_add_rs() -> u64 {
    unsafe {
        std::mem::transmute::<_, u64>(my_add_rs as extern "C" fn(_, _) -> _)
    }
}

rustler::init!("Elixir.RustNIF", [
    pointer_to_my_add_rs
]);
