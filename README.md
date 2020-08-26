# Amnesia-Client

**WARNING: The author tested this program several times on both VMs
and his dev machine before pushing it to Github. But use it at your own
risk. The author can't be held liable if this program caused any kind of damage**


This is a malicious program that when activated, can capture network
traffic and send the capture to a command and control server (checkout
[amnesia](https://github.com/realaravinth)).


## Installation

#### - Debian based distributions:

Pre-compiled packages are available at the release section of this. You
may download and install them with:

`$ sudo apt instlal -f ./<name-of-the-package>`

Don't forget to substitute the package name at `<name-of-the-package>`

If prompted to choose weather non-root users should be allowed to run
wireshark/tshark, then please allow it.
#### - From source:

1. See [dependencies](#dependencies) and install them
2. After installing `cargo`, run:
`$ git clone https://github.com/realaravinth/amnesia-client`
3. `$ cd amnesia-client && cargo build --release`

You'll find the release build at `target/release/`

## Dependencies:

1. [Cargo](https://github.com/rust-lang/cargo/)
2. [Tshark](https://tshark.dev/setup/install/):
	You'll find it in your distribution's repository, so you can get it
	from there.
If prompted to choose weather non-root users should be allowed to run
wireshark/tshark, then please allow it.

## Setup and usage:

The name of the internet facing network interface should be based as a
parameter to the program. Running the following command will list all
network interfaces on your machine:

`$ sudo ifconfig`

Ethernet devices will begin with an 'e' and wireless interfaces will
begin with an 'w'. Assuming your's is `wlan0`, run the following command
to start the program:

`$ amnesia-client wlan0`

By default, the program binds itself to port 8000. 
So the web interface can be accessed at

`<your-IP-address>:8000`

To find your IP address, you may run:

`$ sudo ifconfig`

your IP is the value following `INET` against your network interface.
See highlighted value in the img

![ifconfig output](/img/ip.png)



