use winapi::shared::winerror::{HRESULT, S_OK};
pub struct WinResult;
impl WinResult {
    pub fn from(hr: HRESULT) -> Result<(), i32> {
        match hr {
            S_OK => Ok(()),
            err => Err(err),
        }
    }
}
