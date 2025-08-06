# Use official Rust image
FROM rust:1-trixie

# Set build-time arguments for user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

# Install packages, install bash and sudo
RUN apt-get update && apt-get install -y \
    bash \
    sudo \
    curl \
    unzip \
    libusb-1.0-0-dev \
    # Required for lsusb
    usbutils \
    && rm -rf /var/lib/apt/lists/*

# Create the VSCode user (and add to the sudo group)
RUN groupadd --gid $USER_GID ${USERNAME} \
    && useradd --uid $USER_UID --gid $USER_GID -m ${USERNAME} -s /bin/bash \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME} \
    && chsh -s /bin/bash ${USERNAME}

# Install Starship (for a prettier terminal)
RUN curl -sS https://starship.rs/install.sh | sh -s -- -y

# Configure Starship for bash
RUN echo 'eval "$(starship init bash)"' >> /home/${USERNAME}/.bashrc

# Copy Starship config
COPY starship.toml /home/${USERNAME}/.config/starship.toml
RUN chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}/.config

USER ${USERNAME}

# Install probe-rs as a system-wide
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
