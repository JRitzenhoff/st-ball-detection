# ST-Ball-Detection

Code that runs on an STML4S5I development board.


## Debugging Programming to the board

See the [debugging](./debugging.md) file for some information about how to setup the board for programming

## Attempted Docker

I tried to get embassy working inside of a Devcontainer.

Unfortunately, because MacOS does not yet support passing through USB connections from host to Devcontainer, this failed

It is necessary to install rust and probe-rs to the host machine
* Rust - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Probe-Rs - `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh`