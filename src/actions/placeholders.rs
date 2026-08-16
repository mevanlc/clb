pub fn execute(name: &str, is_deferred: bool) -> Result<(), String> {
    if is_deferred {
        println!("clb: '{}' is deferred and not implemented yet.", name);
    } else {
        println!("clb: '{}' is not implemented yet.", name);
    }
    Ok(())
}
