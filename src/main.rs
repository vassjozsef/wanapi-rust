use core::ffi::c_void;
use std::{
    mem,
    ptr::{self, null_mut},
};
use widestring::U16String;
use winapi::{
    shared::winerror::HRESULT, um::combaseapi::CoInitializeEx, um::objbase::COINIT_MULTITHREADED,
    winrt::roapi::RoActivateInstance, winrt::winstring::WindowsCreateString,
    winrt::winstring::WindowsGetStringRawBuffer,
};

pub type RawPtr = *mut core::ffi::c_void;

fn main() {
    let hr = unsafe { CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED) };
    dbg!(hr);

    let mut name = U16String::from("Windows.Foundation.Collections.StringMap");
    let mut hstr = ptr::null_mut();
    let hr = unsafe { WindowsCreateString(name.as_mut_ptr(), name.len() as u32, &mut hstr) };
    dbg!(hr);
    let mut instance = ptr::null_mut();
    let hr = unsafe { RoActivateInstance(hstr, &mut instance) };
    dbg!(hr);
    let mut class_name = ptr::null_mut();
    let hr = unsafe {
        instance
            .as_ref()
            .unwrap()
            .GetRuntimeClassName(&mut class_name)
    };
    dbg!(hr);
    let mut class_name_str_len = 0;
    let class_name_str_ptr =
        unsafe { WindowsGetStringRawBuffer(class_name, &mut class_name_str_len) };
    let class_name_str =
        unsafe { U16String::from_ptr(class_name_str_ptr, class_name_str_len as usize) };
    dbg!(class_name_str);

    //pump
    create_dispatcher_queu_controller();
}

pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);

pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);

#[repr(C)]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}

extern "system" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherqueuecontroller: *mut RawPtr,
    ) -> HRESULT;
}

fn create_dispatcher_queu_controller() {
    let options = DispatcherQueueOptions {
        dwSize: mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_STA,
    };

    let mut controller = null_mut();
    let hr = unsafe { CreateDispatcherQueueController(options, &mut controller) };
    dbg!(hr);
    dbg!(controller);
}
