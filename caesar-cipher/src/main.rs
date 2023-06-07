fn encrypt(input: String, shift: i32) -> String {


    let a_variation = ['Á','À', 'Ã', 'Ã'];
    let e_variation = ['É','È', 'Ê', 'Ẽ'];
    let i_variation = ['Í','Ì','Ĩ','Î'];
    let o_variation = ['Ó', 'Ò', 'Õ', 'Ô'];
    let u_variation = ['Ú','Ù','Ũ','Û'];


    // let mut output = String::new();

    let inp = input.as_str().trim();
    
    let mut cipher_text: String;
    let mut result : String;
    result = "".to_string();
    cipher_text = inp.to_uppercase().replace("Ç", "C");

    for l in 0..a_variation.len() {
        cipher_text = cipher_text.replace(a_variation[l], "A");
        cipher_text = cipher_text.replace(e_variation[l], "E");
        cipher_text = cipher_text.replace(i_variation[l], "I");
        cipher_text = cipher_text.replace(o_variation[l], "O");
        cipher_text = cipher_text.replace(u_variation[l], "U");
        
    }

    for i in 0..cipher_text.len() {
    
    let mut c = cipher_text.chars().nth(i).unwrap();
    


        if a_variation.contains(&c){
            c = 'A';
        }
        else if e_variation.contains(&c){
            c = 'E';
        }
        else if i_variation.contains(&c){
            c = 'I';
        }
        else if o_variation.contains(&c){
            c = 'O';
        }
        else if u_variation.contains(&c){
            c = 'U';
        }


        let code = c as i32;
        // print!("  {}  ", code);

        if code >= 65 && code <= 90 {

        let new_code = (code + shift-65) % 26 + 65;
        let new_char = char::from_u32(new_code as u32).unwrap();
        result.push(new_char);
        }
        else {
            result.push(c);
        }

    }
    return result;
}

fn decrypt(input: String, shift: i32) -> String {

    let a_variation = ['Á','À', 'Ã', 'Ã'];
    let e_variation = ['É','È', 'Ê', 'Ẽ'];
    let i_variation = ['Í','Ì','Ĩ','Î'];
    let o_variation = ['Ó', 'Ò', 'Õ', 'Ô'];
    let u_variation = ['Ú','Ù','Ũ','Û'];


    // let mut output = String::new();

    let inp = input.as_str().trim();
    
    let mut cipher_text: String;
    let mut result : String;
    result = "".to_string();
    cipher_text = inp.to_uppercase().replace("Ç", "C");

    for l in 0..a_variation.len() {
        cipher_text = cipher_text.replace(a_variation[l], "A");
        cipher_text = cipher_text.replace(e_variation[l], "E");
        cipher_text = cipher_text.replace(i_variation[l], "I");
        cipher_text = cipher_text.replace(o_variation[l], "O");
        cipher_text = cipher_text.replace(u_variation[l], "U");
        
    }

    for i in 0..cipher_text.len() {
    
    let mut c = cipher_text.chars().nth(i).unwrap();
    


        if a_variation.contains(&c){
            c = 'A';
        }
        else if e_variation.contains(&c){
            c = 'E';
        }
        else if i_variation.contains(&c){
            c = 'I';
        }
        else if o_variation.contains(&c){
            c = 'O';
        }
        else if u_variation.contains(&c){
            c = 'U';
        }


        let code = c as i32;
        // print!("  {}  ", code);

        if code >= 65 && code <= 90 {

        let new_code = (code - shift-65) % 26 + 65;
        let new_char = char::from_u32(new_code as u32).unwrap();
        result.push(new_char);
        }
        else {
            result.push(c);
        }

    }
    return result;
}

fn main() {
println!("{}",encrypt("Hello, World!".to_string(), 3));
println!("{}",decrypt("Khoor, Zruog!".to_string(), 3));
}
