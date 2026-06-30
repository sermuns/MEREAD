use nvim_oxi::api::{self, opts::*, types::*};
use nvim_oxi::{Dictionary, Function, Object, print};

#[nvim_oxi::plugin]
fn meread() -> Dictionary {
    Dictionary::from_iter([("setup", Function::from_fn(setup))])
}

fn setup(_args: Object) {
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let greetings = |args: CommandArgs| {
        let who = args.args.unwrap_or("from Rust".to_owned());
        let bang = if args.bang { "!" } else { "" };
        print!("Hello {}{}", who, bang);
    };

    api::create_user_command("Greetings", greetings, &opts).unwrap();
}
