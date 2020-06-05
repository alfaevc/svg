# svg graph by wasm example

In this example, we show that how to create a svg graph file using Node.js. The code that produces the 
svg graph given the attributes of the graph, the data points and a svg template is written in rust and 
executed in WebAssembly. The hyperparameters of the graph and the data points are read and loaded in 
JavaScript and runs in Node.js. 

## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
$ [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
$ [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

$ nvm install v10.19.0
$ nvm use v10.19.0

$ npm install -g wasm-pack
```
