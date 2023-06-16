// use substring::Substring;
use std::env;
use rand::seq::SliceRandom;
use home::home_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
   
        if args.len() > 1 { 
        let query = &args[1];
        let prompt = &args[2];
        if query == "-scroll" {sayy(prompt.to_string())}
        else {sayy("".to_string());}}
    else {sayy("".to_string())}
    // let input = &args[2];

    
    // println!("|￣￣￣￣￣￣￣￣￣￣|\n| GOD MAY JUDGE YOU, |\n|   BUT HIS SINS     |\n| OUTNUMBER YOUR OWN |\n|＿＿＿＿＿＿＿＿＿＿|\n(\\__/) ||\n(•ㅅ•) || \n/ 　  づ\n");
    // println!("{}", say);

     
    
    // read();
}

fn sayy(mut inp: String) {
    let dragontop = r"                                  /   \
                          )      ((   ))     (
(@)                      /|\      ))_((     /|\
|-|\.                   / | \    (/\|/\)   / | \                      (@)
| |--------------------/--|-voV---\`|'/--Vov-|--\---------------------|-|
|-|                         '^`   (o o)  '^`                          | |
| |                               `\Y/'                               |-|
| |                                                                   | |";
    let dragonbot = r"| |                                                                   |-|
|_|___________________________________________________________________| |
(@)                 I   /\ /      ( (       \ /\   I                `\|-|
                    I /   V        \ \       V   \ I                  (@)
                    I/              | |           \I 
                                   _) )_ 
                                   `\ /'
";
    let signpre = r"| |     ";
    println!("{}", dragontop);
    //
    // println!("{}, {}", generateString(), &generateString()[1..8]);
    let mut text = Vec::<String>::new();
    if inp != "" {
        text = formatString(inp);
    } else {
        text = formatString(generateString().to_string());
    }

    let finalstr = String::new;
    if text.len() == 1 {
        finalstr = signpre.to_string() + &" ".repeat(29 - (text[0].len() / 2))[..];
    }
    for item in text {
        let signpost = " ".repeat(62 - item.len());
        println!("{}{}{}| |",signpre, item, signpost);
    }

    println!("{}", dragonbot);
}
// extern crate random;

fn generateString() -> String {
    let home = home_dir().expect("READON").into_os_string().into_string().unwrap();
    // println!("{}", home);
    let options = lines_from_file(home + "/lines");
    // let mut rng = rand::thread_rng();
    let pick = options.choose(&mut rand::thread_rng()).unwrap();
    return pick.to_string()
}

fn formatString(mut input: String) -> Vec<String> {
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

//start while loop 
//start another while loop, that continues if char 60 is a whitespace
//if not, do char - 1, if that is a whitespace, cut the string left of char and append to list
//restart on cut string, and char 60
}


use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file (please put it in home)");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


