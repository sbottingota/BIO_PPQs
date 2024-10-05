static NUMERALS: [(u32, &str); 13]  = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),   
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I")
];

pub fn look_and_say(old: &String) -> String {
    let mut new = String::new();
    let mut current: char = old.chars().nth(0).unwrap();
    let mut length: u32 = 0;

    for c in old.chars() {
        if c != current {
            new.push_str(&to_roman(length));
            new.push(current);

            length = 0;
            current = c;
        } 

        length += 1;
    }

    new.push_str(&to_roman(length));
    new.push(current);

    new
}

pub fn to_roman(mut n: u32) -> String {
    let mut numeral = String::new();
    for (key, value) in NUMERALS {
        while key <= n {
            n -= key;
            numeral.push_str(value);
        }
    }

    numeral
}

