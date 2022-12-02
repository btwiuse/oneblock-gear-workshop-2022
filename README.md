---
theme: default
class:
- lead
marp: true
style: |
  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }
  .columns-left {
    background: yellow;
  }
  .columns-right {
    background: beige;
  }
size: 16:9
---

# Gear 合约入门介绍

<div class="columns">
<div>

<h3>
<details open><summary>智能合约特性和开发范例</summary>

<div class="columns">
<p align="center">
  <img width="240" height="240" src="https://api.qrserver.com/v1/create-qr-code/?color=000000&amp;bgcolor=FFFFFF&amp;data=https%3A%2F%2Fgithub.com%2Fbtwiuse%2Foneblock-gear-workshop-2022&amp;qzone=1&amp;margin=0&amp;size=400x400&amp;ecc=L" alt="qr code" />
</p>

<p align="center">
  <a href="https://gitpod.io/#https://github.com/btwiuse/oneblock-gear-workshop-2022" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>
<div/>

</details>
</h3>

## Hangbiao

### ⚙️ Validator Manager

</div>
<div>

- #### [什么是 Gear](#什么是-gear)
  - [Rust 与 WebAssembly](#rust-与-webassembly)
  - [Hello World](#hello-world)
- #### [Actor 模型](#actor-模型)
  - [Processing](#)
  - [Communication](#)
  - [State](#)
- #### [开发范例](#开发范例)

</div>
</div>

---

# 什么是 Gear

## 基于 Substrate 的智能合约平台 (PoS, L1)

<div class="columns">
<div>

### Actor Model

### Persistent Memory

### WebAssembly

</div>
<div>

<details><summary>Vara Standalone Network</summary>

```
...
    {
      "prefix": 137,
      "network": "vara",
      "displayName": "Vara Network",
      "symbols": ["VARA"],
      "decimals": [12],
      "standardAccount": "*25519",
      "website": "https://vara-network.io/"
    },
...
```

Coming soon!
[source](https://github.com/paritytech/ss58-registry/blob/13019a7d23901c499d97855ba6c2145962c42fd0/ss58-registry.json#L787-L795)

</details>

<details><summary>Kusama & Polkadot Parachain</summary>

Stay tuned!

</details>

</div>
</div>

<!-- 近期将上线 Vara Network 主网 -->

---

# Rust 与 WebAssembly

## WebAssembly: 高效、可移植的二进制指令格式 (W3C标准)

- Rust / C / C++ / Zig / AssemblyScript ...

## Rust: 多范式通用编程语言 (Safety, Speed, Concurrency)

- 完善的工具链
  - `wasm32-unknown-unknown`
    [Tier 2](https://doc.rust-lang.org/rustc/platform-support.html#tier-2)
  - [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) ->
    [gstd](https://docs.gear.rs/gstd/)
- `cargo` 包管理器 &
  [crates.io/categories/no-std](https://crates.io/categories/no-std)
- 丰富的插件: `rustfmt`, `cargo-clippy`, `rust-analyzer`, ...

---

# Hello World

## `cargo new`

```
$ cargo new hello-gear && cd hello-gear
```

## `cargo add`

```
$ cargo add --git ...
```

---

## `./rust-toolchain`

```
[toolchain]
channel = "nightly"
components = [ "rustfmt", "clippy" ]
targets = [ "wasm32-unknown-unknown" ]
profile = "minimal"
```

## `./src/lib.rs`

```
#![no_std]

#[no_mangle]
extern "C" fn handle() {
  let _ = gstd::msg::load_bytes(); // read input message and do nothing 
}
```

---

## `./build.rs`

```
fn main() {
    gear_wasm_builder::build();
}
```

## `cargo build --release`

- `./target/wasm32-unknown-unknown/release/*.opt.wasm`
  - 合约代码 (Code)
  - 提交上链 -> CodeId -> 部署 (+salt) -> ProgramId
- `./target/wasm32-unknown-unknown/release/*.meta.wasm`
  - 合约 Metadata
  - @gear-js/api

---

# Actor 模型

## 账户体系

`gstd::ActorId`: 32-bit Unique Identifier

- 合约(Program)地址 (aka ProgramId), 例如
  `0x512905fcf25de5e576c5c9302b75efd68372e9d835945089f046f6170a0ef91a`
- 用户地址 (即 SS58 公钥), 例如:
  `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`
  - `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` //Alice
  - [ss58.org](https://ss58.org)

---

# 开发范例

- [gstd 标准库](#)
- [@gear-js/api](#)

## Ping Pong

## gFT (ERC-20)

## gNFT (ERC-721)

## gMT (ERC-1155)

## DAO
