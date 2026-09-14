mod zz_build_cfg;
fn main() {
    zz_build_cfg::ensure();
    let rounds: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(16);
    if std::env::args().any(|a| a == "--help") {
        eprintln!("ohlcv-mmap [rounds]");
        return;
    }
    let (job, n) = ohlcv_mmap::work::bench(rounds);
    println!("job={} rounds={}", job, n);
}
