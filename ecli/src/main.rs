extern crate dotenv;

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