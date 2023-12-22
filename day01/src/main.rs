use std::{io, char::from_digit};

fn part1() -> io::Result<()>
{
    let stdin = io::stdin();
    let mut line = String::new();
    let mut sum = 0;

    while stdin.read_line(&mut line)? > 0
    {
        let mut start_num = ' ';
        let mut end_num = ' ';
        for c in line.chars()
        {
            if c.is_ascii_digit() 
            {
                if start_num == ' '
                {
                    start_num = c;
                }

                end_num = c;
            }
        }

        if start_num.is_ascii_digit() && end_num.is_ascii_digit()
        {
            let number_str = format!("{}{}", start_num, end_num);
            let number = i32::from_str_radix(&number_str, 10).unwrap();
            sum += number;
        }

        line.clear();
    }

    println!("{}", sum);

    Ok(())
}

fn get_number(line : &str) -> Option<char>
{
    let number_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (j, number_str) in number_strings.iter().enumerate()
    {
        if line.starts_with(number_str)
        {
            return Some(from_digit(u32::try_from(j + 1).unwrap(), 10).unwrap());
        }
    }
    return None;
}


fn part2() -> io::Result<()>
{
    let stdin = io::stdin();
    let mut line = String::new();
    let mut sum = 0;
    
    while stdin.read_line(&mut line)? > 0
    {
        let mut start_num = ' ';
        let mut end_num = ' ';
        for i in 0..line.len()
        {
            // Check string names
            let number_from_name = get_number(&line[i..]);
            let mut character = line.chars().nth(i).unwrap();
            if number_from_name.is_some()
            {
                character = number_from_name.unwrap();
            }
            // Check int names
            if character.is_ascii_digit()
            {
                if start_num == ' '
                {
                    start_num = character;
                }

                end_num = character;
            }
        }

        if start_num.is_ascii_digit() && end_num.is_ascii_digit()
        {
            let number_str = format!("{}{}", start_num, end_num);
            let number = i32::from_str_radix(&number_str, 10).unwrap();
            sum += number;
        }

        line.clear();
    }

    println!("{}", sum);

    Ok(())
}

fn main()
{
    // let _ = part1();
    let _ = part2();
}
