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

---
### 方針
#### CreateRemoteThread関数を利用する
1. 攻撃対象のプロセス仮想メモリに挿入するDLLのファイル名を書き込む（VirtualAllocEx, WriteProcessMemory）
2. 挿入するDLLを攻撃対象のプロセスに読み込ませる（CreateRemoteThread)
3. 攻撃対象のプロセスの関数のアドレスを挿入したDLL内の関数のアドレスで上書き