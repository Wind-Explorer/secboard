use clipboard::{ClipboardContext, ClipboardProvider};

///
/// Write to the clipboard
/// ## Arguments
/// * `data` - Data to write
/// ## Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Result
///
pub fn write(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(data.to_owned()).unwrap();
    return Ok(());
}

///
/// Read from the clipboard
/// ## Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Data
///
pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let data = ctx.get_contents().unwrap();
    return Ok(data);
}
