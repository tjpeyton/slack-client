fn main() {
    const NAME : &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    println!("{0} v{1}", NAME, VERSION);
}
