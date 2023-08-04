use std::io;
pub fn order(){
    let mut array: Vec<u32> = vec![];
    let mut counter = 0;
    while counter < 5{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read from stdin");
    
        let trimmed = input.trim();
        match input.trim().parse::<u32>() {
            Ok(i) => {
                println!("okay, good number!");
                counter += 1;
                array.push(i);
            },
            Err(..) => {
                println!("this was not an integer: {}, please enter a number.", trimmed);
            },
        };
    }
    for i in 0..5 {
        for j in i..5{
            if array[i] > array[j]{
                let temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
    for i in 0..5{
        print!("{} ", array[i]);
    }
}