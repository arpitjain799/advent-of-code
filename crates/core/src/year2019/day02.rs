use super::int_code::Program;
use super::int_code::Word;

pub fn part1(input_string: &str) -> Result<Word, String> {
    part1_patch(input_string, true)
}

pub fn part1_patch(input_string: &str, patch: bool) -> Result<Word, String> {
    let mut program = Program::parse(input_string)?;

    if patch {
        // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
        program.write_memory(1, 12);
        program.write_memory(2, 2);
    }

    Ok(program.run_for_register0())
}

pub fn part2(input_string: &str) -> Result<Word, String> {
    let initial_program = Program::parse(input_string)?;
    const DESIRED_OUTPUT: Word = 19_690_720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = initial_program.clone();
            program.write_memory(1, noun);
            program.write_memory(2, verb);
            if program.run_for_register0() == DESIRED_OUTPUT {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(format!(
        "Desired output {} is never produced",
        DESIRED_OUTPUT
    ))
}

#[test]
pub fn tests_part1() {
    assert_eq!(
        Ok(3500),
        part1_patch("1,9,10,3,2,3,11,0,99,30,40,50", false)
    );
    assert_eq!(Ok(2), part1_patch("1,0,0,0,99", false));
    assert_eq!(Ok(2), part1_patch("2,3,0,3,99", false));
    assert_eq!(Ok(2), part1_patch("2,4,4,5,99,0", false));
    assert_eq!(Ok(30), part1_patch("1,1,1,4,99,5,6,0,99", false));

    assert_eq!(Ok(4570637), part1(include_str!("day02_input.txt")));

    assert_eq!(
        Err("Unable to parse program word hi: invalid digit found in string".to_string()),
        part1("hi")
    );
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(5485), part2(include_str!("day02_input.txt")));
}
