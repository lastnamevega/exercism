fn bottle_phrase(num_bottles: i32) -> String {
    match num_bottles {
        -1 => "99 bottles of beer".to_string(),
        0 => "No more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        _ => format!("{} bottles of beer", num_bottles),
    }
}

fn verb_phrase(num_bottles: i32) -> String {
    match num_bottles {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string(),
    }
}

fn first_line(verse_num: i32) -> String {
    let phrase = bottle_phrase(verse_num);

    format!("{} on the wall, {}.\n", phrase, phrase.to_lowercase())
}

fn second_line(verse_num: i32) -> String {
    let phrase = bottle_phrase(verse_num - 1);

    format!(
        "{}, {} on the wall.\n",
        verb_phrase(verse_num),
        phrase.to_lowercase()
    )
}

pub fn verse(n: i32) -> String {
    first_line(n) + &second_line(n)
}

pub fn sing(start: i32, end: i32) -> String {
    let mut verses: Vec<String> = vec![];

    for i in (end..=start).rev() {
        verses.push(verse(i));
    }

    verses.join("\n")
}
