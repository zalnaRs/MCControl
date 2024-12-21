# MCControl

MCControl is a web application for controlling a Minecraft server. It is written in Rust and has basic HTTP auth.

## Development/Building

### Requirements

- Rust

Running:
```bash
cargo run
```

Compiling:
```bash
cargo build --release
```

## Usage

You need to have a `start.sh` in the working directory of MCControl, which starts the server and uses the standard output. If you don't have one use this [generator by PaperMC](https://docs.papermc.io/misc/tools/start-script-gen).

The password is read from the `PASSWORD` environment variable.
You can start the server with, for example `systemd` with this service (`/etc/systemd/system/mccontrol.service`):
```ini
[Unit]
Description=Minecraft Control Service
After=network.target

[Service]
WorkingDirectory=/home/mcserver/mccontrol
ExecStart=/home/mcserver/mccontrol/MCControl --workdir /home/mcserver/mcserver
User=mcserver
Group=mcserver
Restart=on-failure
Environment=PASSWORD=<a secure password>

[Install]
WantedBy=multi-user.target
```

By default, [Rocket](https://rocket.rs/) doesn't expose itself to the public, so use a reverse proxy. I like [Caddy](https://caddyserver.com/) for it's simplicity, you can use this `Caddyfile`:
```Caddyfile
:80 {
  reverse_proxy localhost:8000
}
```
