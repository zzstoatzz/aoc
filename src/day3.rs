use std::fs::read_to_string;

fn is_valid_mul_instruction(s: &str) -> Option<(i32, i32)> {
    // Check if it starts with exactly "mul("
    if !s.starts_with("mul(") {
        return None;
    }

    // Split the content between parentheses by comma
    let content = s.strip_prefix("mul(")?.strip_suffix(')')?;
    let mut parts = content.split(',');

    // Get both numbers
    let x = parts.next()?.parse::<i32>().ok()?;
    let y = parts.next()?.parse::<i32>().ok()?;

    // Ensure there's nothing else after the numbers
    if parts.next().is_some() {
        return None;
    }

    // Check if numbers are 1-3 digits
    if x > 999 || y > 999 || x < 1 || y < 1 {
        return None;
    }

    Some((x, y))
}

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn find_instructions(content: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < content.len() {
        if content[i..].starts_with("mul(") {
            if let Some(end) = content[i..].find(')') {
                let instruction = &content[i..=i + end];
                if let Some((x, y)) = is_valid_mul_instruction(instruction) {
                    instructions.push(Instruction::Mul(x, y));
                }
            }
        } else if content[i..].starts_with("do()") {
            instructions.push(Instruction::Do);
        } else if content[i..].starts_with("don't()") {
            instructions.push(Instruction::Dont);
        }
        i += 1;
    }

    instructions
}

pub fn part1(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");

    find_instructions(&content)
        .iter()
        .filter_map(|instr| {
            if let Instruction::Mul(x, y) = instr {
                Some(x * y)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let instructions = find_instructions(&content);

    let mut sum = 0;
    let mut enabled = true;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(x, y) if enabled => {
                sum += x * y;
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            _ => {}
        }
    }

    sum
}
