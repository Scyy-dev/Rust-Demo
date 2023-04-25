use rust_demo::ui::*;

fn main() {
    let option = main_menu::main_menu();
    dbg!(option);

    // TODO - interpreting of menu options

    loop {}

    let command = console::read_command("Please enter your first command");
    dbg!(command);
}
