fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        usage();
        return;
    }
    match args[1].as_str() {
        "-d" => {
            if args.len() <= 2 {
                // encode
                encode(&args[1..]);
                return;
            }
            decode(&args[2..]);
        }
        _ => {
            encode(&args[1..]);
        }
    }
}

fn decode(nums: &[String]) {
    println!("|  CHR  |  DEC  |  HEX  |");
    println!("+-------+-------+-------+");
    let v: Vec<u8> = nums.iter().map(|x| {
        if x.starts_with("0x") {
            u8::from_str_radix(&x[2..], 16).unwrap()
        } else if x.starts_with("0b") {
            u8::from_str_radix(&x[2..], 2).unwrap()
        } else if x.starts_with("0") {
            u8::from_str_radix(&x[1..], 8).unwrap()
        } else {
            u8::from_str_radix(x, 10).unwrap()
        }
    }).collect();
    for x in v.iter() {
        println!("| {:>5} |  {:>3}  |  0x{:>02X} |", ASCII_LIST[*x as usize], x, x);
    }
    println!("+-------+-------+-------+");
}

fn encode(texts: &[String]) {
    println!("|  CHR  |  DEC  |  HEX  |");
    println!("+-------+-------+-------+");
    for text in texts.iter() {
        let text = text.clone();
        for x in text.into_bytes().iter() {
            println!("| {:>5} |  {:>3}  |  0x{:>02X} |", ASCII_LIST[*x as usize], x, x);
        }
        println!("+-------+-------+-------+");
    }
}

fn usage() {
    println!("ascii code checker");
    println!("https://github.com/takubokudori/ascii");
    println!();
    println!("-d [num]...");
    println!("[string]...");
}

static ASCII_LIST: [&str; 128] = ["[NUL]"
    , "[SOH]"
    , "[STX]"
    , "[ETX]"
    , "[EOT]"
    , "[ENQ]"
    , "[ACK]"
    , "[BEL]"
    , "[B S]"
    , "[H T]"
    , "[L F]"
    , "[V T]"
    , "[F F]"
    , "[C R]"
    , "[S O]"
    , "[S I]"
    , "[DLE]"
    , "[DC1]"
    , "[DC2]"
    , "[DC3]"
    , "[DC4]"
    , "[NAK]"
    , "[SYN]"
    , "[ETB]"
    , "[CAN]"
    , "[E M]"
    , "[SUB]"
    , "[ESC]"
    , "[F S]"
    , "[G S]"
    , "[R S]"
    , "[U S]"
    , "[   ]"
    , "[ ! ]"
    , "[ \" ]"
    , "[ # ]"
    , "[ $ ]"
    , "[ % ]"
    , "[ & ]"
    , "[ ' ]"
    , "[ ( ]"
    , "[ ) ]"
    , "[ * ]"
    , "[ + ]"
    , "[ , ]"
    , "[ - ]"
    , "[ . ]"
    , "[ / ]"
    , "[ 0 ]"
    , "[ 1 ]"
    , "[ 2 ]"
    , "[ 3 ]"
    , "[ 4 ]"
    , "[ 5 ]"
    , "[ 6 ]"
    , "[ 7 ]"
    , "[ 8 ]"
    , "[ 9 ]"
    , "[ : ]"
    , "[ ; ]"
    , "[ < ]"
    , "[ = ]"
    , "[ > ]"
    , "[ ? ]"
    , "[ @ ]"
    , "[ A ]"
    , "[ B ]"
    , "[ C ]"
    , "[ D ]"
    , "[ E ]"
    , "[ F ]"
    , "[ G ]"
    , "[ H ]"
    , "[ I ]"
    , "[ J ]"
    , "[ K ]"
    , "[ L ]"
    , "[ M ]"
    , "[ N ]"
    , "[ O ]"
    , "[ P ]"
    , "[ Q ]"
    , "[ R ]"
    , "[ S ]"
    , "[ T ]"
    , "[ U ]"
    , "[ V ]"
    , "[ W ]"
    , "[ X ]"
    , "[ Y ]"
    , "[ Z ]"
    , "[ [ ]"
    , "[ \\ ]"
    , "[ ] ]"
    , "[ ^ ]"
    , "[ _ ]"
    , "[ ` ]"
    , "[ a ]"
    , "[ b ]"
    , "[ c ]"
    , "[ d ]"
    , "[ e ]"
    , "[ f ]"
    , "[ g ]"
    , "[ h ]"
    , "[ i ]"
    , "[ j ]"
    , "[ k ]"
    , "[ l ]"
    , "[ m ]"
    , "[ n ]"
    , "[ o ]"
    , "[ p ]"
    , "[ q ]"
    , "[ r ]"
    , "[ s ]"
    , "[ t ]"
    , "[ u ]"
    , "[ v ]"
    , "[ w ]"
    , "[ x ]"
    , "[ y ]"
    , "[ z ]"
    , "[ { ]"
    , "[ | ]"
    , "[ } ]"
    , "[ ~ ]"
    , "[DEL]"
];

#[test]
fn test() {
    println!("**************** decode ****************");
    decode(&["0x41".to_string(), "65".to_string(), "0101".to_string(), "0b01000001".to_string()]); // AAAA
    println!("**************** encode ****************");
    encode(&["AAAA".to_string(), "test".to_string(), "0123".to_string()]);
}
