extern crate rand;

use rand::prelude::*; // For random number generation
use std::thread;


static mut SOLVE_DEPTH: u32 = 9;

fn main() {
    let cube3 = build_cube();
    let cube3 = cube3.scramble_cube(8);
    cube3.print_cube();
    println!(
        "Applied {} random moves to cube: {:?}",
        cube3.num_moves, cube3.previous_moves
    );
    let cube3 = cube3.forget_moves();

    let handles: Vec<_> = starter_cubes(cube3)
        .into_iter()
        .map(|c| {
            thread::spawn(move || {
                solve_cube(c);
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
fn starter_cubes(old_cube: Cube) -> Vec<Cube> { //The first 144 cubes for thread seed data.
    let in_cube = old_cube.copy_cube();
    let mut out_cubes1 = vec![];
    out_cubes1.push(in_cube.rotate_bottom_clockwise());
    out_cubes1.push(in_cube.rotate_bottom_counter_clockwise());
    out_cubes1.push(in_cube.rotate_down_clockwise());
    out_cubes1.push(in_cube.rotate_down_counter_clockwise());
    out_cubes1.push(in_cube.rotate_facing_clockwise());
    out_cubes1.push(in_cube.rotate_facing_counter_clockwise());
    out_cubes1.push(in_cube.rotate_left_clockwise());
    out_cubes1.push(in_cube.rotate_left_counter_clockwise());
    out_cubes1.push(in_cube.rotate_right_clockwise());
    out_cubes1.push(in_cube.rotate_right_counter_clockwise());
    out_cubes1.push(in_cube.rotate_up_clockwise());
    out_cubes1.push(in_cube.rotate_up_counter_clockwise());
    let mut out_cubes = vec![];
    for in_cube in &out_cubes1{
    out_cubes.push(in_cube.rotate_bottom_clockwise());
    out_cubes.push(in_cube.rotate_bottom_counter_clockwise());
    out_cubes.push(in_cube.rotate_down_clockwise());
    out_cubes.push(in_cube.rotate_down_counter_clockwise());
    out_cubes.push(in_cube.rotate_facing_clockwise());
    out_cubes.push(in_cube.rotate_facing_counter_clockwise());
    out_cubes.push(in_cube.rotate_left_clockwise());
    out_cubes.push(in_cube.rotate_left_counter_clockwise());
    out_cubes.push(in_cube.rotate_right_clockwise());
    out_cubes.push(in_cube.rotate_right_counter_clockwise());
    out_cubes.push(in_cube.rotate_up_clockwise());
    out_cubes.push(in_cube.rotate_up_counter_clockwise());
    }
    out_cubes
}
fn solve_cube(in_cube: Cube) -> () {
    unsafe {
        if in_cube.num_moves >= SOLVE_DEPTH {
            return;
        }
    }
    if in_cube.is_solved() {
        unsafe {
            if in_cube.num_moves < SOLVE_DEPTH {
                SOLVE_DEPTH = in_cube.num_moves;
            }
        }
        println!(
            "Found solution in {} moves:  {:?}",
            in_cube.num_moves, in_cube.previous_moves
        );
    }
     else {
        //This statement prevents doing the opposite of the previous move.
        // println!("Current level: {}", in_cube.num_moves);
        let last_move = in_cube.previous_moves.last().unwrap().as_str();
        match last_move {
            "F" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "F`" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "U" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
            }

            "U'" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "D" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "D`" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "L" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "L`" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "R" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "R`" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "B" => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            "B`" => {
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
            _ => {
                solve_cube(in_cube.rotate_bottom_clockwise());
                solve_cube(in_cube.rotate_bottom_counter_clockwise());
                solve_cube(in_cube.rotate_down_clockwise());
                solve_cube(in_cube.rotate_down_counter_clockwise());
                solve_cube(in_cube.rotate_facing_clockwise());
                solve_cube(in_cube.rotate_facing_counter_clockwise());
                solve_cube(in_cube.rotate_left_clockwise());
                solve_cube(in_cube.rotate_left_counter_clockwise());
                solve_cube(in_cube.rotate_right_clockwise());
                solve_cube(in_cube.rotate_right_counter_clockwise());
                solve_cube(in_cube.rotate_up_clockwise());
                solve_cube(in_cube.rotate_up_counter_clockwise());
            }
        }
    }
}

struct Side {
    faces: Vec<char>,
}
impl Side {
    fn copy_side(&self) -> Side {
        let old_faces = &self.faces;
        Side {
            faces: old_faces.clone(),
        }
    }
    fn is_solved(&self) -> bool {
        //A side is solved if all faces are equal
        let first_face = &self.faces[0];
        for face in &self.faces {
            if first_face != face {
                return false;
            }
        }
        true
    }
    fn _to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.faces[0],
            self.faces[1],
            self.faces[2],
            self.faces[3],
            self.faces[4],
            self.faces[5],
            self.faces[6],
            self.faces[7],
            self.faces[8]
        )
    }
}
struct Cube {
    sides: Vec<Side>,
    previous_moves: Vec<String>,
    num_moves: u32,
}
impl Cube {
    fn copy_cube(&self) -> Cube {
        Cube {
            sides: vec![
                self.sides[0].copy_side(),
                self.sides[1].copy_side(),
                self.sides[2].copy_side(),
                self.sides[3].copy_side(),
                self.sides[4].copy_side(),
                self.sides[5].copy_side(),
            ],
            previous_moves: self.previous_moves.clone(),
            num_moves: self.num_moves,
        }
    }
    fn is_solved(&self) -> bool {
        //A cube is solved if all sides have all the same colours on their faces.
        for side in &self.sides {
            if side.is_solved() == false {
                return false;
            }
        }
        true
    }

    fn print_cube(&self) -> () {
        println!(
            "
             {}{}{}
             {}{}{}
             {}{}{}
            -----
         {}{}{}|{}{}{}|{}{}{}|{}{}{}
         {}{}{}|{}{}{}|{}{}{}|{}{}{}
         {}{}{}|{}{}{}|{}{}{}|{}{}{}
            -----
             {}{}{}
             {}{}{}
             {}{}{}
        ",
            //top cube
            self.sides[1].faces[0],
            self.sides[1].faces[1],
            self.sides[1].faces[2],
            self.sides[1].faces[3],
            self.sides[1].faces[4],
            self.sides[1].faces[5],
            self.sides[1].faces[6],
            self.sides[1].faces[7],
            self.sides[1].faces[8],
            //middle cubes, first row
            self.sides[4].faces[0],
            self.sides[4].faces[1],
            self.sides[4].faces[2],
            self.sides[0].faces[0],
            self.sides[0].faces[1],
            self.sides[0].faces[2],
            self.sides[2].faces[0],
            self.sides[2].faces[1],
            self.sides[2].faces[2],
            self.sides[5].faces[0],
            self.sides[5].faces[1],
            self.sides[5].faces[2],
            //middle cubes, second row
            self.sides[4].faces[3],
            self.sides[4].faces[4],
            self.sides[4].faces[5],
            self.sides[0].faces[3],
            self.sides[0].faces[4],
            self.sides[0].faces[5],
            self.sides[2].faces[3],
            self.sides[2].faces[4],
            self.sides[2].faces[5],
            self.sides[5].faces[3],
            self.sides[5].faces[4],
            self.sides[5].faces[5],
            //middle cubes, third row
            self.sides[4].faces[6],
            self.sides[4].faces[7],
            self.sides[4].faces[8],
            self.sides[0].faces[6],
            self.sides[0].faces[7],
            self.sides[0].faces[8],
            self.sides[2].faces[6],
            self.sides[2].faces[7],
            self.sides[2].faces[8],
            self.sides[5].faces[6],
            self.sides[5].faces[7],
            self.sides[5].faces[8],
            //bottom cube
            self.sides[3].faces[0],
            self.sides[3].faces[1],
            self.sides[3].faces[2],
            self.sides[3].faces[3],
            self.sides[3].faces[4],
            self.sides[3].faces[5],
            self.sides[3].faces[6],
            self.sides[3].faces[7],
            self.sides[3].faces[8]
        )
    }
    fn _print_moves(&self) -> () {
        println!(
            "
        Num moves: {}
        Move list: {:?}",
            self.num_moves, self.previous_moves
        );
    }
    fn scramble_cube(&self, n: u16) -> Cube {
        //Applies n random moves to cube.
        let mut new_cube = self.copy_cube();
        for _ in 0..n {
            //Loop runs n times. Underscore for variable name means we don't use the index for anything.
            let rng = thread_rng().gen_range(0, 12); //Random int from 0 to 11
            let rotated_cube = match rng {
                0 => new_cube.rotate_bottom_clockwise(),
                1 => new_cube.rotate_bottom_counter_clockwise(),
                2 => new_cube.rotate_down_clockwise(),
                3 => new_cube.rotate_down_counter_clockwise(),
                4 => new_cube.rotate_facing_clockwise(),
                5 => new_cube.rotate_facing_counter_clockwise(),
                6 => new_cube.rotate_left_clockwise(),
                7 => new_cube.rotate_left_counter_clockwise(),
                8 => new_cube.rotate_right_clockwise(),
                9 => new_cube.rotate_right_counter_clockwise(),
                10 => new_cube.rotate_up_clockwise(),
                11 => new_cube.rotate_up_counter_clockwise(),
                _ => panic!("RNG ran out of bounds!"),
            };
            new_cube = rotated_cube;
        }
        new_cube
    }
    fn forget_moves(&self) -> Cube {
        //Blanks the list of previous moves, keeps the state of the cube intact.
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves = vec![];
        new_cube.num_moves = 0;
        new_cube
    }
    fn rotate_facing_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("F".to_string());
        new_cube.num_moves += 1;
        //rotate top side
        new_cube.sides[0].faces[0] = self.sides[0].faces[6];
        new_cube.sides[0].faces[1] = self.sides[0].faces[3];
        new_cube.sides[0].faces[2] = self.sides[0].faces[0];
        new_cube.sides[0].faces[3] = self.sides[0].faces[7];
        new_cube.sides[0].faces[5] = self.sides[0].faces[1];
        new_cube.sides[0].faces[6] = self.sides[0].faces[8];
        new_cube.sides[0].faces[7] = self.sides[0].faces[5];
        new_cube.sides[0].faces[8] = self.sides[0].faces[2];
        //rotate left side, top row
        new_cube.sides[4].faces[2] = self.sides[3].faces[0];
        new_cube.sides[4].faces[5] = self.sides[3].faces[1];
        new_cube.sides[4].faces[8] = self.sides[3].faces[2];
        //rotate Up side, top row
        new_cube.sides[1].faces[6] = self.sides[4].faces[2];
        new_cube.sides[1].faces[7] = self.sides[4].faces[5];
        new_cube.sides[1].faces[8] = self.sides[4].faces[8];
        //rotate Right side, top row
        new_cube.sides[2].faces[0] = self.sides[1].faces[6];
        new_cube.sides[2].faces[3] = self.sides[1].faces[7];
        new_cube.sides[2].faces[6] = self.sides[1].faces[8];
        //rotate Down side, top row
        new_cube.sides[3].faces[0] = self.sides[2].faces[0];
        new_cube.sides[3].faces[1] = self.sides[2].faces[3];
        new_cube.sides[3].faces[2] = self.sides[2].faces[6];
        new_cube
    }
    fn rotate_facing_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("F`".to_string());
        new_cube.num_moves += 1;
        //rotate Facing side
        new_cube.sides[0].faces[6] = self.sides[0].faces[0];
        new_cube.sides[0].faces[3] = self.sides[0].faces[1];
        new_cube.sides[0].faces[0] = self.sides[0].faces[2];
        new_cube.sides[0].faces[7] = self.sides[0].faces[3];
        new_cube.sides[0].faces[1] = self.sides[0].faces[5];
        new_cube.sides[0].faces[8] = self.sides[0].faces[6];
        new_cube.sides[0].faces[5] = self.sides[0].faces[7];
        new_cube.sides[0].faces[2] = self.sides[0].faces[8];
        //rotate Down side, top row
        new_cube.sides[3].faces[0] = self.sides[4].faces[2];
        new_cube.sides[3].faces[1] = self.sides[4].faces[5];
        new_cube.sides[3].faces[2] = self.sides[4].faces[8];
        //rotate Left side, top row
        new_cube.sides[4].faces[2] = self.sides[1].faces[6];
        new_cube.sides[4].faces[5] = self.sides[1].faces[7];
        new_cube.sides[4].faces[8] = self.sides[1].faces[8];
        //rotate Up side, top row
        new_cube.sides[1].faces[6] = self.sides[2].faces[0];
        new_cube.sides[1].faces[7] = self.sides[2].faces[3];
        new_cube.sides[1].faces[8] = self.sides[2].faces[6];
        //rotate Right side, top row
        new_cube.sides[2].faces[0] = self.sides[3].faces[0];
        new_cube.sides[2].faces[3] = self.sides[3].faces[1];
        new_cube.sides[2].faces[6] = self.sides[3].faces[2];
        new_cube
    }
    fn rotate_up_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("U".to_string());
        new_cube.num_moves += 1;
        //rotate up side
        new_cube.sides[1].faces[0] = self.sides[1].faces[6];
        new_cube.sides[1].faces[1] = self.sides[1].faces[3];
        new_cube.sides[1].faces[2] = self.sides[1].faces[0];
        new_cube.sides[1].faces[3] = self.sides[1].faces[7];
        new_cube.sides[1].faces[5] = self.sides[1].faces[1];
        new_cube.sides[1].faces[6] = self.sides[1].faces[8];
        new_cube.sides[1].faces[7] = self.sides[1].faces[5];
        new_cube.sides[1].faces[8] = self.sides[1].faces[2];
        //rotate left side
        new_cube.sides[4].faces[0] = self.sides[0].faces[0];
        new_cube.sides[4].faces[1] = self.sides[0].faces[1];
        new_cube.sides[4].faces[2] = self.sides[0].faces[2];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[4].faces[0];
        new_cube.sides[5].faces[1] = self.sides[4].faces[1];
        new_cube.sides[5].faces[2] = self.sides[4].faces[2];
        //rotate Right side
        new_cube.sides[2].faces[0] = self.sides[5].faces[0];
        new_cube.sides[2].faces[1] = self.sides[5].faces[1];
        new_cube.sides[2].faces[2] = self.sides[5].faces[2];
        //rotate Down side
        new_cube.sides[0].faces[0] = self.sides[2].faces[0];
        new_cube.sides[0].faces[1] = self.sides[2].faces[1];
        new_cube.sides[0].faces[2] = self.sides[2].faces[2];
        new_cube
    }
    fn rotate_up_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("U`".to_string());
        new_cube.num_moves += 1;

        //rotate up side
        new_cube.sides[1].faces[6] = self.sides[1].faces[0];
        new_cube.sides[1].faces[3] = self.sides[1].faces[1];
        new_cube.sides[1].faces[0] = self.sides[1].faces[2];
        new_cube.sides[1].faces[7] = self.sides[1].faces[3];
        new_cube.sides[1].faces[1] = self.sides[1].faces[5];
        new_cube.sides[1].faces[8] = self.sides[1].faces[6];
        new_cube.sides[1].faces[5] = self.sides[1].faces[7];
        new_cube.sides[1].faces[2] = self.sides[1].faces[8];
        //rotate top side
        new_cube.sides[0].faces[0] = self.sides[4].faces[0];
        new_cube.sides[0].faces[1] = self.sides[4].faces[1];
        new_cube.sides[0].faces[2] = self.sides[4].faces[2];
        //rotate left side
        new_cube.sides[4].faces[0] = self.sides[5].faces[0];
        new_cube.sides[4].faces[1] = self.sides[5].faces[1];
        new_cube.sides[4].faces[2] = self.sides[5].faces[2];
        //rotate Bottom side
        new_cube.sides[5].faces[0] = self.sides[2].faces[0];
        new_cube.sides[5].faces[1] = self.sides[2].faces[1];
        new_cube.sides[5].faces[2] = self.sides[2].faces[2];
        //rotate right side
        new_cube.sides[2].faces[0] = self.sides[0].faces[0];
        new_cube.sides[2].faces[1] = self.sides[0].faces[1];
        new_cube.sides[2].faces[2] = self.sides[0].faces[2];
        new_cube
    }
    fn rotate_right_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("R".to_string());
        new_cube.num_moves += 1;
        //rotate right side
        new_cube.sides[2].faces[0] = self.sides[2].faces[6];
        new_cube.sides[2].faces[1] = self.sides[2].faces[3];
        new_cube.sides[2].faces[2] = self.sides[2].faces[0];
        new_cube.sides[2].faces[3] = self.sides[2].faces[7];
        new_cube.sides[2].faces[5] = self.sides[2].faces[1];
        new_cube.sides[2].faces[6] = self.sides[2].faces[8];
        new_cube.sides[2].faces[7] = self.sides[2].faces[5];
        new_cube.sides[2].faces[8] = self.sides[2].faces[2];
        //rotate Down side
        new_cube.sides[3].faces[2] = self.sides[5].faces[0];
        new_cube.sides[3].faces[5] = self.sides[5].faces[3];
        new_cube.sides[3].faces[8] = self.sides[5].faces[6];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[1].faces[2];
        new_cube.sides[5].faces[3] = self.sides[1].faces[5];
        new_cube.sides[5].faces[6] = self.sides[1].faces[8];
        //rotate Front side
        new_cube.sides[0].faces[2] = self.sides[3].faces[2];
        new_cube.sides[0].faces[5] = self.sides[3].faces[5];
        new_cube.sides[0].faces[8] = self.sides[3].faces[8];
        //rotate up side
        new_cube.sides[1].faces[2] = self.sides[0].faces[2];
        new_cube.sides[1].faces[5] = self.sides[0].faces[5];
        new_cube.sides[1].faces[8] = self.sides[0].faces[8];
        new_cube
    }
    fn rotate_right_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("R`".to_string());
        new_cube.num_moves += 1;
        //rotate right side
        new_cube.sides[2].faces[6] = self.sides[2].faces[0];
        new_cube.sides[2].faces[3] = self.sides[2].faces[1];
        new_cube.sides[2].faces[0] = self.sides[2].faces[2];
        new_cube.sides[2].faces[7] = self.sides[2].faces[3];
        new_cube.sides[2].faces[1] = self.sides[2].faces[5];
        new_cube.sides[2].faces[8] = self.sides[2].faces[6];
        new_cube.sides[2].faces[5] = self.sides[2].faces[7];
        new_cube.sides[2].faces[2] = self.sides[2].faces[8];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[3].faces[2];
        new_cube.sides[5].faces[3] = self.sides[3].faces[5];
        new_cube.sides[5].faces[6] = self.sides[3].faces[8];
        //rotate up side
        new_cube.sides[1].faces[2] = self.sides[5].faces[0];
        new_cube.sides[1].faces[5] = self.sides[5].faces[3];
        new_cube.sides[1].faces[8] = self.sides[5].faces[6];
        //rotate Down side
        new_cube.sides[3].faces[2] = self.sides[0].faces[2];
        new_cube.sides[3].faces[5] = self.sides[0].faces[5];
        new_cube.sides[3].faces[8] = self.sides[0].faces[8];
        //rotate Front side
        new_cube.sides[0].faces[2] = self.sides[1].faces[2];
        new_cube.sides[0].faces[5] = self.sides[1].faces[5];
        new_cube.sides[0].faces[8] = self.sides[1].faces[8];
        new_cube
    }
    fn rotate_down_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("D".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[3].faces[0] = self.sides[3].faces[6];
        new_cube.sides[3].faces[1] = self.sides[3].faces[3];
        new_cube.sides[3].faces[2] = self.sides[3].faces[0];
        new_cube.sides[3].faces[3] = self.sides[3].faces[7];
        new_cube.sides[3].faces[5] = self.sides[3].faces[1];
        new_cube.sides[3].faces[6] = self.sides[3].faces[8];
        new_cube.sides[3].faces[7] = self.sides[3].faces[5];
        new_cube.sides[3].faces[8] = self.sides[3].faces[2];
        //rotate Front side
        new_cube.sides[0].faces[6] = self.sides[4].faces[6];
        new_cube.sides[0].faces[7] = self.sides[4].faces[7];
        new_cube.sides[0].faces[8] = self.sides[4].faces[8];
        //rotate Right side
        new_cube.sides[2].faces[6] = self.sides[0].faces[6];
        new_cube.sides[2].faces[7] = self.sides[0].faces[7];
        new_cube.sides[2].faces[8] = self.sides[0].faces[8];
        //rotate Bottom side
        new_cube.sides[5].faces[6] = self.sides[2].faces[6];
        new_cube.sides[5].faces[7] = self.sides[2].faces[7];
        new_cube.sides[5].faces[8] = self.sides[2].faces[8];
        //rotate left side
        new_cube.sides[4].faces[6] = self.sides[5].faces[6];
        new_cube.sides[4].faces[7] = self.sides[5].faces[7];
        new_cube.sides[4].faces[8] = self.sides[5].faces[8];
        new_cube
    }
    fn rotate_down_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("D`".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[3].faces[6] = self.sides[3].faces[0];
        new_cube.sides[3].faces[3] = self.sides[3].faces[1];
        new_cube.sides[3].faces[0] = self.sides[3].faces[2];
        new_cube.sides[3].faces[7] = self.sides[3].faces[3];
        new_cube.sides[3].faces[1] = self.sides[3].faces[5];
        new_cube.sides[3].faces[8] = self.sides[3].faces[6];
        new_cube.sides[3].faces[5] = self.sides[3].faces[7];
        new_cube.sides[3].faces[2] = self.sides[3].faces[8];
        //rotate Front side
        new_cube.sides[4].faces[6] = self.sides[0].faces[6];
        new_cube.sides[4].faces[7] = self.sides[0].faces[7];
        new_cube.sides[4].faces[8] = self.sides[0].faces[8];
        //rotate Right side
        new_cube.sides[0].faces[6] = self.sides[2].faces[6];
        new_cube.sides[0].faces[7] = self.sides[2].faces[7];
        new_cube.sides[0].faces[8] = self.sides[2].faces[8];
        //rotate Bottom side
        new_cube.sides[2].faces[6] = self.sides[5].faces[6];
        new_cube.sides[2].faces[7] = self.sides[5].faces[7];
        new_cube.sides[2].faces[8] = self.sides[5].faces[8];
        //rotate left side
        new_cube.sides[5].faces[6] = self.sides[4].faces[6];
        new_cube.sides[5].faces[7] = self.sides[4].faces[7];
        new_cube.sides[5].faces[8] = self.sides[4].faces[8];
        new_cube
    }
    fn rotate_left_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("L".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[4].faces[0] = self.sides[4].faces[6];
        new_cube.sides[4].faces[1] = self.sides[4].faces[3];
        new_cube.sides[4].faces[2] = self.sides[4].faces[0];
        new_cube.sides[4].faces[3] = self.sides[4].faces[7];
        new_cube.sides[4].faces[5] = self.sides[4].faces[1];
        new_cube.sides[4].faces[6] = self.sides[4].faces[8];
        new_cube.sides[4].faces[7] = self.sides[4].faces[5];
        new_cube.sides[4].faces[8] = self.sides[4].faces[2];
        //rotate Front side
        new_cube.sides[0].faces[0] = self.sides[1].faces[0];
        new_cube.sides[0].faces[3] = self.sides[1].faces[3];
        new_cube.sides[0].faces[6] = self.sides[1].faces[6];
        //rotate Down side
        new_cube.sides[3].faces[0] = self.sides[0].faces[0];
        new_cube.sides[3].faces[3] = self.sides[0].faces[3];
        new_cube.sides[3].faces[6] = self.sides[0].faces[6];
        //rotate Up side
        new_cube.sides[1].faces[0] = self.sides[5].faces[2];
        new_cube.sides[1].faces[3] = self.sides[5].faces[5];
        new_cube.sides[1].faces[6] = self.sides[5].faces[8];
        //rotate Bottom side
        new_cube.sides[5].faces[2] = self.sides[3].faces[0];
        new_cube.sides[5].faces[5] = self.sides[3].faces[3];
        new_cube.sides[5].faces[8] = self.sides[3].faces[6];
        new_cube
    }
    fn rotate_left_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("L`".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[4].faces[6] = self.sides[4].faces[0];
        new_cube.sides[4].faces[3] = self.sides[4].faces[1];
        new_cube.sides[4].faces[0] = self.sides[4].faces[2];
        new_cube.sides[4].faces[7] = self.sides[4].faces[3];
        new_cube.sides[4].faces[1] = self.sides[4].faces[5];
        new_cube.sides[4].faces[8] = self.sides[4].faces[6];
        new_cube.sides[4].faces[5] = self.sides[4].faces[7];
        new_cube.sides[4].faces[2] = self.sides[4].faces[8];
        //rotate Front side
        new_cube.sides[1].faces[0] = self.sides[0].faces[0];
        new_cube.sides[1].faces[3] = self.sides[0].faces[3];
        new_cube.sides[1].faces[6] = self.sides[0].faces[6];
        //rotate Down side
        new_cube.sides[0].faces[0] = self.sides[3].faces[0];
        new_cube.sides[0].faces[3] = self.sides[3].faces[3];
        new_cube.sides[0].faces[6] = self.sides[3].faces[6];
        //rotate Up side
        new_cube.sides[5].faces[2] = self.sides[1].faces[0];
        new_cube.sides[5].faces[5] = self.sides[1].faces[3];
        new_cube.sides[5].faces[8] = self.sides[1].faces[6];
        //rotate Bottom side
        new_cube.sides[3].faces[0] = self.sides[5].faces[2];
        new_cube.sides[3].faces[3] = self.sides[5].faces[5];
        new_cube.sides[3].faces[6] = self.sides[5].faces[8];
        new_cube
    }
    fn rotate_bottom_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("B".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[5].faces[0] = self.sides[5].faces[6];
        new_cube.sides[5].faces[1] = self.sides[5].faces[3];
        new_cube.sides[5].faces[2] = self.sides[5].faces[0];
        new_cube.sides[5].faces[3] = self.sides[5].faces[7];
        new_cube.sides[5].faces[5] = self.sides[5].faces[1];
        new_cube.sides[5].faces[6] = self.sides[5].faces[8];
        new_cube.sides[5].faces[7] = self.sides[5].faces[5];
        new_cube.sides[5].faces[8] = self.sides[5].faces[2];
        //rotate Up side
        new_cube.sides[1].faces[0] = self.sides[4].faces[0];
        new_cube.sides[1].faces[1] = self.sides[4].faces[3];
        new_cube.sides[1].faces[2] = self.sides[4].faces[6];
        //rotate Right side
        new_cube.sides[2].faces[2] = self.sides[1].faces[0];
        new_cube.sides[2].faces[5] = self.sides[1].faces[1];
        new_cube.sides[2].faces[8] = self.sides[1].faces[2];
        //rotate Up side
        new_cube.sides[3].faces[6] = self.sides[2].faces[2];
        new_cube.sides[3].faces[7] = self.sides[2].faces[5];
        new_cube.sides[3].faces[8] = self.sides[2].faces[8];
        //rotate Left side
        new_cube.sides[4].faces[0] = self.sides[3].faces[6];
        new_cube.sides[4].faces[3] = self.sides[3].faces[7];
        new_cube.sides[4].faces[6] = self.sides[3].faces[8];
        new_cube
    }
    fn rotate_bottom_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push("B`".to_string());
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[5].faces[6] = self.sides[5].faces[0];
        new_cube.sides[5].faces[3] = self.sides[5].faces[1];
        new_cube.sides[5].faces[0] = self.sides[5].faces[2];
        new_cube.sides[5].faces[7] = self.sides[5].faces[3];
        new_cube.sides[5].faces[1] = self.sides[5].faces[5];
        new_cube.sides[5].faces[8] = self.sides[5].faces[6];
        new_cube.sides[5].faces[5] = self.sides[5].faces[7];
        new_cube.sides[5].faces[2] = self.sides[5].faces[8];
        //rotate Up side
        new_cube.sides[4].faces[0] = self.sides[1].faces[0];
        new_cube.sides[4].faces[3] = self.sides[1].faces[1];
        new_cube.sides[4].faces[6] = self.sides[1].faces[2];
        //rotate Right side
        new_cube.sides[1].faces[0] = self.sides[2].faces[2];
        new_cube.sides[1].faces[1] = self.sides[2].faces[5];
        new_cube.sides[1].faces[2] = self.sides[2].faces[8];
        //rotate Up side
        new_cube.sides[2].faces[2] = self.sides[3].faces[6];
        new_cube.sides[2].faces[5] = self.sides[3].faces[7];
        new_cube.sides[2].faces[8] = self.sides[3].faces[8];
        //rotate Left side
        new_cube.sides[3].faces[6] = self.sides[4].faces[0];
        new_cube.sides[3].faces[7] = self.sides[4].faces[3];
        new_cube.sides[3].faces[8] = self.sides[4].faces[6];
        new_cube
    }
}

fn build_side(colour: char) -> Side {
    //Builds a side of all the same colour faces
    Side {
        faces: vec![colour; 9],
    }
}
fn build_cube() -> Cube {
    //Builds a solved cube
    Cube {
        sides: vec![
            build_side('R'),
            build_side('W'),
            build_side('B'),
            build_side('Y'),
            build_side('G'),
            build_side('O'),
        ],
        previous_moves: vec![],
        num_moves: 0,
    }
}

#[test]
fn test_sides() {
    let side1 = build_side('W');
    let side2 = Side {
        faces: vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'W'],
    };
    assert_eq!(side1.is_solved(), true);
    assert_eq!(side2.is_solved(), false);
}
#[test]
fn test_cubes() {
    let cube1 = build_cube();
    assert_eq!(cube1.is_solved(), true);

    let cube1 = cube1.rotate_down_clockwise();
    assert_eq!(cube1.is_solved(), false);

    let mut cube2 = build_cube();
    cube2.sides[0].faces[0] = 'B';
    assert_eq!(cube2.is_solved(), false);

    let cube3 = build_cube() //rotate one way, and rotate back. Should result in no change.
        .rotate_facing_clockwise()
        .rotate_facing_counter_clockwise()
        .rotate_up_clockwise()
        .rotate_up_counter_clockwise()
        .rotate_right_clockwise()
        .rotate_right_counter_clockwise()
        .rotate_down_clockwise()
        .rotate_down_counter_clockwise()
        .rotate_left_clockwise()
        .rotate_left_counter_clockwise()
        .rotate_bottom_clockwise()
        .rotate_bottom_counter_clockwise();
    assert_eq!(cube3.is_solved(), true);
    assert_eq!(cube3.num_moves, 12);

    let cube4 = build_cube() //Four rotations in same direction should result in no change.
        .rotate_facing_clockwise()
        .rotate_facing_clockwise()
        .rotate_facing_clockwise()
        .rotate_facing_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_up_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_right_clockwise()
        .rotate_bottom_clockwise()
        .rotate_bottom_clockwise()
        .rotate_bottom_clockwise()
        .rotate_bottom_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_left_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise()
        .rotate_down_clockwise();
    assert_eq!(cube4.is_solved(), true);
    assert_eq!(cube4.num_moves, 24);
}
