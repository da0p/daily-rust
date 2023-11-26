pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("{:#?}", minefield);
    let mut output = vec![];
    for (x, row) in minefield.iter().enumerate() {
        let mut line = String::new();
        for (y, col) in row.chars().enumerate() {
            if col == '*' {
                line.push_str("*");
            } else {
                let mines = count_adjacent_mines(minefield, x, y);
                if mines != 0 {
                    line.push_str(&mines.to_string());
                } else {
                    line.push_str(" ");
                }
            }
        }
        output.push(format!("{}", line));
    }
    output
}

pub fn count_adjacent_mines(minefield: &[&str], row: usize, col: usize) -> usize {
    let lower_r = match row > 0 {
        true => row - 1,
        _ => row,
    };

    let upper_r = match row < minefield.len() - 1 {
        true => row + 1,
        _ => row,
    };

    let lower_c = match col > 0 {
        true => col - 1,
        _ => col,
    };

    let upper_c = match col < minefield[0].len() - 1 {
        true => col + 1,
        _ => col,
    };

    let mut mines = 0;
    for r in lower_r..upper_r + 1 {
        for c in lower_c..upper_c + 1 {
            if (r, c) != (row, col) && minefield[r].chars().nth(c).unwrap() == '*' {
                mines += 1;
            }
        }
    }

    mines
}
