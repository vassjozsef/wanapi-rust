#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::winrt::inspectable::{IInspectable, IInspectableVtbl};
use winapi::RIDL;

RIDL! {#[uuid(0x603e88e4, 0xa338, 0x4ffe, 0xa4, 0x57, 0xa5, 0xcf, 0xb9, 0xce, 0xb8, 0x99)]
    interface IDispatcherQueue(IDispatcherQueue_Vtbl): IUnknown(IUnknownVtbl) {
}}

RIDL! {#[uuid(0x22f34e66, 0x50db, 0x4e36, 0xa9, 0x8d, 0x61, 0xc0, 0x1b, 0x38, 0x4d, 0x20)]
    interface IDispatcherQueueController(IDispatcherQueueControllerVtbl) :  IInspectable(IInspectableVtbl){
    fn DispatcherQueue() -> IDispatcherQueue,
}}

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
    pub fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        // TODO: Actually return a pointer to DispatcherQueueController
        dispatcherqueuecontroller: *mut *mut IDispatcherQueueController,
    ) -> HRESULT;
}
