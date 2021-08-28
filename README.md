# DLLi_in_Rust
### RustでのDLL injection学習

---
### DLLiとは
DLLiはAPI Hookの手法の一つ
他にはWriteProcessMemory APIを利用したCode InjectionやWin32 Debug API toolsetsを利用する方法等がある．

---
### DLLiを行う方法
- SetWindowsHookEx関数の利用 => GUIアプリのみ対象
- CreateRemoteThread関数の利用 => どのプロセスにも使えるが複雑
- Remote Thread Context Putching(?)の利用 => 上二つよりさらに複雑 効果は大

(他にもCallNextHookEx，UnhookWindowsHookEx関数などが利用できる)

---
### SetWindowsHookEx関数を使ったDLLiの流れ
1. あるプログラムのスレッドがウィンドウに表示するメッセージを送ろうとする
2. そのメッセージを途中でフックする
3. 挿入したいDLLがプログラムのメモリ空間へマップされる
4. InjectされたDLLのDLLMainが呼び出される

