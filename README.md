# ns_daemon
NetSpeed Daemon for NetSpeed Overlay app written in Rust that provides 
- the upload, download bytes used of present session 
- total bandwidth usage since the daemon started running. 

Upload and Download total bytes updates every 1s, the total is updated every 60s.

### Crates used
- `serde_json`
- `abserde`
- `serde`
- `chrono`
- `sysinfo `
- `byte-unit `

## Tests
✔️ Manjaro XFCE (Linux, based on Arch)  
Tested on Linux but uses libs that should function on Windows and Mac OS platform. Feel free to test in any other platform and tell me the results! 
  
## Setup
- If you use the executable run the executable with the interface name from any shell if you wish to monitor a specific interface else just double click on the executable. 

The data will be available at `http://localhost:6798` as a json file with session upload bytes, session download bytes, today data usage as the three fields in that order.


- If you wish to run the daemon directly from the source code
Steps to install rust is available at bottom of this page [here](https://github.com/visnkmr/ns_daemon/edit/main/README.md#installing-rust). 

Clone the project and in the same directory open preffered shell and `cargo run <interface-name>(optional)`. You can find the debug executable in `target/debug` folder after running `cargo run`.
  
## Reporting issues

Found a bug? We'd love to know about it!

Please report all issues on the GitHub [issue tracker][issues].

[issues]: https://github.com/visnkmr/ns_daemon/issues

## Installing Rust
Full docs [here](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/getting-started.html#installing-rust)

All you need to do on Unix systems like Linux and macOS is open a terminal and type this:

`$ curl https://sh.rustup.rs -sSf | sh`

It will download a script, and start the installation. If everything goes well, you’ll see this appear:

`Rust is installed now. Great! `

Installing on Windows is nearly as easy: download and run [rustup-init.exe](https://win.rustup.rs/). It will start the installation in a console and present the above message on success.

For other installation options and information, visit the [install page of the Rust website](https://www.rust-lang.org/install.html).
Cargo comes installed with Rust itself, if you used the official installers.
