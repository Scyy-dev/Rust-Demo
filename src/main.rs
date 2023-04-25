use rust_demo::ui::*;

fn main() {
    let option = main_menu::main_menu();
    dbg!(option);

    let command = console::read_command("Please enter your first command");
    dbg!(command);
}
