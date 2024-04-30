/* This is a library that randomly returns fruits native
to Portugal.

THe style of the code is that we use a const array of stringd to store the fruits.
We then use ths const in a funstion that later gets called in the main.rs file as CLI.

The CLI should suppor the following:

//The quantity of the fruits to be returned 
--count 5*/

use rand::Rng;


const FRUITS: [&str; 5] = ["Apple", "Banana", "Orange", "Pineapple", "Strawberry"];

/*returns a random fruit from the FRUITS vector, also accepts a count parameter that specifies the number of fruits to return*/
pub fn get_fruits(count: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut fruits = Vec::new();
    for _ in 0..count {
        let fruit = FRUITS[rng.gen_range(0..FRUITS.len())];
        fruits.push(fruit.to_string());
    }
    fruits
}