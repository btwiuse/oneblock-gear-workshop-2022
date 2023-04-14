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
  .columns13 {
    display: grid;
    grid-template-columns: 1fr 3fr;
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

# Gear 智能合约技术纵览

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

- **Gear Protocol**
- **Actor Model**
- **Program Anatomy**
- **Message Dispatching**

<!--
  - **Rust / WebAssembly**
  - **Actor Model** / **Asynchronous Messaging**
  - **Persistent Memory**
  - **上手示例: Flipper Contract**
-->


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

# Gear Protocol

## Smart Contract Platform built on Substrate

<div class="columns">

<div align="center">

![](https://i.imgur.com/ixxN8sf.png)

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


</div>

<div>

<div class="columns13">
<div>

![h:120](https://i.imgur.com/X3XbnIv.png)

![h:120](https://i.imgur.com/sOcLAOY.png)

![h:120](https://i.imgur.com/bBtZ3Zj.png)

</div>

<div>

## Actor Model

<br/>

## WebAssembly

<br/>

## Persistent Memory

</div>

</div>

<!--

- ### Actor Model

- ### Persistent Memory

- ### WebAssembly

-->

</div>

</div>

<!-- 近期将上线 Vara Network 主网 -->

---

# Actor Model - Message

![bg](./assets/Ambient.png)

<div align="center">

![h:550](https://i.imgur.com/9tUxBwx.png)

<div/>


---

# Actor Model - Program

![bg](./assets/Ambient.png)

<div align="center">

![h:550](https://i.imgur.com/2egiONF.png)

<div/>


---

# Program Anatomy

![bg](./assets/Ambient.png)

<div align="center">

![h:550](https://i.imgur.com/w1PZqWo.png)

<div/>

---

# Example Gear WASM Module

![bg](./assets/Ambient.png)

```
(module
    (import "env" "gr_reply_to"  (func $gr_reply_to (param i32)))
    (import "env" "memory" (memory 2))
    (export "handle" (func $handle))
    (export "init" (func $init))
    (func $handle
        i32.const 65536
        call $gr_reply_to
    )
    (func $init)
)
```

https://docs.gear.rs/gstd/index.html

---

# Message Dispatching

![bg](./assets/Ambient.png)

<div align="center">

![h:550](https://i.imgur.com/ZleLcL1.png)

<div/>

---

# @gear-js/api

![bg](./assets/Ambient.png)

<div align="center">

![h:550](https://i.imgur.com/ksXVqE4.png)

<div/>

---

![bg](./assets/Ambient.png)

# Follow Us

<div class="columns" align="center">

<div>

<img src="./assets/GearTwitter.jpg" width=300 height=300/>

</div>

<div>

<img src="./assets/GearWechat.jpg" width=300 height=300/>

</div>

</div>

### 👉 https://wiki.gear-tech.io/docs/examples/prerequisites
### 👉 https://academy.gear-tech.io/
