// rc1.rs

/* Up to now, the relationship between a value and its borrowed references has been clear and known at compile time. The value is the owner, and the references cannot outlive it, but many cases simply don't fit into this pattern. For example, say we have a Player struct and a Role struct. A Player keeps a vector of references to Role objects. There isn't a neat one-to-one relationship between these values, and persuading 'rustc' to cooperate becomes nasty.

'Rc' works like 'Box', heap memory is allocated and the value is moved to it. If you clone a 'Box', it allocates a full cloned copy of the value. But cloning an 'Rc' is cheap, because each time you clone it just updates a reference count to the same data. This is an old and very popular strategy for memory management, for example it's used in the Objective C runtime on iOS/MacOS. In modern C++, it is implemented with 'std::shared_ptr'. When a 'Rc' is dropped, the reference count is decremented. When that count goes to zero the owned value is dropped and the memory freed. */

// rc1.rs
use std::rc::Rc;

fn main() {
    let s = "hello dolly".to_string();
    let rs1 = Rc::new(s); // s moves to heap; ref count 1
    let rs2 = rs1.clone(); // ref count 2

    println!("len {}, {}", rs1.len(), rs2.len());
} // both rs1 and rs2 drop, string dies.

/* You may make as many references as you like to the original value; it's dynamic borrowing again. You do not have to carefully track the relationship between the value 'T' and its references '&T'. There is some runtime cost involved, so it isn't the first solution you should choose, but it makes patterns of sharing possible which would fall foul of the borrow checker. Note that 'Rc' gives you immutable shared references, since otherwise that would break one of the very basic rules of borrowing. 

In the case of a Player, it can now keep its roles as a 'Vec<Rc<Role>>' and things work out fine; we can add or remove roles but not change them after their creation.

However, what if each Player needs to keep references to a team as a vector of Player references? Then everything becomes immutable, because all the Player values need to be stored as 'Rc'. This is the place where 'RefCell' becomes necessary. The team may be then defined as 'Vec<Rc<RefCell<Player>>>'. It is now possible to change a Player value using 'borrow_mut', provided no-one has checked out a reference to a Player at the same time. For example, say we have a rule that if something special happens to a player, then all of their team gets stronger. */ 

for p in &self.team {
    p.borrow_mut().make_stronger();
}

// So the application code isn't too bad, but the type signatures get a bit scary. You can simplify them with a type alias.
type PlayerRef = Rc<RefCell<Player>>; 