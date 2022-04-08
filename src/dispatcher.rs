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
