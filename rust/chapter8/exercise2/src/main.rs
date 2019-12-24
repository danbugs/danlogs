/*
Convert strings to pig latin. 
The first consonant of each word is moved to the end of the 
word and “ay” is added, so “first” becomes “irst-fay.” 
Words that start with a vowel have “hay” added to the end 
instead (“apple” becomes “apple-hay”). 
Keep in mind the details about UTF-8 encoding!
*/
fn main() {
    /* 
    NOTE: This program assumes all
    characters within a given string
    need the same number of bytes to
    be represented
    
    i.e. "дi" gives an error
    - д: 2 bytes
    - i: 1 byte
    */
    let mut s = "дд".to_string();
    let mut counter = 0;
    for _ in s.chars(){
        counter +=1;
    }
    let len_of_char = s.bytes().len()/counter;
    if &s[0..len_of_char] == "a" || 
    &s[0..len_of_char] == "e" ||
    &s[0..len_of_char] == "i" ||
    &s[0..len_of_char] == "o" ||
    &s[0..len_of_char] == "u"{
        s.push_str(&format!("-{}", "hay".to_string()));
    }
    else{
        s = format!("{}-{}{}", &s[len_of_char..], &s[0..len_of_char], "ay".to_string());
    }
    println!("{}", s);
}
