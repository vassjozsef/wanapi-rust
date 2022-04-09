#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
