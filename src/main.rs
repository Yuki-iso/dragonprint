use std::{
    env, 
    fs::OpenOptions,
    io::{prelude::*, BufReader}
};
use rand::seq::SliceRandom;


fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        sayy(args[1].to_string())
    }
    else {
        sayy(generate_string());
    }

}

//fn print_type_of<T>(_: &T) { //new to Rust (as you can see), this block identifies type of object
//    println!("{}", std::any::type_name::<T>())
//}

fn sayy(inp: String) {
    log::debug!("Displaying line: '{inp}'");
    let padd = if let Some((w, _h)) = term_size::dimensions() {
        (w - 73) / 2 //w is width, 73 is width of dragon o7
    } else {
        // Handle case where terminal dimensions are not available
        log::debug!("Could not retrieve terminal dimensions.");
        0 // Fallback padding value
    };
    let padding = " ".repeat(padd);
    let dragontop = r"
                                  /   \
                          )      ((   ))     (
(@)                      /|\      ))_((     /|\
|-|\.                   / | \    (/\|/\)   / | \                      (@)
| |--------------------/--|-voV---\`|'/--Vov-|--\---------------------|-|
|-|                         '^`   (o o)  '^`                          | |
| |                               `\Y/'                               |-|
| |                                                                   | |";
    let dragonbot = 
r"
| |                                                                   |-|
|_|___________________________________________________________________| |
(@)                 I   /\ /      ( (       \ /\   I                `\|-|
                    I /   V        \ \       V   \ I                  (@)
                    I/              | |           \I 
                                   _) )_ 
                                   `\ /'
";
    let signpre = r"| |     ";
    //
    // println!("{}, {}", generateString(), &generateString()[1..8]);
    let mut text;// = Vec::<String>::new();
    if !inp.is_empty() {
        text = format_string(inp);
    } else {
        text = format_string(generate_string().to_string());
    }

    if text.len() == 1 {
        let v = text[0].len() as f32 / 2.0;
        text[0] = r" ".repeat(28 - v.floor() as usize) + &text[0][..];
        // signpre = &finalstr[..];
        // println!("{}", finalstr)
    }

    
    //println!("{}", dragontop);
    let dragontop: Vec<String> = dragontop.lines().skip(1)
        .map(String::from)
        .collect();
    for line in dragontop.iter() {
        println!("{padding}{line}", );
    }

    for item in text {
        let signpost = " ".repeat(62 - item.len());
        println!("{padding}{}{}{}| |",signpre, item, signpost);
    }
    
    //println!("{}", dragonbot);
    let dragonbot: Vec<String> = dragonbot.lines().skip(1)
        .map(String::from)
        .collect();

    for line in dragonbot.iter() {
        println!("{padding}{line}", );
    }
}

fn generate_string() -> String {
    let config = dirs::config_dir().expect("READON").into_os_string().into_string().unwrap() + "/dragonprint/lines";
    //below this is all stupid
    //try to read the lines file here
    //if succeed, then pick a random line from it and return
    //if fail, create the file with 'add lines to ~/.config/dragonprint/lines' in it
    //re-run this function
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(config).expect("fuck");

    let buf = BufReader::new(&file);
    let options: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // let mut rng = rand::thread_rng();
    let pick = match options.choose(&mut rand::thread_rng()) {
        Some(entry) => entry.to_string(),
        None => {let _ = file.write_all(b"Add dialogue to `~/.config/dragonprint/lines"); generate_string()},
    };
    pick
}

fn format_string(mut input: String) -> Vec<String> {
    input = " ".to_owned() + &input;
    let mut vecc: Vec<String> = vec![];
    while input.len() > 60 {
        let mut charr = 60;
        // println!("{} {}", input, charr - 1);
        while input[charr..].chars().next().unwrap() != ' ' {
            charr = charr - 1;
        }
        vecc.push(input[1..charr].to_string());
        input = input[charr..input.len()].to_string();
    }
    vecc.push(input[1..input.len()].to_string());
    return vecc;

//TODO: I think?
//start while loop 
//start another while loop, that continues if char 60 is a whitespace
//if not, do char - 1, if that is a whitespace, cut the string left of char and append to list
//restart on cut string, and char 60
}





