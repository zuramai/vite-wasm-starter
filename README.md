<p align="center">
  <img src="https://user-images.githubusercontent.com/45036724/185058831-59263fc8-97e7-4801-8182-c892637731c4.svg" width="8%">
  <img src="https://user-images.githubusercontent.com/45036724/185058837-a63a6957-5458-4038-b63b-b23a5109ece3.svg" width="8%">
  <img src="https://user-images.githubusercontent.com/45036724/185058839-7e3ae58b-a886-4fdf-8aad-e68484695e59.png" width="13%">
</p>

<h1 align="center">Vite + Typescript+ Webassembly</h1>
<p align="center"> A starter project for you to create a blazingly fast web application </p>

## Before getting started

You need to get these prerequisites installed:

* [Rust](https://www.rust-lang.org/learn/get-started)
* [NodeJS](https://nodejs.org)
* [wasm-pack](https://github.com/rustwasm/wasm-pack)
* [rsw-rs](https://github.com/lencx/rsw-rs)

## Installation

#### 1. Clone the repository
```sh
git clone https://github.com/zuramai/vite-wasm-starter
cd vite-wasm-starter
```

#### 2. Install dependencies
```sh
pnpm install # OR npm install
```

#### 3. Run the application

You should open two tabs of terminal. **The order of execution is important and do not close the first window ⚠️**.

In the first tab, run:
```
pnpm rsw watch
```

In the second tab, run:
```sh
pnpm dev
```

## Deploying

Run:
```sh
pnpm run build
```


## Adding a crate

1. Generate rust crate
```sh
pnpm rsw new hello-world
```

This will create a crate in a folder named `hello-world`

2. Edit `rsw.toml` to register the crate
```sh
[[crates]]
name = "hello-world"
```


