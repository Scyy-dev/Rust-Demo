use rust_demo::{stat::StatSet, ui::*};

fn main() {
    let option = main_menu::main_menu();
    dbg!(option);

    // TODO - interpreting of menu options

    let command = console::read_command("Please enter your first command");
    dbg!(command);

    let stat_set1 = StatSet::from_string("v:0/a:5/d:0");
    dbg!(stat_set1);

    //let item1 =

    //let
}
