#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ari::os::win::ComPtr;
use dispatcher::IDispatcherQueueController;
use std::os::windows::prelude::*;
use std::{ffi::OsString, fmt, mem, ptr};
use widestring::U16String;
use winapi::{
    shared::{
        dxgi::IDXGIDevice,
        minwindef::{BOOL, FALSE, LPARAM, TRUE},
        windef::HWND,
        winerror::{HRESULT, S_OK},
    },
    um::combaseapi::CoInitializeEx,
    um::dwmapi::DwmGetWindowAttribute,
    um::objbase::COINIT_MULTITHREADED,
    um::{
        d3d11::{
            D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
        },
        d3dcommon::{
            D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_9_1,
            D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3,
        },
        dwmapi::{DWMWA_CLOAKED, DWM_CLOAKED_SHELL},
        winuser::{
            EnumWindows, GetAncestor, GetShellWindow, GetWindowLongA, GetWindowTextW,
            IsWindowVisible, GA_ROOT, GWL_STYLE, WS_DISABLED,
        },
    },
    winrt::{
        inspectable::IInspectable,
        roapi::RoActivateInstance,
        winstring::{WindowsCreateString, WindowsGetStringRawBuffer},
    },
};

pub type RawPtr = *mut core::ffi::c_void;

mod dispatcher;

pub struct Window {
    pub hwnd: HWND,
    pub name: OsString,
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("hwnd", &self.hwnd)
            .field("name", &self.name)
            .finish()
    }
}

const DIRECT3D_FEATURE_LEVELS: &[u32] = &[
    D3D_FEATURE_LEVEL_11_1,
    D3D_FEATURE_LEVEL_11_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_10_0,
    D3D_FEATURE_LEVEL_9_3,
    D3D_FEATURE_LEVEL_9_2,
    D3D_FEATURE_LEVEL_9_1,
];

fn main() -> Result<(), i32> {
    let hr = unsafe { CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED) };
    dbg!(hr);

    let mut name = U16String::from("Windows.Foundation.Collections.StringMap");
    let mut hstr = ptr::null_mut();
    let hr = unsafe { WindowsCreateString(name.as_mut_ptr(), name.len() as u32, &mut hstr) };
    dbg!(hr);
    let mut instance = ptr::null_mut();
    let hr = unsafe { RoActivateInstance(hstr, &mut instance) };
    dbg!(hr);
    let string_map = unsafe { instance.as_ref().unwrap() };
    print_runtime_class_name(string_map);

    //pump
    let controller = create_dispatcher_queu_controller()?;
    print_runtime_class_name(&controller);
    let _queue = unsafe { controller.DispatcherQueue() };

    // enumerate windows
    let mut windows = Vec::new();
    let res = unsafe {
        EnumWindows(
            Some(enum_window),
            &mut windows as *mut Vec<Window> as LPARAM,
        )
    };
    if res == FALSE {
        return Err(-1);
    }
    dbg!(windows);

    let device = create_d3d_device()?;
    let dxgi_device = device.query::<IDXGIDevice>()?;
    let mut instance = std::ptr::null_mut();
    let hr = unsafe {
        CreateDirect3D11DeviceFromDXGIDevice(dxgi_device.as_mut_ptr() as RawPtr, &mut instance)
    };
    if hr != S_OK {
        return Err(hr);
    }
    let direct3d_device = unsafe { instance.as_ref().unwrap() };

    print_runtime_class_name(direct3d_device);

    Ok(())
}

fn print_runtime_class_name(class: &IInspectable) {
    let mut class_name = ptr::null_mut();
    let hr = unsafe { class.GetRuntimeClassName(&mut class_name) };
    dbg!(hr);
    let mut class_name_str_len = 0;
    let class_name_str_ptr =
        unsafe { WindowsGetStringRawBuffer(class_name, &mut class_name_str_len) };
    let class_name_str =
        unsafe { U16String::from_ptr(class_name_str_ptr, class_name_str_len as usize) };
    dbg!(class_name_str);
}

#[repr(C)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);

#[repr(C)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE =
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);

#[repr(C)]
pub struct DispatcherQueueOptions {
    pub dw_size: u32,
    pub thread_type: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}

extern "system" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        // TODO: Actually return a pointer to DispatcherQueueController
        dispatcherqueuecontroller: *mut *mut IDispatcherQueueController,
    ) -> HRESULT;
}

extern "system" {
    fn CreateDirect3D11DeviceFromDXGIDevice(
        dxgidevice: RawPtr,
        graphicsdevice: *mut *mut IInspectable,
    ) -> HRESULT;
}

fn create_dispatcher_queu_controller() -> Result<&'static IDispatcherQueueController, i32> {
    let options = DispatcherQueueOptions {
        dw_size: mem::size_of::<DispatcherQueueOptions>() as u32,
        thread_type: DQTYPE_THREAD_CURRENT,
        apartment_type: DQTAT_COM_STA,
    };

    let mut controller = ptr::null_mut();
    let hr = unsafe { CreateDispatcherQueueController(options, &mut controller) };
    dbg!(hr);
    dbg!(controller);
    match hr {
        S_OK => Ok(unsafe { controller.as_ref().unwrap() }),
        error => Err(error),
    }
}

fn create_d3d_device() -> Result<ComPtr<ID3D11Device>, i32> {
    let levels = DIRECT3D_FEATURE_LEVELS;
    let flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    let driver_type = D3D_DRIVER_TYPE_HARDWARE;

    let mut device = std::ptr::null_mut();
    let mut context = std::ptr::null_mut();

    let hr = unsafe {
        D3D11CreateDevice(
            std::ptr::null_mut(),
            driver_type,
            std::ptr::null_mut(),
            flags,
            levels.as_ptr(),
            levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut device,
            std::ptr::null_mut(),
            &mut context,
        )
    };

    dbg!(hr);

    let device = unsafe { ComPtr::new(device) };

    match hr {
        S_OK => Ok(device),
        error => Err(error),
    }
}

extern "system" fn enum_window(handle: HWND, data: LPARAM) -> BOOL {
    let shell_window = unsafe { GetShellWindow() };
    if handle == shell_window {
        return TRUE;
    }
    unsafe {
        if IsWindowVisible(handle) == FALSE {
            return TRUE;
        }
    }

    unsafe {
        if GetAncestor(handle, GA_ROOT) != handle {
            return TRUE;
        }
    }

    unsafe {
        let style = GetWindowLongA(handle, GWL_STYLE) as u32;
        if style & WS_DISABLED == WS_DISABLED {
            return TRUE;
        }
    }

    unsafe {
        let mut cloaked: i32 = 0;
        let ptr = &mut cloaked as *mut _ as *mut _;
        let hr = DwmGetWindowAttribute(handle, DWMWA_CLOAKED, ptr, mem::size_of::<i32>() as u32);
        if hr == S_OK && cloaked as u32 == DWM_CLOAKED_SHELL {
            return TRUE;
        }
    }

    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(handle, text.as_mut_ptr(), text.len() as i32);
        if len == 0 {
            return TRUE;
        }
        let text = OsString::from_wide(&text[..len as usize]);

        let windows = &mut *(data as *mut Vec<Window>);
        windows.push(Window {
            hwnd: handle,
            name: text,
        });

        return TRUE;
    }
}
