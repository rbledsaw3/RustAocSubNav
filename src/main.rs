fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(7);
    }

    return Ok(());
}


fn main() -> Result<(), usize> {
    
    error_me(false)?;

    // This is the same as:
    // let value = match error_me(false) {
        // Err(e) => return Err(e),
        // Ok(v) => v,
    // };

    return Ok(());

}

