fn main() {
    let cube1 = build_cube();
    let mut cube2 = cube1.copy_cube();
    cube2.sides[0].faces[0] = 'O';
    let cube3 = cube2.rotate_down_counter_clockwise();

    cube3.print_cube();
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
    fn to_string(&self) -> String {
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
}

impl Cube {
    fn copy_cube(&self) -> Cube {
        let old_sides = &self.sides;
        Cube {
            sides: vec![
                self.sides[0].copy_side(),
                self.sides[1].copy_side(),
                self.sides[2].copy_side(),
                self.sides[3].copy_side(),
                self.sides[4].copy_side(),
                self.sides[5].copy_side(),
            ],
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

    fn rotate_facing_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
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
        //rotate Facing side
        new_cube.sides[0].faces[6] = self.sides[0].faces[0];
        new_cube.sides[0].faces[3] = self.sides[0].faces[1];
        new_cube.sides[0].faces[0] = self.sides[0].faces[2];
        new_cube.sides[0].faces[7] = self.sides[0].faces[3];
        new_cube.sides[0].faces[1] = self.sides[0].faces[4];
        new_cube.sides[0].faces[8] = self.sides[0].faces[5];
        new_cube.sides[0].faces[5] = self.sides[0].faces[6];
        new_cube.sides[0].faces[2] = self.sides[0].faces[7];
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

    let mut cube2 = build_cube();
    cube2.sides[0].faces[0] = 'B';
    assert_eq!(cube2.is_solved(), false);

    let cube3 = build_cube()
        .rotate_facing_clockwise()
        .rotate_facing_counter_clockwise()
        .rotate_up_clockwise()
        .rotate_up_counter_clockwise()
        .rotate_right_clockwise()
        .rotate_right_counter_clockwise();
    assert_eq!(cube3.is_solved(), true); //rotate one way, and rotate back.

    let cube4 = build_cube()
        .rotate_facing_clockwise()
        .rotate_facing_clockwise()
        .rotate_facing_clockwise()
        .rotate_facing_clockwise();
    assert_eq!(cube4.is_solved(), true); //Four rotations in same direction
}
