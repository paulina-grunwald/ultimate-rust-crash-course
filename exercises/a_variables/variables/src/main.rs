const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT:i32 = 2;
fn main() {
    let mut missiles:i32 = 9;
    let ready:i32 = 2;
    missiles = missiles - ready;
    let ( new_missless, new_ready): (i32, i32) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("Firing {} of my {} missiles...", STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", new_ready, new_missless);
}
