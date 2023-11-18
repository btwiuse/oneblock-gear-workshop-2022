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

# 如何开始在 Vara 上构建 DApp

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

- **Gear & Vara Network**
- **Actor 模型简介**
- **Hello World 合约样例**
- **Gear IDEA 部署与交互**
- **Gear JS 前端库**
- **Gear Wiki 与进阶课程**

</div>
</div>

---

<!-- About the company -->

![bg](https://i.imgur.com/Tp9YqQ6.png)

---

<!-- Nikolay Volf -->

![bg](https://i.imgur.com/H20r5pN.jpg)

---

<!-- CBDO / CFO / CTO -->

![bg](https://i.imgur.com/xo3YfJU.png)

---

<!-- Istanbul && California -->

![bg](https://i.imgur.com/abbsaIe.jpg)

---

<!-- California && Mexico -->

![bg](https://i.imgur.com/0Tntbgr.jpg)

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

![bg](./assets/Ambient.png)

# Workshop 大纲

| 模块 | 描述 | 目标 |
| :---: | :---: | :---: |
| [./app](./app) | 模板合约 | 搭建开发环境 / 了解 Gear 合约项目的结构, 入口函数 / 使用 Gear IDEA 上部署合约 / 向合约发送消息 / 查询合约状态 |
| [./tamagotchi](./tamagotchi) | 宠物 NFT 合约 | 了解合约代码逻辑 / 创建宠物 / 自定义宠物属性 |
| [./battle](./battle) | 对战合约 | 了解对战游戏合约运行逻辑 |
| [./frontend](./frontend) | 游戏前端&合约交互 | 在本地运行游戏前端 / 使用 gear-js 脚本读取合约状态 / 体验游戏 [![Play Button](https://img.icons8.com/material-rounded/24/000000/play--v1.png)](https://tamagotchi-battle.vercel.app) |

---

![bg](./assets/Ambient.png)

# 准备工作

## 安装 Polkadot.js extension

https://polkadot.js.org/extension

## 生成随机钱包地址

右上角 + => Create new account => 保存助记词 => 设置账户名称 / 密码

## 打开 GitPod 在线编辑器 (使用 GitHub 登录)

https://github.com/GearFans/tamagotchi-battle
