# USB in MacOS

https://docs.docker.com/desktop/features/usbip/

* Need to run a USB/IP server on the host
* Need to connect to that server from the Docker container/Devcontainer

## USB in MacOS blog

https://blog.golioth.io/usb-docker-windows-macos/

1. Need to download rust on the host
    * `https://www.rust-lang.org/tools/install`
1. Need to download the USB/IP server
    1. `git clone https://github.com/jiegec/usbip/tree/master`
    1. `cargo run --example host`
1. Need to spin up the device-manager Dockerfile
    * It is spun up with privileged access, opens a process with the host's ID, and creates a Docker network
    * `docker run --rm -it --privileged --pid=host jonathanberi/devmgr`
    * NOTE: Also see the [docker-compose](./docker-compose/) Dockerfile for post-launch steps
