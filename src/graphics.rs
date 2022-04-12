#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use winapi::shared::guiddef::REFIID;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::winrt::hstring::HSTRING;
use winapi::winrt::inspectable::{IInspectable, IInspectableVtbl};
use winapi::RIDL;

RIDL! {#[uuid(0x3628e81b, 0x3cac, 0x4c60, 0xb7, 0xf4, 0x23, 0xce, 0x0e, 0x0c, 0x33, 0x56)]
    interface IGraphicsCaptureItemInterop(IGraphicsCaptureItemInterop_Vtbl): IUnknown(IUnknownVtbl) {
    fn CreateForWindow(window: HWND, riid: REFIID, result: *mut *mut ::core::ffi::c_void,) -> HRESULT,
}}

#[repr(C)]
#[derive(Debug)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}

RIDL! {#[uuid(0x79c3f95b, 0x31f7, 0x4ec2, 0xa4, 0x64, 0x63, 0x2e, 0xf5, 0xd3, 0x07, 0x60)]
    interface IGraphicsCaptureItem(IGraphicsCaptureItem_Vtbl): IInspectable(IInspectableVtbl) {
    fn DisplayName(value: *mut HSTRING,) -> HRESULT,
    fn Size(value: *mut SizeInt32,) -> HRESULT,
}}

#[repr(C)]
pub struct DirectXPixelFormat(pub i32);
pub const B8G8R8A8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(87i32);

RIDL! {#[uuid(0x7784056a, 0x67aa, 0x4d53, 0xae, 0x54, 0x10, 0x88, 0xd5, 0xa8, 0xca, 0x21)]
    interface IDirect3D11CaptureFramePoolStatics(IDirect3D11CaptureFramePoolStatics_Vtbl): IInspectable(IInspectableVtbl) {
        fn Create(device: *mut IInspectable, pixelformat: DirectXPixelFormat, numberofbuffers: i32, size: SizeInt32, result: *mut  *mut core::ffi::c_void,) -> HRESULT,
}}

RIDL! {#[uuid(0x24eb6d22, 0x1975, 0x422e, 0x82, 0xe7, 0x78, 0x0d, 0xbd, 0x8d, 0xdf, 0x24)]
    interface IDirect3D11CaptureFramePool(IDirect3D11CaptureFramePool_Vtbl): IInspectable(IInspectableVtbl) {
        fn CreateCaptureSession(item: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void,) -> HRESULT,
    }
}
