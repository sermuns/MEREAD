use nvim_oxi::{
    Dictionary, Function, Object,
    api::{self, types::*},
    print,
};

#[nvim_oxi::plugin]
fn meread() -> Dictionary {
    Dictionary::from_iter([("setup", Function::from_fn(setup))])
}

fn setup(_: Object) {
    api::create_user_command("MereadPreview", preview, &Default::default()).unwrap();
}

fn preview(_: CommandArgs) {
    print!("Starting preview on http://localhost:3000");
}
