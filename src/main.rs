use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let a = String::from("QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,йцукенгшщзхъфывапролджэячсмитьбю.!@#$%^&_+№:?*();");
    let b = String::from(".!@#$%^&*()+№:?_;ЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmйцукенгшщзхъфывапролджэячсмитьбю");
        for i in a.len() {
        input.replace(a[i],b[i]);
    }
    println!{"{}", input};
}


