# Description

This repo is for educational purposes only.
it contains homework project for rust course, provided by otus.ru

A UI for SmartSocket.
SmartSocket itself runs on remote server(127.0.0.1:8282)
Commands from frontend (Tauri + Vue 3) are sent to backend (tcp_smart_socket::SmartSocketServer), executed on server, that drives smart_socket,
and results are sent back to frontend and shown.

## Usage

$ yarn tauri dev

## To start server: 
### Clone get repo:
$ git clone https://github.com/inspired2/stp.git

### switch to async branch:
$ git checkout async

### run server:
$ cargo run --example async-server

### or run with logging: 
$ RUST_LOG=debug cargo run --example async_server
