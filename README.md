# MC Server View

A web interface for managing a Minecraft server

# Installation
I will assume you know how to run a Minecraft server (and have the appropriate Java runtime installed).

## Overview
Using this application at this period of development is currently a little more involved than the planned release:
- No binaries are available, so you must build from source.
- Minecraft servers are downloaded, organized, and configured manually.

## Build the application with the Rust toolchain
1. https://www.rust-lang.org/tools/install
    - The default toolchain should suffice
    - Verify installation with `cargo --version`
2. Run `cargo build --release`
    - This may take some time
3. To deploy the server run, `cargo run --release`
    - This should run immediately if the server is built
    - The built executable is stored in target/release if you wish to run that directly instead (you may have to copy .env)

## Configure the environment
Note before you begin: you only need to forward 2 ports (80 and 25565) even though the application uses a few others.

1. Forward port 25565 for Minecraft (editable in .env with MINECRAFT_PORT)
2. Forward port 80 for the webserver
    - Operating systems seem to not like servers binding to port 80 so the webserver is bound to 3000 by default (editable in .env with WEBSERVER_PORT).
    - Redirect port 80 to 3000 (look into the `iptables` command for Linux).
3. Find a place to store the servers. Edit SERVER_PATH in .env to point to a server folder.
    - Suggestion: create a folder to store the Minecraft servers. Inside that, create a unique folder for each server.
4. Create a shell script (e.g. run.bat or run.sh) and add the java jar command to run the server.
    - Make sure to add a shebang (like #!/bin/bash) to the top of the script on Linux
    - Set RUN_COMMAND to be the name of the script.
5. Set the following values in server.properties:
    - enable-rcon=true
	- rcon.password=\<some password\>
	- rcon.port=\<some unused port\>
6. Set corresponding values in .env to be the same as in server.properties
