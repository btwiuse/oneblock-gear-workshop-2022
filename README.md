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

<div align="center">

![h:550](./assets/ActorModel.png)

<div/>

---

![bg](./assets/AmbientBlack.png)

# Flipper Contract

初始状态: 🌚
flip => 🌝
flip => 🌚
flip => 🌝
flip => 🌚
...

---


![bg](./assets/AmbientBlack.png)

# Init: 状态初始化

---


![bg](./assets/AmbientBlack.png)

# Handle：状态更新

---


![bg](./assets/AmbientBlack.png)

# State：状态查询


---


![bg](./assets/AmbientBlack.png)

# Gear IDEA 部署与交互

<div align="center">

![h:500](./assets/GearIDEA.png)

<div/>

https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara-network.io

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

### 👉 https://wiki.gear-tech.io/docs/examples/prerequisites
