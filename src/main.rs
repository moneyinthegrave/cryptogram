use std::io;
use std::mem;
fn code () {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // создание двух алфавитов
     /*let a = String::from("QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,йцукенгшщзхъфывапролджэячсмитьбю.!@#$%^&_+№:?*();");
       let b = String::from(".!@#$%^&*()+№:?_;ЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnmйцукенгшщзхъфывапролджэячсмитьбю");
    let  v: Vec<char> = a.chars().collect();
    let v2: Vec<char> = b.chars().collect();
    let mut v3: Vec<char> = input.trim().chars().collect();

    //проход по всему алфавиту и замена на второй
    for i in 0..134 {
        if v3[i]==v[i] {
        let input = std::mem::replace(
            &mut v3[i],
            v2[i]
        );
        println!("{}", input);}*/
    let a = vec!['Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В','А','П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р','о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю','.','!','@','#','$','%','^','&','_','+','№',':','?','*','(',')',';','"'];
    let b = vec!['.','!','@','#','$','%','^','&','*','(',')','+','№',':','?','_',';','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В','А','П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m','й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р','о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю',"'"];
    let lenght = input.trim().len();
    println!("{}",lenght);
    let mut input2: Vec<char> = input.trim().chars().collect();
    let mut j = 0;
    let mut indexx = 0;
    let mut popitki = 0;
    while (true) {

    if input2[j] == a[indexx] {
        popitki+=1;
        println!("{} POPITKI", popitki);
        println!("вошел в первый");
    let got = std::mem::replace(&mut input2[j], b[indexx]);
        println!("{:?}",got);
    j+=1;
        if j == lenght {
          println!("  вошел во второй");
            j = 0;
            indexx+=1;

            if lenght  == popitki {
                println!("закончил");
                break;
            }
        }
        indexx+=1;
    }
        indexx+=1;
        if indexx == 16 {
            indexx = 0;
        }
    }
    let mut i = 0;
    while i < lenght {
print!("{}",input2[i]);
        i+=1;
    }O
}





fn decode(){

}

fn main() {
    code();
    decode();
}


