# svg graph by wasm example

In this example, we show that how to output a svg graph file in the web app using Node.js. The code that produces the 
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

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains Rust functions to read the attributes and hyperparameters of the graph and the json array of x coordinates and the json array of y coordinates from the Node.js, and return a string of a xml file of the svg graph where the adjacent data points are connected by a linear line. The string is written to a new svg file given the svg template [src/graph.svg].

## Build the WASM bytecode

```
$ ssvmup build
```

## Create a node app

### setup express
```
$ npm install -g express-generator
$ express nodeweb
$ npm install
```

### (Optional) change Jade to handlebars
```
$ npm uninstall jade --save
$ npm install express-handlebars --save
```
Adjust implementations in app.js after installing handlebars and layouts.hbs

### Setup express-validator and express-session

```
$ npm install --save express-validator
$ npm uninstall express-validator
$ npm install express-validator@5.3.1
$ npm install --save express-session
```
Again adjust implementations in app.js.

### Make the web app

The [nodeweb/routes/index.js](nodeweb/routes/index.js) app shows how to call the Rust functions from JavaScript, and together with the hbs files in [nodeweb/views], it's meant to take 3 inputs on the web server, which are x coordinates, y coordinates and title of the graph respectively by clicking the submit button, it's expected to redirect to a page that demonstrates the resultant svg graph.


## Test

Go to node directory and compile
```
$ npm start
```


