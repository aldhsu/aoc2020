fn part1(input: &mut Vec<u8>) -> String {
    let mut current_index = 0;

    for i in 0..100 {
        let current_label = input[current_index].clone();
        let mut remove_index = [
            (current_index + 1) % input.len(),
            (current_index + 2) % input.len(),
            (current_index + 3) % input.len(),
        ];

        let removed = [
            input[remove_index[0]],
            input[remove_index[1]],
            input[remove_index[2]],
        ];
        remove_index.sort();

        for i in remove_index.iter().rev() {
            input.remove(*i);
        }

        let mut next_label = current_label - 1;
        loop {
            match next_label {
                label if removed.iter().find(|&&removed| removed == label).is_some() => {
                    next_label -= 1
                }
                negative_label if negative_label < 1 => next_label = 9,
                _ => break,
            }
        }

        let mut insert_index = input
            .iter()
            .position(|&i| i == next_label)
            .expect(&format!("coudln't find label {}", next_label))
            + 1;
        for i in removed.iter() {
            input.insert(insert_index, *i);
            insert_index += 1;
        }

        current_index = (input
            .iter()
            .position(|&i| i == current_label)
            .expect("coudln't find label")
            + 1)
            % input.len();
    }

    let position_1 = input.iter().position(|&i| i == 1).expect("couldn't find 1");
    input.rotate_left(position_1);
    input
        .iter()
        .skip(1)
        .map(|i| i.to_string())
        .collect::<String>()
}

fn chase_pointers(vec: &[usize]) -> String {
    let mut string = String::new();
    let mut pos = vec[0];
    loop {
        string += &(pos + 1).to_string();
        pos = vec[pos];
        if pos == 0 {
            break;
        }
    }
    string
}

fn get_vec(input: &str, total_size: usize, iterations: usize) -> Vec<usize> {
    let mut vec = vec![0_usize; total_size];
    let initial = input
        .chars()
        .map(|i| (i.to_digit(10).unwrap() - 1) as usize)
        .chain(input.len()..total_size)
        .collect::<Vec<usize>>();

    // point to next
    for (pos, pointer) in initial.iter().zip(initial.iter().cycle().skip(1)) {
        vec[*pos] = *pointer;
    }

    let mut starting_pos = initial[0];

    for i in 0..iterations {
        let first_pointer = vec[starting_pos];
        let second_pointer = vec[first_pointer];
        let third_pointer = vec[second_pointer];
        let fourth_pointer = vec[third_pointer];

        // removing 3 cups
        // move pointer at current to pointer at third
        vec[starting_pos] = fourth_pointer;

        let new_label = {
            let mut temp = starting_pos as isize - 1;
            if temp == -1 {
                temp = (vec.len() - 1) as isize;
            }

            while let Some(_) = [first_pointer, second_pointer, third_pointer]
                .iter()
                .find(|&&label| label as isize == temp)
            {
                temp -= 1;
                if temp == -1 {
                    temp = (vec.len() - 1) as isize;
                }
            }
            temp as usize
        };
        // inserting 3 cups
        // save new label pointer
        let new_end = vec[new_label].clone();
        // move pointer at new label to point to first
        vec[new_label] = first_pointer;
        // move at third to point to what new label was pointing at
        vec[third_pointer] = new_end;
        starting_pos = fourth_pointer;
    }

    vec
}

fn part2(input: &str) -> usize {
    let vec = get_vec(&input, 1_000_000, 10_000_000);
    let first_cup = vec[0];
    let second_cup = vec[first_cup];

    (first_cup + 1) * (second_cup + 1)
}

fn main() {
    let mut input = "653427918"
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).expect("couldn't parse"))
        .collect::<Vec<u8>>();
    let part1 = part1(&mut input);

    println!("part1: {}", part1);

    let mut input = "653427918";
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn it_works() {
        let mut input = "389125467"
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as u8).expect("couldn't parse"))
            .collect::<Vec<u8>>();

        assert_eq!(part1(&mut input), "76952348");
    }

    #[test]
    fn part2_works() {
        let input = "389125467";
        assert_eq!(part2(&input), 149245887792);
    }
}
