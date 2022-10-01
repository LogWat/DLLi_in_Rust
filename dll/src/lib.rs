#![allow(non_snake_case)]

use winapi::{
    shared::minwindef::{HINSTANCE, LPVOID},
    um::{
        winnt::{
            DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH,
        },
        libloaderapi,
    },
    um::winuser::{
        MessageBoxW, MB_OK,
    },
};

#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    _lpvReserved: LPVOID,
) -> i32 {
    match fdwReason {
        DLL_PROCESS_ATTACH => {
            unsafe { libloaderapi::DisableThreadLibraryCalls(hinstDLL) };
            println!("DLL_PROCESS_ATTACH");
            MsgBox("INJECTED!", "Hello from DLL!");
            true as i32
        },
        DLL_PROCESS_DETACH => {
            println!("DLL_PROCESS_DETACH");
            true as i32
        },
        _ => {
            println!("Unknown reason");
            true as i32
        }
    }
}

fn MsgBox(text: &str, title: &str) {
    let lp_text = text.encode_utf16().collect::<Vec<u16>>();
    let lp_title = title.encode_utf16().collect::<Vec<u16>>();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_title.as_ptr(),
            MB_OK,
        );
    }
}