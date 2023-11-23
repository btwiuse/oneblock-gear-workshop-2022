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

# Hello World 合约

![bg](./assets/AmbientBlack.png)

<div align="center">

![h:300](./assets/PingPong.png)

<div/>

[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=ping/https://github.com/gear-foundation/dapps)

https://github.com/gear-foundation/dapps/tree/master/contracts/ping


---


![bg](./assets/AmbientBlack.png)

# 准备工作

## 安装 Polkadot.js extension

https://polkadot.js.org/extension

## 生成随机钱包地址

右上角 + => Create new account => 保存助记词 => 设置账户名称 / 密码

## 打开 Gear IDEA 并切换到 Vara Testnet

https://idea.gear-tech.io

---


![bg](./assets/AmbientBlack.png)

# Gear IDEA 部署与交互

<div align="center">

![h:500](./assets/GearIDEA.png)

<div/>

https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara-network.io

---

![bg](./assets/AmbientBlack.png)

# Gear Wiki

Gear Documentation Portal: The place to start developing with Gear


<div align="center">

![h:400](./assets/GearWiki.png)

<div/>

https://wiki.gear-tech.io/docs/getting-started-in-5-minutes
