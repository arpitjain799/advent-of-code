use crate::input::Input;

pub fn solve(input: &Input) -> Result<String, String> {
    const SIZE: usize = 256;
    let mut list: Vec<u8> = (0..SIZE).map(|i| i as u8).collect();

    let mut current_position = 0;
    let mut skip_size = 0;

    let input_vec = if input.is_part_one() {
        input
            .text
            .split(',')
            .map(|length| {
                length
                    .parse::<u8>()
                    .map_err(|e| format!("Invalid length: {e}"))
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let to_append = [17_u8, 31_u8, 73_u8, 47_u8, 23_u8];
        input
            .text
            .bytes()
            .chain(to_append.iter().copied())
            .collect()
    };

    for _round in 0..input.part_values(1, 64) {
        for &length in &input_vec {
            let length = length as usize;

            // "Reverse the order of that length of elements in the list, starting with the element at the current position."
            for i in 0..(length / 2) {
                list.swap(
                    (current_position + i) % SIZE,
                    (current_position + length - 1 - i) % SIZE,
                );
            }

            // "Move the current position forward by that length plus the skip size."
            current_position = (current_position + length + skip_size) % SIZE;

            // "Increase the skip size by one."
            skip_size += 1;
        }
    }

    Ok(if input.is_part_one() {
        (u32::from(list[0]) * u32::from(list[1])).to_string()
    } else {
        list.chunks(16)
            .map(|block| block.iter().fold(0, |acc, x| acc ^ x))
            .map(|number| format!("{number:02x}"))
            .collect()
    })
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => "62238".to_string());
    test_part_two!(real_input => "2b0c9cc0449507a0db3babd57ad9e8d8".to_string());
}
