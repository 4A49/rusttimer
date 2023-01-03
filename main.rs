use owo_colors::{DynColors, OwoColorize};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};


//baseless constant.(You can display another art or leave blank)
const OWO: &str = r#"."#;
const COLORS: &str = r#"  
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠤⠤⠤⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢀⠔⠉⢁⡨⠛⠉⠀⠀⠀⠀⠀⠀⠀⠉⠒⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢠⡟⠊⡱⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠒⠤⠤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢰⠃⢣⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠁⠲⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢧⡀⡞⠓⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠢⣄⠀⣀⠤⠶⠚⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢹⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠔⠚⢻⠀⢀⠔⠉⠉⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡄⠀⠀⠀⣠⡾⠅⠀⠀⠀⢘⡶⠋⠀⠀⠀⢸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡀⢀⠞⠁⠀⠀⠀⠀⠀⡜⠀⠀⠀⣀⠴⠊⠉⠉⠁⠒⠤⢄⡀⠀⠀⠀⠀
⠀⠀⠀⠱⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⣏⠀⠀⠀⠀⠀⡠⠞⠀⢀⡴⠊⠁⠀⠀⠀⠀⠀⠀⠀⢀⠙⣦⠀⠀⠀
⠀⠀⠀⠀⠈⢦⡀⠀⢶⣄⡀⠀⠀⠀⠀⠀⠉⠐⠒⠂⠉⠀⢀⡔⠋⠀⠀⠀⠀⠀⠀⣀⠤⠒⠊⣹⣧⡟⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢨⣦⡀⠳⡉⠲⢄⡀⠀⠀⠀⠀⠀⠀⠀⡰⠋⠀⠀⠀⠀⠀⣠⠔⠋⢀⣰⢺⣿⣻⠏⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠩⡉⠳⢿⣄⣀⡀⠀⠀⠀⠀⠀⠀⡼⠁⠀⠀⠀⠀⡠⠊⢁⣼⢟⣩⠞⡿⠊⠁⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠙⠂⢰⠋⣠⡤⠀⠀⠀⠀⠀⡼⠁⠀⠀⠀⣠⠎⠀⣾⡟⢡⠞⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢦⣉⣃⣀⡾⠀⠀⢰⠁⠀⠀⠀⡴⢡⣴⣾⣿⣿⢋⡞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⡇⠀⠀⠀⡞⠀⠀⠀⣼⣥⣾⣿⣿⣿⣯⢸⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⣇⠀⠀⢸⣿⣿⣿⣿⡟⠿⣿⠆⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠸⡄⠀⢸⣷⣿⣾⣧⢳⣶⣿⡇⢸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡇⠀⠀⠀⠀⠙⢄⠀⠉⢻⠀⠹⣿⣿⠿⡷⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠛⢧⣀⠀⠀⠀⠀⠀⠙⠒⠼⣄⣀⠀⠀⠀⣷⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠈⠉⠒⠦⠤⣀⡀⠀⠀⠀⠈⠙⣆⠘⢹⡾⠉⢢⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⢹⢦⡀⠀⠀⠀⡼⢧⠀⠈⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣀⠤⠄⠒⠒⠒⠛⠿⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠑⣄⠀⠀⡇⠈⢆⠀⣹⠤⣀⠀⠀⠀⠀⠀
⠀⠀⢀⠔⠋⠀⠀⠀⠀⠀⠀⠀⠀⠈⠓⡌⠑⠢⢄⣀⠀⠀⢠⠁⠀⠀⠈⢣⣠⠃⠀⠈⢶⣧⠀⠀⠉⠒⢄⠀⠀
⠀⡰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡀⠀⠀⠀⠉⠑⠋⠀⠀⠀⠀⠀⠹⠀⠀⠀⠀⠈⢣⠀⠀⠀⠀⠙⠆
⠰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠧⠀⠀⠀⠀⠀  "#;
fn main() {
    //print the Homer Ascii Art
    let colors: [DynColors; 6] = [
        "#B80A41", "#4E4BA8", "#6EB122", "#DAAC06", "#00938A", "#E23838",
    ]
    .map(|color| color.parse().unwrap());

    println!("\n\n\n\n\n{}", OWO.fg_rgb::<0x2E, 0x31, 0x92>().bold());

    for line in COLORS.split_inclusive('\n') {
        for (text, color) in line.split('|').zip(colors.iter().copied()) {
            print!("{}", text.color(color).bold());
        }
    }

    println!("\n\n\n\n\n\n");

    println!("Enter the number of seconds for the timer:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let seconds: u64 = input.trim().parse().unwrap();
    let duration = Duration::from_secs(seconds);
    let half_duration = duration / 2;
    let almost_done_duration = (duration.as_secs_f64() * 0.95) as u64;

    println!("Timer started for {} seconds.", seconds);

    thread::sleep(half_duration);
    println!("\x07");
    println!("You are should be Halfway Your task!!!");

    thread::sleep(half_duration);

    thread::sleep(Duration::from_secs(almost_done_duration) - half_duration);
    println!("\x07");
    println!("You should be winding up");

    thread::sleep(duration - Duration::from_secs(almost_done_duration));

    println!("\x07");
    println!("Your time is over!!");
}
