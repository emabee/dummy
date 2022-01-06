fn main() {
    let s_ts = "2021-10-29_14-29-22";
    println!(
        "{:?}",
        time::PrimitiveDateTime::parse(s_ts, "%Y-%m-%d_%H-%M-%S"),
    );
}
