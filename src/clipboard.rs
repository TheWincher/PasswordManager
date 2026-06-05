pub enum ClipboardError {
    Init,
    Copy,
}

pub fn copy(text: &str) -> Result<(), ClipboardError> {
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ClipboardError::Init)?;
    clipboard.set_text(text).map_err(|_| ClipboardError::Copy)
}
