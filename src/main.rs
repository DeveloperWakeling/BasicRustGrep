use std::env; //Needed for accessing the environment args


fn main() {
    //Accepting command line arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if &args.len() > &1 {
        let paramOne = &args[1];
        println!("{}", paramOne);
    }
    else {
        println!("no value passed");
    }

    //Adding to database file (plain txt file should work)

    //Retreiving from txt file
}
