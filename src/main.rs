use std::io;
fn code () {
    let mut slovo = String::new();
    io::stdin().read_line(&mut slovo);
    let slovo_2 = String::from(slovo);
    let slovo_2 : String = slovo_2.replace("q", "х").replace("w", "щ").replace("e", "ю").replace("r", "а")
        .replace("t", "м").replace("y", "ж").replace("u", "л").replace("i", "у").replace("o", "с").replace("p", "и")
        .replace("a", "ф").replace("s", "з").replace("d", "э").replace("f", "п").replace("g", "в")
        .replace("h", "т").replace("j", "я").replace("k", "н").replace("l", "д").replace("z", "ы").replace("x", "ь")
        .replace("c", "й").replace("v", "к").replace("b", "ъ").replace("n", "ч").replace("m", "р");
    println!("{}", slovo_2);
}
fn decode () {
    let mut slovo = String::new();
    io::stdin().read_line(&mut slovo);
    let slovo_2 = String::from(slovo);
    let slovo_2 : String = slovo_2.replace("х","q" ).replace("щ","w" ).replace("ю","e" ).replace("а","r" )
        .replace("м","t" ).replace("ж","y" ).replace("л","u" ).replace("у","i" ).replace("с","o" ).replace("и","p" )
        .replace("ф","a" ).replace("з","s" ).replace("э","d" ).replace("п","f" ).replace("в","g" )
        .replace("т","h" ).replace("я","j" ).replace("н","k" ).replace("д","l" ).replace("ы","z" ).replace("ь","x" )
        .replace("й","c" ).replace("к","v" ).replace("ъ", "b").replace("ч","n" ).replace("р","m" );
    println!("{}", slovo_2);
}
fn main() {
    println!("code");
    code();
    println!("decode");
    decode();
}
