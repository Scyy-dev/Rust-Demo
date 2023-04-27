use rust_demo::{
    entity::{player::Player, Entity},
    inventory::Inventory,
    stat::StatSet,
    ui::main_menu,
};

fn main() {
    let option = main_menu::main_menu();
    // TODO - interpreting of menu options
    println!("You chose option: {}", option);
    let mut player = Player::new();
    for _ in 0..5 {
        player.damage(5);
        println!("Player now has {} health", player.get_health())
    }

    let stat_set1 = StatSet::try_from("v:0/a:5/d:0");
    let inventory1 = Inventory::try_from("v:5/a:0/d:0|a\nv:0/a:5/d:0|b\nv:0/a:0/d:5|ab");
    let player = Player::create(stat_set1.unwrap(), 100, inventory1.unwrap());
    let actions = player.get_action_set();
    dbg!(actions);
}
