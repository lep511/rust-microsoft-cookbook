fn main() {
    let sentence = "* __Primer paso__ [link](www.google.com.uy) `salida`";
    
    // Using a const array for better compile-time optimization
    const ESCAPE_CHARS: [char; 19] = [
        '\\', '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', 
        '+', '-', '=', '|', '{', '}', '.', '!'
    ];
    
    // Preallocate string with estimated capacity
    let estimated_size = sentence.len() * 2;
    let mut new_sentence = String::with_capacity(estimated_size);
    
    // Using bytes iterator which is faster than chars() for ASCII
    sentence.bytes().for_each(|b| {
        let c = b as char;
        if ESCAPE_CHARS.contains(&c) {
            new_sentence.push('\\');
            new_sentence.push(c);
        } else {
            new_sentence.push(c);
        }
    });
    
    println!("{}", new_sentence);
}