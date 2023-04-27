use rust_demo::{
    entity::{player::Player, Entity},
    inventory::Inventory,
    stat::StatSet,
    ui::{console, main_menu},
};

fn main() {
    let option = main_menu::main_menu();
    dbg!(option);

    // TODO - interpreting of menu options

    let command = console::read_command("Please enter your first command");
    dbg!(command);

    let stat_set1 = StatSet::try_from("v:0/a:5/d:0");
    let inventory1 = Inventory::try_from("v:5/a:0/d:0|a\nv:0/a:5/d:0|b\nv:0/a:0/d:5|ab");
    let player = Player::create(stat_set1.unwrap(), 100, inventory1.unwrap());
    let actions = player.get_action_set();
    dbg!(actions);
}
