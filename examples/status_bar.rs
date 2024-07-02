use rustibia::client::status_bar::StatusBar;
use rustibia::utils::image::load_image;

fn main() {
    let content = load_image("./assets/examples/client/status_bar/100_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("100%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/90_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("90%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/80_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("80%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/70_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("70%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/60_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("60%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/50_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("50%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/40_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("40%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/30_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("30%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/20_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("20%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/10_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("10%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");

    let content = load_image("./assets/examples/client/status_bar/1_percent.png").unwrap();
    let status_bar = StatusBar { content };
    println!("1%");
    println!("hp: {:?}", status_bar.get_hp_percentage());
    println!("mana: {:?}", status_bar.get_mana_percentage());
    println!("");
}
