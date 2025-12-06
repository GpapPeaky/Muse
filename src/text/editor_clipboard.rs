use copypasta::{ClipboardContext, ClipboardProvider};

pub fn cb_set(text: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text.to_string()).unwrap();
}

pub fn cb_get() -> Option<String> {
    let mut ctx = ClipboardContext::new().ok()?;
    ctx.get_contents().ok()
}
