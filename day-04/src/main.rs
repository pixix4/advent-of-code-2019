fn main() {
    let min = 147_981;
    let max = 691_423;

    let mut count = 0;
    for candiate in min..=max {
        let s = candiate.to_string();

        let chars: Vec<i8> = s.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();

        let mut last = chars[0];
        let mut repeats = 0;
        let mut does_repeat = false;
        let mut is_desending = false;
        for x in &chars[1..] {
            let x = *x;

            if last == x {
                repeats += 1;
            } else {
                if repeats == 1 {
                    does_repeat = true;
                }
                repeats = 0;
            }

            if x < last {
                is_desending = true;
                break;
            }

            last = x;
        }
        if repeats == 1 {
            does_repeat = true;
        }

        if !does_repeat || is_desending {
            continue;
        }

        println!("Match: {}", candiate);
        count += 1;
    }

    println!("Count: {}", count);
}
