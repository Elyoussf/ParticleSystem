pub struct Container {
    pub matrix: Vec<Vec<String>>,
}

impl Container {
    pub fn new() -> Self {
        let mut matrix = vec![vec![" ".to_string(); 30]; 25]; // this matrix will be accessed by every particle so
                                                              // so we are not gonna define it here and it should be thread - safe

        for j in 0..30 {
            matrix[0][j] = "#".to_string();
        }
        for i in 1..25 {
            matrix[i][0] = "#".to_string();
            matrix[i][29] = "#".to_string();
        }
        for j in 0..30 {
            matrix[24][j] = "#".to_string();
        }
        Container { matrix }
    }

    pub fn render(&self) {
        // while c < 20{
        for i in 0..25 {
            for j in 0..30 {
                print!("{} ", self.matrix[i][j]);
            }
            println!();
        }

        // }
    }
}
