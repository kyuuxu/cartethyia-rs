# Wicked Waifus

![Screenshot](screenshot.png)

## About
**Wicked Waifus is an open-source Wuthering Waves server emulator written in Rust**. 
The goal of this project is to ensure a clean, easy-to-understand code environment. 
Wicked Waifus uses **tokio** for asynchronous networking operations, **axum** as http framework and **ZeroMQ** for communication between servers.
It also implements **performant and extensible ECS** for emulation of the game world.

## Getting started
#### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Protoc](https://github.com/protocolbuffers/protobuf/releases) (for protobuf codegen)

#### Setup
##### a) building from sources

```sh
git clone --recursive https://git.xeondev.com/wickedwaifus/wicked-waifus-rs.git
cd wicked-waifus-rs
cargo run --bin wicked-waifus-config-server
cargo run --bin wicked-waifus-hotpatch-server
cargo run --bin wicked-waifus-login-server
cargo run --bin wicked-waifus-gateway-server
cargo run --bin wicked-waifus-game-server
```

##### b) building from sources(docker edition)
If you are to wheelchair'd for option A, you can fallback to option b.
In this case you will need [Docker Desktop](https://www.docker.com/products/docker-desktop/)

Once installed, to build the images, run:
```sh
# or builder.bat if you run it on windows
./builder.sh
```

And to run the containers:
```sh
docker compose up -d
```

##### c) using pre-built binaries
Navigate to the [Releases](https://git.xeondev.com/wickedwaifus/wicked-waifus-rs/releases)
page and download the latest release for your platform.<br>
Launch all servers: `wicked-waifus-config-server`, `wicked-waifus-hotpatch-server`, `wicked-waifus-login-server`, `wicked-waifus-gateway-server`, `wicked-waifus-game-server`

##### NOTE: you don't have to install Rust and Protoc if you're going to use pre-built binaries, although the preferred way is building from sources.<br>We don't provide any support for pre-built binaries.

#### Configuration
You should configure each server using their own config files. They're being created in current working directory upon first startup.

##### Database section
You have to specify credentials for **PostgreSQL**<br>
###### An example of database configuration:
```
[database]
host = "localhost:5432"
user_name = "postgres"
password = ""
db_name = "wicked_waifus_db"
```
##### NOTE: don't forget to create database with specified `db_name` (default: `wicked_waifus_db`). For example, you can do so with PgAdmin.

#### Data
The data files: Logic JSON collections (`data/assets/game-data/BinData`) and config/hotpatch indexes (`data/assets/config-server`, `data/assets/hotpatch-server`) are included in this repository. Keep in mind that you need to have the `data` subdirectory in current working directory.

#### Connecting
You have to download client of Wuthering Waves Beta 2.1, apply the [wicked-waifus-win-patch](https://git.xeondev.com/wickedwaifus/wicked-waifus-win-patch/releases) and add necessary `.pak` files, which you can get here: [wicked-waifus-pak](https://git.xeondev.com/wickedwaifus/wicked-waifus-pak)

### Troubleshooting
[Visit our discord](https://discord.gg/reversedrooms) if you have any questions/issues

### Support
If you want to support this project, feel free to [send a tip via boosty](https://boosty.to/xeondev/donate)