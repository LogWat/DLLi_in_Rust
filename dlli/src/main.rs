extern crate user32;
extern crate winapi;

const WH_KEYBOARD_LL: i32 = 13;

fn main() {
    unsafe {
        let hook_id = user32::SetWindowsHookExA(
            WH_KEYBOAD_LL, 
            lpfn: HOOKPROC, 
            hmod: HINSTANCE, 
            dwThreadId: DWORD)
    }
}