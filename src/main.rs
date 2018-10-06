
fn main() 
{
    let  side1 = build_side('W');
    println!("{}", side1.is_solved());

    let  side2 = Side { faces: vec!['B','B','B','B','B','B','B','B','W']};
    println!("{}", side2.is_solved());

    side2.print_side();

} 


struct Side {
    faces: Vec<char>,
}

impl Side {
    fn is_solved(&self) -> bool {
        let first = &self.faces[0];
        for face in &self.faces {
            if first != face {return false;}
        }
        true
    }
        fn print_side(&self) -> () {
        println!(
            "
            {}|{}|{}\n
            {}|{}|{}\n
            {}|{}|{}", self.faces[0], self.faces[1],self.faces[2],self.faces[3],self.faces[4],self.faces[5],self.faces[6],self.faces[7],self.faces[8]
        )
    }
}

fn build_side(colour: char) -> Side {
    Side{
        faces: vec![colour; 9],
    }
}

#[test]
fn test_sides() {
    let side1 = build_side('W');
    let side2 = Side { faces: vec!['B','B','B','B','B','B','B','B','W']};
    assert_eq!(side1.is_solved(), true);
    assert_eq!(side2.is_solved(), false);
}