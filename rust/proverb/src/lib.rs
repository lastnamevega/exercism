pub fn build_proverb(list: &[&str]) -> String {
    let mut lines: Vec<String> = vec![];

    for index in 0..list.len() {
        let next_index = index + 1;

        if next_index == list.len() {
            let original_noun = list[0];
            let line = format!("And all for the want of a {}.", original_noun);

            lines.push(line);
        } else {
            let first_noun = list[index];
            let second_noun = list[next_index];
            let line = format!("For want of a {} the {} was lost.", first_noun, second_noun);

            lines.push(line);
        }
    }

    lines.join("\n")
}
