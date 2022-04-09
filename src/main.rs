use ari::os::win::ComPtr;
use std::os::windows::prelude::*;
use std::{ffi::OsString, mem, ptr};
use winapi::winrt::roapi::RoGetActivationFactory;
use winapi::Interface;
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
    winrt::{inspectable::IInspectable, roapi::RoActivateInstance},
};

use crate::dispatcher::{
    CreateDispatcherQueueController, DispatcherQueueOptions, IDispatcherQueueController,
    DQTAT_COM_STA, DQTYPE_THREAD_CURRENT,
};
use crate::graphics::{IGraphicsCaptureItem, IGraphicsCaptureItemInterop, SizeInt32};
use crate::util::{from_hstring, print_runtime_class_name, to_hstring, WinResult};

mod dispatcher;
mod graphics;
mod util;

#[derive(Debug, Clone)]
pub struct Window {
    pub hwnd: HWND,
    pub name: OsString,
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
    WinResult::from(unsafe { CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED) })?;

    let name = to_hstring("Windows.Foundation.Collections.StringMap".to_string())?;
    let mut instance = ptr::null_mut();
    WinResult::from(unsafe { RoActivateInstance(name, &mut instance) })?;
    let string_map = unsafe { instance.as_ref().unwrap() };
    print_runtime_class_name(string_map);

    // create IDispatcherQueue
    let controller = create_dispatcher_queue_controller()?;
    print_runtime_class_name(&controller);
    let _queue = unsafe { controller.DispatcherQueue() };

    // enumerate windows
    let mut windows: Vec<Window> = Vec::new();
    let res = unsafe {
        EnumWindows(
            Some(enum_window),
            &mut windows as *mut Vec<Window> as LPARAM,
        )
    };
    if res == FALSE {
        return Err(-1);
    }
    dbg!(&windows);

    // create GrpahicsCaptureItem
    let name = to_hstring("Windows.Graphics.Capture.GraphicsCaptureItem".to_string())?;
    let mut ptr = std::ptr::null_mut();
    WinResult::from(unsafe {
        RoGetActivationFactory(name, &IGraphicsCaptureItemInterop::uuidof(), &mut ptr)
    })?;
    let interop = unsafe { (ptr as *mut IGraphicsCaptureItemInterop).as_ref().unwrap() };

    let mut ptr = ptr::null_mut();
    let hwnd = windows[2].hwnd;
    dbg!(hwnd);
    WinResult::from(unsafe {
        interop.CreateForWindow(hwnd, &IGraphicsCaptureItem::uuidof(), &mut ptr)
    })?;
    let item = unsafe { (ptr as *mut IGraphicsCaptureItem).as_ref().unwrap() };
    print_runtime_class_name(item);

    let mut name = std::ptr::null_mut();
    WinResult::from(unsafe { item.DisplayName(&mut name) })?;
    dbg!(from_hstring(name));
    let mut size = SizeInt32 {
        Width: 0,
        Height: 0,
    };
    WinResult::from(unsafe { item.Size(&mut size) })?;
    dbg!(size);

    // create IDirectD3Device
    let device = create_d3d_device()?;
    let dxgi_device = device.query::<IDXGIDevice>()?;
    let mut instance = std::ptr::null_mut();
    WinResult::from(unsafe {
        CreateDirect3D11DeviceFromDXGIDevice(
            dxgi_device.as_mut_ptr() as *mut core::ffi::c_void,
            &mut instance,
        )
    })?;
    let direct3d_device = unsafe { instance.as_ref().unwrap() };
    print_runtime_class_name(direct3d_device);

    Ok(())
}

extern "system" {
    fn CreateDirect3D11DeviceFromDXGIDevice(
        dxgidevice: *mut core::ffi::c_void,
        graphicsdevice: *mut *mut IInspectable,
    ) -> HRESULT;
}

fn create_dispatcher_queue_controller() -> Result<ComPtr<IDispatcherQueueController>, i32> {
    let options = DispatcherQueueOptions {
        dw_size: mem::size_of::<DispatcherQueueOptions>() as u32,
        thread_type: DQTYPE_THREAD_CURRENT,
        apartment_type: DQTAT_COM_STA,
    };

    let mut controller = ptr::null_mut();
    let hr = unsafe { CreateDispatcherQueueController(options, &mut controller) };
    match hr {
        S_OK => Ok(unsafe { ComPtr::new(controller) }),
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
