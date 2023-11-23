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
    border-left: 12px solid cyan;
    padding-left: 1rem;
    color: cyan;
  }
  h2 {
    border-left: 0px solid gray;
    padding-left: 1rem;
    color: white;
  }
  p {
    color: white;
  }
  li {
    color: white;
  }
  ul {
    color: white;
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

![bg](./assets/BackgroundGearOrange.png)

<!-- _color: #FFF -->

# Gear 合约的 Actor 模型

## Init, Handle 和 State 入口函数介绍

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

- **Gear Protocol 与 Vara Network**
- **Actor 模型**
- **Init: 状态初始化**
- **Handle：状态更新**
- **State：状态查询**

</div>
</div>

---

<!-- ![bg](./assets/BackgroundGearWhite.png) -->
![bg](./assets/AmbientBlack.png)

# Gear Protocol 与 Vara Network

## Smart Contract Platform built on Substrate

<div class="columns">

<div align="center">

![](https://i.imgur.com/ixxN8sf.png)

<p>Vara Standalone Network</p>

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

</div>

</div>

---

# Actor 模型

![bg](./assets/AmbientBlack.png)

一切皆 Actor

每个 Actor 封装了自己的状态和行为

通过消息传递进行通信，而不是共享内存

在 Actor 处理收到的消息时，它可以:

- 向另一个 Actor 发送消息
- 创建新的 Actor
- 改变其内部状态

> Note: Gear Protocol 在传统的 Actor 模型上额外保证了合约(Program)间消息的顺序

---

![bg](./assets/AmbientBlack.png)

<div align="center">

![h:550](./assets/ActorModel.png)

<div/>

---

![bg](./assets/AmbientBlack.png)

# Flipper Contract

初始状态: 🌚

```
static mut FLIPPER: bool = false;
```

flip => 🌝
flip => 🌚
flip => 🌝
flip => 🌚
...

---

![bg](./assets/AmbientBlack.png)

# Metadata 与消息定义

```
impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<FlipperAction, FlipperEvent>;
    type State = InOut<FlipperQuery, bool>;
    ...
}

pub enum FlipperAction {
    Flip,
}

pub enum FlipperEvent {
    FlippedTo(bool),
}

pub enum FlipperQuery {
    State,
}
```

---

![bg](./assets/AmbientBlack.png)

# Init: 状态初始化

```
#[no_mangle]
unsafe extern fn init() {
    FLIPPER = false
}
```

---


![bg](./assets/AmbientBlack.png)

# Handle：状态更新

```
#[no_mangle]
unsafe extern fn handle() {
    let action: FlipperAction = gstd::msg::load().expect("failed to load input message");
    match action {
        FlipperAction::Flip => {
            FLIPPER = !FLIPPER;
        },
    }
    gstd::msg::reply(FlipperEvent::FlippedTo(FLIPPER), 0).expect("failed to send response");
}
```

---


![bg](./assets/AmbientBlack.png)

# State：状态查询

```
unsafe extern fn state() {
    let query: FlipperQuery = gstd::msg::load().expect("failed to load input message");
    match query {
        FlipperQuery::State => {
            gstd::msg::reply(FLIPPER, 0).expect("failed to send response");
        },
    }
}
```

---


![bg](./assets/AmbientBlack.png)

# Gear IDEA 部署与交互

<div align="center">

![h:500](./assets/GearIDEA.png)

<div/>

https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara-network.io

[0xd02c3bae46ab3a9e4a037eb3f5e452f75d6a3a00f79bd3710fb5d4ee9dceb494](https://idea.gear-tech.io/programs/0xd02c3bae46ab3a9e4a037eb3f5e452f75d6a3a00f79bd3710fb5d4ee9dceb494?node=wss%3A%2F%2Ftestnet.vara-network.io)

---

![bg](./assets/AmbientBlack.png)

# Follow Us

<div class="columns">

<div>

<img src="./assets/GearTwitter.jpg" width=300 height=300/>

</div>

<div>

<img src="./assets/GearWechat.jpg" width=300 height=300/>

</div>

</div>

### 👉 https://wiki.gear-tech.io/docs/developing-contracts/state/
