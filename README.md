# rust_in_action_study_ch11-12

## require

- Visual Studio Code
    - use devcontainer
- Docker Desktop
- Xquartz
    - ホスト側でXquartz起動の上、 `xhost + localhost` の実行が必要
        - `brew install xquartz`
        - add `/usr/X11/bin` to $PATH
    - see https://gist.github.com/cschiewek/246a244ba23da8b9f0e7b11a68bf3285

## what

- 詳解Rustプログラミングの写経用
    - 第11章 カーネル
        - Rust nightly build
        - QEMU

## memo

- x86_64のバージョンを0.13 -> 0.14に変更
    - see https://github.com/rust-in-action/code/issues/44

## thanks

- https://zenn.dev/karaage0703/articles/1bdb8930182c6c
- https://blog.aoirint.com/entry/2020/xquartz_docker/
