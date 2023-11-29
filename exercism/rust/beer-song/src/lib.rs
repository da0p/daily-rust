
pub fn verse(n: u32) -> String {
    let mut verse = String::new();
    if n > 1 {
        verse.push_str(&format!(
            "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around",
            n, n
        ));
        verse.push_str(&format!(", {} {} of beer on the wall.\n", n - 1, if n - 1 > 1 {"bottles"} else {"bottle"}));
    } else if n == 1 {
        verse.push_str(&format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"));
    } else {
        verse.push_str(&format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"));
    }

    verse
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for n in (end..start + 1).rev() {
        if n > end {
            song.push_str(&format!("{}\n", verse(n)));
        } else {
            song.push_str(&verse(n));
        }
    }
    song
}
