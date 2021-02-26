use std::io;

fn fibonacchi_recursion(n: i32) -> i32 {

    if n==0 {
        return 0;
    }
    else if n==1 {
        return n;
    }
    else {
        return fibonacchi_recursion(n-1) + fibonacchi_recursion(n-2);
    }
}

fn main() {
    println!("Please enter nth word of the sequence: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            let n: i32 = input.trim().parse().unwrap();
            println!("{}", fibonacchi_recursion(n));
        },
        Err(e) => println!("Oops, something went wrong: {}", e)
    }
}
