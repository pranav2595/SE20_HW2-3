use std::collections::HashMap;
use std::collections::HashSet;

use std::time::Duration;
use std::thread::sleep;

type Cell = (i32, i32);
type Alives = HashSet<Cell>;
 
fn print_colony(col: &Alives, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            print!("{} ",
                if col.contains(&(x, y)) {"1"}
                else {"."}
            );
        }
        println!();
    }
}
 
fn neighbours(&(x,y): &Cell) -> Vec<Cell> {
    vec![
        (x-1,y-1), (x,y-1), (x+1,y-1),
        (x-1,y),            (x+1,y),
        (x-1,y+1), (x,y+1), (x+1,y+1),
    ]
}
 
fn neighbour_counts(col: &Alives) -> HashMap<Cell, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    // for (key, value) in &ncnts {
    //     println!("({},{}): {}", key.1,key.0, value);
    // }
    ncnts
}
 
fn generation(col: Alives) -> Alives {
    neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)|
            match (cnt, col.contains(&cell)) {
                (2, true) |
                (3, ..) => Some(cell),
                _ => None
        })
        .collect()
}
 
fn life(init: Vec<Cell>, iterations: i32, width: i32, height: i32) {
    let mut col: Alives = init.into_iter().collect(); 
    for i in 0..iterations+1
    {
        println!("({})", &i);
        if i != 0 {
            col = generation(col);
        }
        print_colony(&col, width, height);
        sleep(Duration::from_millis(200));
        // Clearing the console
        print!("\x1B[2J\x1B[1;1H");
    }
}
 
fn main() { 
    // Glider shape which moves through the Cartesian plane.
    let shape1: Vec<(i32, i32)> = vec![
                (1,0),
                        (2,1),
        (0,2),  (1,2),  (2,2)];
 
 
    // Shape which changes continuously with repetitions after 4 iterations.
    let shape2: Vec<(i32, i32)> = vec![
              (10,10),
      (9,11),(10,11),(11,11)

    ];
    // life(shape1,1,20,20);
    life(shape1, 190, 30, 30);
    life(shape2, 190, 30, 30);
    

}
 