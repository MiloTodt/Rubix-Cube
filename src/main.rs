extern crate rand;
mod cube;

use cube::Cube;
use cube::Side;
use std::thread;

static NUMBER_OF_MOVES: usize = 7; //Number of random moves applied to cube. Setting this higher will take a long time by a factor of 11^N
static mut SOLVE_DEPTH: usize = NUMBER_OF_MOVES + 1;

fn main() {
    let cube = build_cube().scramble_cube(NUMBER_OF_MOVES);

    cube.print_cube();
    println!(
        "Applied {} random moves to cube: {:?}",
        cube.num_moves, cube.previous_moves
    );

    let cube = cube.forget_moves(); //Blanks the list of previous moves, keeps the state of the cube faces.

    let handles: Vec<_> = starter_cubes(cube) //12 threads, based on the first 12 states of the cube.
        .into_iter()
        .map(|c| {
            thread::spawn(move || {
                solve_cube(c);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
fn starter_cubes(old_cube: Cube) -> Vec<Cube> {
    //The first 12 cubes for thread seed data.
    let in_cube = old_cube.copy_cube();
    let mut out_cubes = vec![];
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
    out_cubes
}
fn solve_cube(in_cube: Cube) -> () {
    let num_moves = in_cube.num_moves;
    let previous_moves = &in_cube.previous_moves;
    unsafe {
        if num_moves >= SOLVE_DEPTH {
            return;
        }
    }
    if in_cube.is_solved() {
        unsafe {
            if num_moves < SOLVE_DEPTH {
                SOLVE_DEPTH = num_moves;
            }
        }
        println!(
            "Found solution in {} moves:  {:?}",
            num_moves, previous_moves
        );
    } else {
        //If last two moves are opposites of one another, can not be minimal solution
        if num_moves >= 2 { 
            let second_last_move = &previous_moves[(num_moves - 2)].as_str();
            let last_move = &previous_moves.last().unwrap().as_str();

            if (last_move.to_string() == format!("{}{}", second_last_move, "`"))
                || (second_last_move.to_string() == format!("{}{}", last_move, "`"))
            {
                return;
            }
        }
        
        solve_cube(in_cube.rotate_bottom_clockwise());
        solve_cube(in_cube.rotate_bottom_counter_clockwise());
        solve_cube(in_cube.rotate_down_clockwise());
        solve_cube(in_cube.rotate_down_counter_clockwise());
        solve_cube(in_cube.rotate_left_clockwise());
        solve_cube(in_cube.rotate_left_counter_clockwise());
        solve_cube(in_cube.rotate_right_clockwise());
        solve_cube(in_cube.rotate_right_counter_clockwise());
        solve_cube(in_cube.rotate_up_clockwise());
        solve_cube(in_cube.rotate_up_counter_clockwise());
        solve_cube(in_cube.rotate_facing_clockwise());
        solve_cube(in_cube.rotate_facing_counter_clockwise());
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
fn build_side(colour: char) -> Side {
    //Builds a side of all the same colour faces
    Side {
        faces: vec![colour; 9],
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
