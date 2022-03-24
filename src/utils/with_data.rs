use std::sync::{Arc, Mutex};

pub fn with_data<D, R>(data: Arc<Mutex<D>>, func: impl FnOnce(&mut D) -> R) -> Result<R, String> {
    let data = &mut data.lock().map_err(|err| format!("{err}"))?;
    Ok(func(data))
}
