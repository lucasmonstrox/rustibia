use rustibia::client::status_bar::StatusBar;
use rustibia::utils::image::load_image;

#[test]
fn return_100_when_hp_percentage_is_100() {
    let content = load_image("./assets/examples/client/status_bar/100_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 100);
}

#[test]
fn return_90_when_hp_percentage_is_90() {
    let content = load_image("./assets/examples/client/status_bar/90_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 90);
}

#[test]
fn return_80_when_hp_percentage_is_80() {
    let content = load_image("./assets/examples/client/status_bar/80_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 80);
}

#[test]
fn return_70_when_hp_percentage_is_70() {
    let content = load_image("./assets/examples/client/status_bar/70_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 70);
}

#[test]
fn return_60_when_hp_percentage_is_60() {
    let content = load_image("./assets/examples/client/status_bar/60_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 60);
}

#[test]
fn return_50_when_hp_percentage_is_50() {
    let content = load_image("./assets/examples/client/status_bar/50_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 50);
}

#[test]
fn return_40_when_hp_percentage_is_40() {
    let content = load_image("./assets/examples/client/status_bar/40_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 40);
}

#[test]
fn return_30_when_hp_percentage_is_30() {
    let content = load_image("./assets/examples/client/status_bar/30_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 30);
}

#[test]
fn return_20_when_hp_percentage_is_20() {
    let content = load_image("./assets/examples/client/status_bar/20_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 20);
}

#[test]
fn return_10_when_hp_percentage_is_10() {
    let content = load_image("./assets/examples/client/status_bar/10_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 10);
}

#[test]
fn return_1_when_hp_percentage_is_1() {
    let content = load_image("./assets/examples/client/status_bar/1_percent.png").unwrap();
    let status_bar = StatusBar { content };
    assert_eq!(status_bar.get_hp_percentage(), 1);
}
