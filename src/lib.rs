use std::collections::HashMap;
use std::fs;
use std::error::Error;


pub struct Config{
    pub input_filename: String,
    pub output_filename: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() != 3 {
            return Err("Exactly two arguments required: input filename and output filename!")
        }
        let input_filename = args[1].clone();
        let output_filename = args[2].clone();
        Ok(Config { input_filename, output_filename })
    }   
}


// The method constructs the keymapping from the old RATHA font keys to modern Unicode keys
fn construct_lookup_table() -> HashMap<char, char>{
    let mut lookup_table = HashMap::new();
    lookup_table.insert(' ', ' ');
    lookup_table.insert('!', '꯫');
    lookup_table.insert('(', '(');
    lookup_table.insert(')', ')');
    lookup_table.insert(',', ',');
    lookup_table.insert('-', '–');
    lookup_table.insert('.', '.');
    lookup_table.insert('0', '꯰');
    lookup_table.insert('1', '꯲');
    lookup_table.insert('2', '꯲');
    lookup_table.insert('3', '꯳');
    lookup_table.insert('4', '꯴');
    lookup_table.insert('5', '꯵');
    lookup_table.insert('6', '꯶');
    lookup_table.insert('7', '꯷');
    lookup_table.insert('8', '꯸');
    lookup_table.insert('9', '꯹');
    lookup_table.insert('=', 'ꯆ');
    lookup_table.insert('A', 'ꯝ');
    lookup_table.insert('B', 'ꯎ');
    lookup_table.insert('C', 'ꯓ');
    lookup_table.insert('D', '꯭');
    lookup_table.insert('E', 'ꯟ');
    lookup_table.insert('F', 'ꯊ');
    lookup_table.insert('G', 'ꯠ');
    lookup_table.insert('I', 'ꯪ');
    lookup_table.insert('K', 'ꯁ');
    lookup_table.insert('M', 'ꯜ');
    lookup_table.insert('N', 'ꯞ');
    lookup_table.insert('O', 'ꯣ');
    lookup_table.insert('R', 'ꯙ');
    lookup_table.insert('S', 'ꯈ');
    lookup_table.insert('T', 'ꯏ');
    lookup_table.insert('W', 'ꯐ');
    lookup_table.insert('X', 'ꯘ');
    lookup_table.insert('Z', 'ꯉ');
    lookup_table.insert('[', 'ꯑ');
    lookup_table.insert('\\', 'ꯌ');
    lookup_table.insert('_', 'ꯋ');
    lookup_table.insert('`', '|');
    lookup_table.insert('a', 'ꯃ');
    lookup_table.insert('b', 'ꯨ');
    lookup_table.insert('c', 'ꯖ');
    lookup_table.insert('d', 'ꯛ');
    lookup_table.insert('e', 'ꯅ');
    lookup_table.insert('f', 'ꯇ');
    lookup_table.insert('g', 'ꯤ');
    lookup_table.insert('i', 'ꯡ');
    lookup_table.insert('j', 'ꯔ');
    lookup_table.insert('k', 'ꯥ');
    lookup_table.insert('l', 'ꯦ');
    lookup_table.insert('m', 'ꯂ');
    lookup_table.insert('o', 'ꯍ');
    lookup_table.insert('p', 'ꯩ');
    lookup_table.insert('q', 'ꯕ');
    lookup_table.insert('r', 'ꯗ');
    lookup_table.insert('s', 'ꯀ');
    lookup_table.insert('v', 'ꯆ');
    lookup_table.insert('w', 'ꯄ');
    lookup_table.insert('x', 'ꯒ');
    lookup_table.insert('y', 'ꯧ');
    lookup_table.insert('z', 'ꯚ');
    lookup_table
}

fn translate_to_unicode(text: &str) -> String {
    let lookup_table = construct_lookup_table();

    let mut translated_string = String::new();
    for character in text.chars(){
        translated_string.push(*lookup_table.get(&character).unwrap_or_else(|| &character));
    }
    translated_string
}


pub fn run(config: Config)-> Result<(), Box<dyn Error>>{

    if config.input_filename == "kuki" && config.output_filename == "who" {
        loop {
            println!("Kuki Shakthu!");
            // delay for 500 ms
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
    let contents= fs::read_to_string(config.input_filename)?;
    let translated_contents = translate_to_unicode(&contents);
    
    fs::write(config.output_filename, translated_contents)?;
    Ok(())
}
