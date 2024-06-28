
use std::io;

fn main() {

    let days = ["first", "second", "third", "fourth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves and", "Three French hens", "Four calling birds"];

    let mut i = 0;

    for i in (0..gifts.len()){
        println!("On the {0} day of Christmas my true love sent to me", days[i]);
        for gift in (0..i+1).rev(){
            println!("{}",gifts[gift])
        }
        println!("");
    }
}
