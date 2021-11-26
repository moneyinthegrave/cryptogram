use std::io;

fn code (){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // создание двух алфавитов
    let a = String::from("QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,йцукенгшщзхъфывапролджэячсмитьбю.!@#$%^&_+№:?*();");
    let b = String::from(".!@#$%^&*()+№:?_;ЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmйцукенгшщзхъфывапролджэячсмитьбю");
    //проход по всему алфавиту и замена на второй
    for i in a.len() {
        input.replace(a[i],b[i]);
    }
    println!{"{}", input};
}

fn decode(){

}

fn main() {
    code();
    decode();
}


