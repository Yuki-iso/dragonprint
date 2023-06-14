// use substring::Substring;


fn main() {
    // println!("|￣￣￣￣￣￣￣￣￣￣|\n| GOD MAY JUDGE YOU, |\n|   BUT HIS SINS     |\n| OUTNUMBER YOUR OWN |\n|＿＿＿＿＿＿＿＿＿＿|\n(\\__/) ||\n(•ㅅ•) || \n/ 　  づ\n");
    // println!("{}", say);
    sayy();
}

fn sayy() {
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
    let text = formatString(generateString().to_string());
    for item in text {
        let signpost = " ".repeat(62 - item.len());
        println!("{}{}{}| |",signpre, item, signpost);
    }

    println!("{}", dragonbot)
}
use rand::Rng;
fn generateString() -> &'static str {
    let mut options: Vec<&str> = vec![];
    let mut rng = rand::thread_rng();
    options.push("GOD MAY JUDGE YOU, BUT HIS SINS OUTNUMBER YOUR OWN");
    options.push("You know, it's funny... when you look at someone through rose-colored glasses, all the red flags just look like flags.");
    options.push("I've got good news. You see, there's no need to wonder where your god is, 'cause he's right here! And he's fresh out of mercy.");
    options.push("I commend my soul to any god that can find it.");
    options.push("If you feel like the dumbest person in the room, then you are in the right room.");
    options.push("Oh I believe in God, alright. I just don't believe the bastard deserves to be worshipped.");
    options.push("The bar was so low it was practically an tripping hazard in hell, yet here you are, limbo dancing with the devil.");
    options.push("Whenever I collapse is purely up to the gods.");
    options.push("This makes me want to throw a flashbang into a room with epileptic kids."); 
    options.push("Congratulations, you've won yourself a mandatory trip to our euthanasia center.");
    return options[rng.gen_range(0..options.len())];
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
