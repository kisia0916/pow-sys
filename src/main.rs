
use sha3::Shake256;
use sha3::digest::*;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold("".to_owned(), |s, b| format!("{}{:x}", s, b) )
}
fn sha256_hasher(content:&String)->String{
    let mut hasher = Shake256::default();
    hasher.update(content);
    let size:usize = 32;
    let result = hasher.finalize_boxed(size);
    let result_decode = hex(&result);
    result_decode
}
fn check_pow(hash:String)->bool{
    let zero_num = 2;
    let first_texts = &hash[0..zero_num];
    let mut _co = 0;
    for i in 0..5{
        if let Some(c) = first_texts.chars().nth(i){
            if c == '0'{
                _co+=1;
            }
        }
    }
    if _co == zero_num {true} else {false}
}
fn main() {
    let defalut_content = String::from("hello block-chain!");
    let mut add_number:String = String::from("0");
    let mut done_flg = false;
    println!("mining now.....");
    while !done_flg {
        let mut _ans_num:u32 = add_number.clone().parse().unwrap();
        _ans_num+=1;
        add_number = format!("{_ans_num}");
        let hash_text = format!("{defalut_content}{_ans_num}");
        done_flg = check_pow(sha256_hasher(&hash_text));
        if done_flg{
            println!("{}",&hash_text)
        }
    }
}
