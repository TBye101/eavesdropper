/*
Modules to make:

WhoIs
* Performs WhoIs lookups on every IP

Protocol Detector
* Determines what protocol each packet is using

Virus/Malware detector
* Detects virus or malware via their network activity

Encrypted Connection cracking
* Brute forcing for breaking encrypted connections

unencrypted DNS lookup analyzer

Handshake interceptor
* Intercept handshakes to decrypt packets

Program analyzer
* Guess/Determine what program sent the packets

*/

/*
    Lints:
    https://crates.io/crates/clippy


    Hardcore security:
    https://crates.io/crates/scrypt



    https://crates.io/crates/cargo-inspect

    https://crates.io/crates/indicatif


    Testing:
    https://crates.io/crates/static_assertions
    https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
    https://crates.io/crates/assert_cmd
    https://github.com/AltSysrq/proptest
    https://github.com/project-oak/rust-verification-tools

    db:
    https://github.com/spacejam/sled

    performance:
    https://github.com/async-rs/async-std
    https://github.com/tokio-rs/tokio

    Analysis:
    https://crates.io/crates/pcap
    https://crates.io/crates/etherparse
    https://ip-api.com/

    https://crates.io/crates/clap or preferably https://crates.io/crates/pico-args

*/

extern crate etherparse;

mod listen_command;
mod analyze_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let listen_command: Box<dyn cliargs_t::Command> = Box::new(listen_command::ListenCommand {});
    let analyze_command: Box<dyn cliargs_t::Command> = Box::new(analyze_command::AnalyzeCommand {});
    let mut commands = vec![
        listen_command,
        analyze_command
    ];
    let commander = cliargs_t::Commander::new(&mut commands);

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                commander.handle_input(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

    // let mut devices = Device::list().unwrap();
    // println!("{:?}", devices);
    // let mut selected = Device::lookup().unwrap();
    // println!("Selected: {:?}", selected);
    // let mut cap = Device::lookup().unwrap().open().unwrap();
    // let mut savefile = cap.savefile("test.pcap").unwrap();
    // let mut count = 0;
    // while let Ok(packet) = cap.next() {
    //     parsePacket(&packet);
    //     count += 1;
    //     println!("Number of packets captured: {}", count);
    //     savefile.write(&packet);
    // }

// fn parsePacket(packet: &pcap::Packet) {
//     match PacketHeaders::from_ethernet_slice(&packet) {
//         Err(value) => println!("Err {:?}", value),
//         Ok(value) => {
//             println!("link: {:?}", value.link);
//             println!("vlan: {:?}", value.vlan);
//             println!("ip: {:?}", value.ip);
//             println!("transport: {:?}", value.transport);
//         }
//     }
// }