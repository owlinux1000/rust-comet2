# COMET II Emulator written in Rust

Rustの勉強用に書いたCOMET IIエミュレータ

## 使い方

### ツールとして

[rust-casl2](https://git.alicemacs.com/chihiro/rust-casl2)で生成したファイルを用いてエミュレータで実行することができます．

```
$ rust-comet2 sample
```

### ライブラリとして

```
extern crate rust_casl2;
use rust_casl2::emu::Emu;

fn main() {
    let mut emu = Emu::new();
    emu.gr[1] = 0x0;
    emu.gr[2] = 0xdead;
    emu.memory[0] = 0x1412; // LD GR1, GR2
    let code = emu.fetch();
    emu.execute(code);
    println!("{:x}", gr[1]); //=> 0xdead
    println!("{:x}", gr[2]); //=> 0xdead
}
```

## 補足

* SVC命令については未実装
* **Rust初心者なのでRustっぽい書き方を教えてください**

