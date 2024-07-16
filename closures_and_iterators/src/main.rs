use std::thread;
#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor {
    Red,Blue
}

struct Inventory {
    shirts:Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // The closure captures an immutable reference to the
        // self Inventory instance and passes it with the code we specify
        // to the unwrap_or_else method. Functions, on the other hand, are not able to
        // capture their environment in this way.
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

}

fn main(){
    let store = Inventory{
        shirts: vec![ShirtColor::Blue,ShirtColor::Blue,ShirtColor::Red]
    };
    let user_pref = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref);
    println!("user with pref {:?} receives {:?}",user_pref,giveaway1);
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("user with pref {:?} receives {:?}",user_pref2,giveaway2);

    //closures borrowing values
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    //cannot define without 'mut' as the closure mutably borrows the vec
    //println!("After calling closure: {:?}", list); -> this line will give an error as the list is mutably borrowed and no other borrows are supported now.
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //moving to another thread so the variable is not dropped while the new thread is still executing
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

}
