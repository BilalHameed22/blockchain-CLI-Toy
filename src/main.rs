// Removed unnecessary extern crate for serde_derive (Rust 2018+)
use std::io;
use std::process;
use std::io::Write;

mod blockchain;
fn main() {
   let mut miner_addr = String::new();
   let mut difficulty = String::new();
   let mut choice = String::new();

   print!("Input a miner address:");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut miner_addr).unwrap();
   print!("Difficulty:");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut difficulty).unwrap();
   let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
   let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
   loop {
    println!("Menu");
    println!("1) New Transaction");
    println!("2) Mine Block");
    println!("3) Change Difficulty");
    println!("4) Change Reward");
    println!("0) Exit");
    println!("Enter your choice: ");
    io::stdout().flush();
    choice.clear();
    io::stdin().read_line(&mut choice);
    println!("");

    match choice.trim().parse().unwrap(){
        0=>{
            println!("exiting");
            process::exit(0);
        },
        1=>
        {
            let mut sender=String::new();
            let mut receiver = String::new();
            let amount = String::new();

            print!("Enter sender address:");
            io::stdout().flush();
            io::stdin().read_line(&mut sender);
            print!("Enter receiver address:");
            io::stdout().flush();
            io::stdin().read_line(&mut receiver);
            print!("Enter amount to send:");
            io::stdout().flush();
            io::stdin().read_line(&mut amount);

            let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());
             
             match res {
                true => println!("Transaction added"),
                false => println!("Transaction failed"),
             }

        
        },
        2=> {
            println!("Generating block");
            let res = chain.generate_new_block();
            match res {
                true => println!("Block generated successfully"),
                false => println!("block generation failed"),
            }

        },
        3=>{
            let mut new_diff = String::new();
            print!("Enter new difficulty:");
            io::stdout().flush();
            io::stdin().read_line(&mut new_diff);
            chain.update_difficluty(new_diff.trim().parse().unwrap());
            match res {
                true => println!("Updated Difficulty"),
                false => println!("Failed to update Difficluty"),
            }
        },
        4=>{
            let mut new_reward = String::new();
            print!("Enter new reward:");
            io::stdout().flush();
            io::stdin().read_line(&mut new_reward);
            chain.update_reward(new_reward.trim().parse().unwrap());
            match res {
                true => println!("Updated reward"),
                false => println!("Failed to update reward"),
            }

        },
        _=> println!("\t invalid option please retry\t"),
    }
    
   }


}
