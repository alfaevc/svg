# svg graph by wasm example

In this example, we show that how to create a svg graph file using Node.js. The code that produces the 
svg graph given the attributes of the graph, the data points and a svg template is written in rust and 
executed in WebAssembly. The hyperparameters of the graph and the data points are read and loaded in 
JavaScript and runs in Node.js. 

## Set up

```
# Prerequisite
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

# Install rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

# Install nvm
$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
# Follow the on-screen instructions to logout and then log in

# Install node
$ nvm install v10.19.0
$ nvm use v10.19.0

# Install ssvmup toolchain
$ npm install -g ssvmup # Append --unsafe-perm if permission denied

# Install the nodejs addon for SSVM
$ npm install ssvm

$ npm install -g wasm-pack
```

## Create new project

```
$ cargo new --lib svg
$ cd svg
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies.

* The `wasm-bindgen` crate is required for invoking Rust functions from JavaScript. 
* The `serde` and `serde_json` crates allow us to work with JSON strings to represent complex data types. 
* The `nodejs-helper` crate allows the Rust function to access console, file system, database, and network.
* The `tera` crate is used to load and parse the svg template.
