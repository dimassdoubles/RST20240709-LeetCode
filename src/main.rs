use std::collections::HashMap;

fn main() {
    // penulisan romawi besar -> kecil, kiri -> kanan
    // anomali ketika ada romawi kecil di kiri besasr

    let rom = String::from("MCMXCIV");
    let mut result: i32 = 0;

    let symbols: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    
    let mut is_add = true;
    for i in (1..rom.len()).rev() {
        let left_val = symbols.get(&rom.chars().nth(i-1).unwrap()).unwrap();
        let right_val = symbols.get(&rom.chars().nth(i).unwrap()).unwrap();

        if left_val > right_val {
            is_add = true;
        } else if left_val < right_val {
            is_add = false;
        }

        if is_add {
            result += left_val;
        } else {
            result -= left_val;
        }
    }

    println!("{}", result);

}