---
header: "![Header](./assets/Gear.png)"
paginate: true
theme: default
class:
- lead
marp: true
style: |
  header {
    position: absolute;
    float: left;
    left: 950px;
    top: -80px;
  }
  h1 {
    border-left: 15px solid #6666FF;
    padding-left: 1rem;
    color: #131819;
  }
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
  .qr {
    padding-top: 10%;
  }
  .gitpod {
    padding-top: 32%;
  }
  img{background-color:transparent}
size: 16:9
---

![bg](./assets/BackgroundGearBlack.png)

<!-- _color: #FFF -->

# 15 分钟上手 Gear 智能合约

<div class="columns">
<div>

<br/>
<br/>
<br/>
<br/>
<br/>

## Hangbiao

### ⚙️ Validator Manager

</div>
<div>

<br/>
<br/>

- **什么是 Gear**
- **Gear 智能合约基本特性**

<!--
  - **Rust / WebAssembly**
  - **Actor Model** / **Asynchronous Messaging**
  - **Persistent Memory**
-->

- **上手示例: Flipper Contract**

<!--
  - **消息类型**
  - **内部状态**
  - **消息处理**
  - **元数据**
  - **编写测试**
-->

</div>
</div>

---

<!-- ![bg](./assets/BackgroundGearWhite.png) -->
![bg](./assets/Ambient.png)

# 什么是 Gear

## 基于 Substrate 的智能合约平台 (PoS, L1)

<div class="columns">
<div>

> ##### Smart Contract Engine for Polkadot

<!--

- ### Actor Model

- ### Persistent Memory

- ### WebAssembly

-->

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

Coming soon! ([source](https://github.com/paritytech/ss58-registry/blob/13019a7d23901c499d97855ba6c2145962c42fd0/ss58-registry.json#L787-L795))

</details>

<details><summary>Kusama & Polkadot Parachain</summary>

Stay tuned!

</details>

</div>
</div>

<!-- 近期将上线 Vara Network 主网 -->

---

![bg](./assets/Ambient.png)

# Gear 智能合约基本特性

  - **Rust / WebAssembly**
  - **Actor Model** / **Asynchronous Messaging**
  - **Persistent Memory**

---

![bg](./assets/Ambient.png)

# 开发环境搭建

```
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown
```

或 `./rust-toolchain`

```
[toolchain]
channel = "nightly"
components = [ "rustfmt", "clippy" ]
targets = [ "wasm32-unknown-unknown" ]
profile = "minimal"
```
---

![bg](./assets/Ambient.png)

# Flipper Contract

初始状态: 🌚
flip => 🌝
flip => 🌚
flip => 🌝
flip => 🌚
...

---

![bg](./assets/Ambient.png)

# Flipper Contract: 初始化项目

## `cargo init ...`

```
$ cargo init --lib gear-flipper
```

## `Cargo.toml`

```
...
[dependencies]
gstd = { git = "https://github.com/gear-tech/gear.git", branch = "stable" }
scale-info = { version = "2", default-features = false, features = ["derive"] }
parity-scale-codec = { version = "3", default-features = false, features = ["derive"] }

[build-dependencies]
gear-wasm-builder = { git = "https://github.com/gear-tech/gear.git", branch = "stable" }

[dev-dependencies]
gtest = { git = "https://github.com/gear-tech/gear.git", branch = "stable" }
```

---

![bg](./assets/Ambient.png)

# Flipper Contract: 配置 WebAssembly Builder

## `./build.rs`

```
fn main() {
    gear_wasm_builder::build();
}
```

---

![bg](./assets/Ambient.png)

# Flipper Contract: 目录结构

```
.
├── build.rs
├── Cargo.toml
├── rust-toolchain
└── src
    ├── io.rs
    ├── lib.rs
    └── tests.rs
```

---

# Flipper Contract: 消息定义

![bg](./assets/Ambient.png)

`./src/io.rs`

```
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FlipperAction {
    Flip,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FlipperEvent {
    FlippedTo(u8),
}
```

---

# Flipper Contract: 元数据


![bg](./assets/Ambient.png)

```
use gstd::prelude::*;

gstd::metadata! {
  title: "flipper",
  handle:
    input: FlipperAction,
    output: FlipperEvent,
}
```

---

# Flipper Contract: 合约状态

![bg](./assets/Ambient.png)

```
static mut FlipperState: bool = false;
```

---

# Flipper Contract: 消息处理

![bg](./assets/Ambient.png)

```
#[no_mangle]
unsafe extern "C" fn handle() {
    let action: FlipperAction = gstd::msg::load().expect("failed to load input message");
    match action {
        FlipperAction::Flip => {
            FlipperState = !FlipperState;
            let event = FlipperEvent::FlippedTo(FlipperState as u8);
            gstd::msg::reply(event, 0).expect("failed to send response");
        },
    }
}
```

---

# Flipper Contract: 单元测试

![bg](./assets/Ambient.png)

```
use gtest::{Program, System};

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send_bytes(0, "let's goooo!");

    let res = program.send(42, FlipperAction::Flip);
    assert_eq!(res.main_failed(), false);

    ...
}
```

---

![bg](./assets/Ambient.png)

# Follow Us

<div class="columns">

<div>

<img src="./assets/GearTwitter.jpg" width=300 height=300/>

</div>

<div>

<img src="./assets/GearWechat.jpg" width=300 height=300/>

</div>

</div>

### 👉 https://wiki.gear-tech.io/docs/examples/prerequisites
