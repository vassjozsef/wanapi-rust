use widestring::U16String;
use winapi::shared::winerror::{HRESULT, S_OK};
use winapi::winrt::hstring::HSTRING;
use winapi::winrt::inspectable::IInspectable;
use winapi::winrt::winstring::{WindowsCreateString, WindowsGetStringRawBuffer};

pub struct WinResult;
impl WinResult {
    pub fn from(hr: HRESULT) -> Result<(), i32> {
        match hr {
            S_OK => Ok(()),
            err => Err(err),
        }
    }
}

pub fn to_hstring(value: &str) -> Result<HSTRING, i32> {
    let mut name = U16String::from(value);
    let mut hstr = std::ptr::null_mut();
    WinResult::from(unsafe {
        WindowsCreateString(name.as_mut_ptr(), name.len() as u32, &mut hstr)
    })?;
    Ok(hstr)
}

pub fn from_hstring(value: HSTRING) -> String {
    let mut len = 0;
    let ptr = unsafe { WindowsGetStringRawBuffer(value, &mut len) };
    let str = unsafe { U16String::from_ptr(ptr, len as usize) };
    str.to_string_lossy()
}

pub fn print_runtime_class_name(class: &IInspectable) {
    let mut class_name = std::ptr::null_mut();
    let hr = unsafe { class.GetRuntimeClassName(&mut class_name) };
    match hr {
        S_OK => println!("{:?}", from_hstring(class_name)),
        _ => println!("Runtime class name is unavailable"),
    }
}
