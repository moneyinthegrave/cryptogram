use std::io;
use std::mem;


fn code () {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let a = vec!['Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u',
                 'i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В','А',
                 'П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р',
                 'о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю','.','!','@','#','$','%','^','&','_','+','№',':','?','*','(',')',';','"','1','2','3',
                 '4','5','6','7','8','9','0',' '];
    let b = vec!['.','!','@','#','$','%','^',' ','&','*','(',')','+','№',':','?','_',';','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В',
                'А','П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J',
                'K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',
                'й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р','о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю','-','9','5',
                '1','8','7','4','0','6','2','3'];
    let mut input2: Vec<char> = input.trim().chars().collect();
    let lenght = input2.len();
    let mut j = 0;
    let mut indexx = 0;
    loop{
        if input2[j] == a[indexx] {
            std::mem::replace(&mut input2[j], b[indexx]);
                j+=1;
            if j  == lenght {
                break;
            }
    }
        indexx+=1;
            if indexx >= a.len() {
                indexx = 0;
            }
    }
    let mut i = 0;
        while i < lenght {
                print!("{}",input2[i]);
            i+=1;
    }
}


fn decode(){
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let b = vec!['Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u',
                 'i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В','А',
                 'П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р',
                 'о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю','.','!','@','#','$','%','^','&','_','+','№',':','?','*','(',')',';','"','1','2','3',
                 '4','5','6','7','8','9','0',' '];
    let a = vec!['.','!','@','#','$','%','^',' ','&','*','(',')','+','№',':','?','_',';','Й','Ц','У','К','Е','Н','Г','Ш','Щ','З','Х','Ъ','Ф','Ы','В',
                 'А','П','Р','О','Л','Д','Ж','Э','Я','Ч','С','М','И','Т','Ь','Б','Ю',',','Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J',
                 'K','L','Z','X','C','V','B','N','M','q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',
                 'й','ц','у','к','е','н','г','ш','щ','з','х','ъ','ф','ы','в','а','п','р','о','л','д','ж','э','я','ч','с','м','и','т','ь','б','ю','-','9','5',
                 '1','8','7','4','0','6','2','3'];
    let mut input2: Vec<char> = input.trim().chars().collect();
    let lenght = input2.len();
    let mut j = 0; //
    let mut indexx = 0; //
    loop{
        if input2[j] == a[indexx] {
            std::mem::replace(&mut input2[j], b[indexx]);
            j+=1;
                if j  == lenght {
                    break;
                }
        }
        indexx+=1;
            if indexx >= a.len() {
                indexx = 0;
            }
    }
    let mut i = 0;
        while i < lenght {
            print!("{}",input2[i]);
            i+=1;
        }
}

fn main() {
    code();
    println!("  ||  ");
    decode();
}

