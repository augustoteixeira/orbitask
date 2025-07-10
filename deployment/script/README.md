# Troubleshoot running the binary

    sudo add-apt-repository -y ppa:ubuntu-toolchain-r/test
    sudo apt install -y g++-11

If missing glibc, https://github.com/modular/modular/issues/3684

# First installation

On the server:

    cd ~
    git clone https://github.com/augustoteixeira/orbitask.git
    sudo mv orbitask /opt

Compile either locally or on the server and add the file in `orbitask/backend/target/release/backend` to `/opt/orbitask/`.

    cd /opt/orbitask
    ./backend

Supervising the process

Supervisor:

    sudo apt update && sudo apt install supervisor

Edit `/etc/supervisor/conf.d/orbitask.conf` with:

    [program:orbitask]
    directory=/opt/orbitask/backend
    command=/opt/orbitask/backend/backend
    autostart=true
    autorestart=true
    stderr_logfile=/var/log/orbitask.err.log
    stdout_logfile=/var/log/orbitask.out.log

Then:

    sudo apt install supervisor
    sudo supervisorctl update

# Upgrading

Perhaps you want to add `youruser ALL=(ALL) NOPASSWD: /usr/bin/supervisorctl restart orbitask` to your sudo commands with `sudo visudo`.

Copy the `update_default` script to `update` (this name is ignored by git).

Rename the environment variables.
