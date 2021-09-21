extern crate user32;
extern crate winapi;

const WH_KEYBOARD_LL: i32 = 13;

fn main() {
    unsafe {
        let hook_id = user32::SetWindowsHookExA(
            WH_KEYBOARD_LL,                     // フックタイプ
            lpfn: HOOKPROC,                     // windowがmessageを処理する際に呼び出す関数のaddr
            hmod: HINSTANCE,                    // DLL
            0                                   // 0: すべてのGUIスレッドにフックを設定
        );
    }
}