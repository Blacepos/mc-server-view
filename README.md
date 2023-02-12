# MC Server View

A basic Minecraft server wrapper

# Installation
I will assume you know how to run a Minecraft server (and have the appropriate Java runtime installed)

## Build the application with the Rust toolchain
1. https://www.rust-lang.org/tools/install
    - The default toolchain should suffice
    - Verify installation with `cargo --version`
2. Run `cargo build --release`
    - This may take some time
3. Built executable stored in target/release

## Configure the environment
1. Open port 25565 for Minecraft (editable in .env with MINECRAFT_PORT)
2. Open port 80
    - Operating systems seem to not like servers binding to port 80 so the webserver is bound to 3000 by default (editable in .env with WEBSERVER_PORT) (You do not need to forward 3000).
    - Forward port 80 to 3000 (look into the `iptables` command for Linux).
3. Find a place to store the servers. Edit SERVER_PATH in .env to point to a server folder.
    - Suggestion: create a folder to store the Minecraft servers. Inside that, create a unique fold.er for each server.
4. Create a shell script (e.g. run.bat or run.sh) and add the java jar command to run the server.
    - Set RUN_COMMAND to be the name of the script.
5. Enable RCON (so the webserver can communicate with Minecraft) in server.properties.
    - Set enable-rcon=true
	- Set rcon.password=\<some password\>
	- Set rcon.port=\<some unused port\>
	- Set RCON_PASSWORD and RCON_PORT in .env to the same values
	- Set enable-query=true
	- Set query.port=\<some unused port\>
	- Set QUERY_PORT in .env to the same values
	- Suggestion: Do NOT forward the RCON or query ports for security reasons unless you have some other application that needs them.