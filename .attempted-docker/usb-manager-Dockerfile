FROM alpine

# Stolen shamelessly from: https://github.com/beriberikix/docker-devmgr/tree/main

# Needs to be launched with: `docker run --rm -it --privileged --pid=host <build-name>`

RUN \
  apk upgrade --no-cache \
  && apk add --no-cache usbutils

CMD [ "nsenter", "-t", "1", "-m" ]

# Display all of the USB devices
CMD "echo 'usbip list -r host.docker.internal' >> .bash-history"

# Assign the appropriate USB device to the USB/IP server
CMD "echo 'usbip attach -r host.docker.internal -d 20-2-3' >> .bash-history"

# Confirm that the USB device has been added to the server
CMD "echo 'usbip port' >> .bash-history"

# Confirm that the device actually shows up
CMD "echo 'lsusb' >> .bash-history"