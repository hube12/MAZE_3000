use java_random::Random;
use std::io::{stdout, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Colorize}, Result,
};
use core::fmt;
use crossterm::event::{read, Event, KeyCode};
use std::thread::sleep;
use std::time::Duration;
use std::env;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Rotation {
    LEFT,
    RIGHT,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rotation::LEFT => write!(f, "LEFT"),
            Rotation::RIGHT => write!(f, "RIGHT"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}


impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::NORTH => write!(f, "NORTH"),
            Direction::SOUTH => write!(f, "SOUTH"),
            Direction::EAST => write!(f, "EAST"),
            Direction::WEST => write!(f, "WEST"),
        }
    }
}

#[allow(dead_code)]
fn get_direction_boost(rotation: Rotation) -> i8 {
    if rotation == Rotation::RIGHT { 1 } else { -1 }
}

const SIZE_X: u16 = 100;
const SIZE_Y: u16 = 30;
const GRID: &'static [&'static str] = &[
    "1111111111111111111111111111111111111111111E111111111111111111111111111111111111111111111111111111111",
    "1000101000101000100010000000101000001000000010000010000010000000001000001010000000100000000000000000S",
    "10101010101010111110111011101010101011101110111010101110101111111111111010111110101011111110111011111",
    "10101000101010000000100010100000101010000010101011111010101010100010000000100000100000100010101010001",
    "10111110101011111010111110101010101110111010000000000000001010101111111010111011111011111011101110101",
    "10001000101010101010001000001010101000001010101010101111001010000010001010001010001010100010001000101",
    "11101111101010101011111110111011111010101110101111101011011010111011101010111010111010101111101011111",
    "10001000000000000000000010001010000010101010100000011001010010101010001010000000001000100010100000001",
    "10111110111110111010111110111011111111101010101111101011001011101010101111101110111011111010111010111",
    "10100000100010100010001010101000001000100010001000000000001000100000101011101111110000001000100010001",
    "10111111101110101110111010101111111011101011111110101111011011111011101010101010001011101110111011111",
    "10100010001010100010000000100000001000101000001000100001000010000010001010000010000000000000000010101",
    "10101110111011101011111011111111101110111110101010101011001110111111111010111111101011011011101110101",
    "10100000001000001000100010100000001010000010101010101001000100000000000011101000101001010000100000101",
    "10101111101111101111111010111111101010101111101011111111010101101011110110101110111001011110111110101",
    "10000010000010000010001000001010001000101000001010000000011100100010100000100010000011000010001000001",
    "11101111111010111010111011111011101011101110101111111110000001111010010011101110111001011010111111111",
    "10101000000010100000001000000010001010100000100010100000110000101110010110001000001011010000000000101",
    "10111111111011111010101110101111101010101011101110101111101011101010010110111111111111010111111010101",
    "10000010101010000010101000100000001010001000101000100000100010100000000110101010001011010000001010001",
    "10101110101011111110111110101110111111111111111110111011111010111110110110101010101011010011111011101",
    "10101010000000001000101000101000100000000000001000100010000010001000110110101011101011000000100010001",
    "10111010111010101011101010101111101010101110101011111111101111101010100000000000000000011010101111111",
    "10100010101010100000100010101000001010101010101000100010001000001010101111101110101010101010000000001",
    "10101111101110111111111010111111101110111010111010101110111011101111100010111110101010110011101011101",
    "10001000100010100000001010100000000010101010100010101000001010101000101000100000100010000010101000101",
    "10111011101111111010111011111110101110101010111010101110101110101110101111111111111011111110111110111",
    "10000000100010000010000010000000101000001010001010000010100010000010000010001000001000100000000000001",
    "10101111101111101110111010111111111011101011101110111111101111101110101111101110101111111011101011101",
    "10100000100000101000101010000010101010101000100010000000001010000010101000000010100000001010001010001",
    "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
];

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn calculate_maze(number_step: usize, data: &mut Vec<(Rotation, u16)>) {
    let mut up: u16 = 0;
    let mut down: u16 = 0;
    let mut right: u16 = 0;
    let mut left: u16 = 0;
    let mut current_direction: Direction = Direction::NORTH;
    for i in 0..number_step {
        let (rotation, length): (Rotation, u16) = *data.get(i).unwrap_or(&(Rotation::LEFT, 0));
        if rotation == Rotation::RIGHT {
            if current_direction == Direction::NORTH {
                current_direction = Direction::EAST;
                right += length;
            } else if current_direction == Direction::SOUTH {
                current_direction = Direction::WEST;
                left += length;
            } else if current_direction == Direction::EAST {
                current_direction = Direction::SOUTH;
                down += length;
            } else if current_direction == Direction::WEST {
                current_direction = Direction::NORTH;
                up += length;
            }
        } else if rotation == Rotation::LEFT {
            if current_direction == Direction::NORTH {
                current_direction = Direction::WEST;
                left += length;
            } else if current_direction == Direction::SOUTH {
                current_direction = Direction::EAST;
                right += length;
            } else if current_direction == Direction::EAST {
                current_direction = Direction::NORTH;
                up += length;
            } else if current_direction == Direction::WEST {
                current_direction = Direction::SOUTH;
                down += length;
            }
        }
        //println!("Direction {} for {} steps, next current_direction {}", rotation, length, current_direction);
    }
    println!("up: {} down: {} left: {} right: {}", up, down, left, right);
    // enter at the bottom
    assert!(SIZE_X - left - 4 - right > 0, "Oops the sequence make the maze not possible to render, change size");
    assert!(2 + left < (SIZE_X - 2), "Oops the sequence make the maze not possible to render, change size");
}

fn main() -> Result<()> {
    let mut flag: bool = false;
    let args: Vec<String> = env::args().collect();
    let number_step = 12;
    let mut data: Vec<(Rotation, u16)> = Vec::with_capacity(number_step);
    let mut stdout: Stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(terminal::SetSize(110, 40))?;
    let mut test_seed: u64 = 0;
    stdout.execute(cursor::DisableBlinking)?;
    if args.len() == 2 {
        let mut seed: String = args[1].clone();
        trim_newline(&mut seed);
        stdout.queue(cursor::MoveTo(0, SIZE_Y))?;
        println!("                                                                                                                              ");
        println!("Trying to use seed: {}", seed);
        test_seed = seed.parse::<u64>().expect("Not a number");
        let mut r: Random = Random::with_raw_seed(test_seed);
        for _ in 0..number_step {
            let length: u16 = r.next_int_n(16) as u16;
            r.next_double();
            r.next_double();
            let rotation = r.next_boolean();
            print!("{},{},", if rotation { 1 } else { 0 }, length);
            data.push((if rotation { Rotation::LEFT } else { Rotation::RIGHT }, length + 2));
        }
        println!();
        calculate_maze(number_step, &mut data);
        flag = true;
        stdout.flush()?;
    }


    // trace the static grid
    for (y, string) in GRID.iter().enumerate() {
        for (x, c) in string.chars().enumerate() {
            if c == '1' {
                stdout.queue(cursor::MoveTo(x as u16, y as u16))?.queue(style::PrintStyledContent("█".white()))?;
            }
        }
    }
    stdout.queue(cursor::MoveTo(SIZE_X, 1))?.queue(style::PrintStyledContent("█".blue()))?;

    if flag {
        let mut pos_x: i16 = SIZE_X as i16;
        let mut pos_y: i16 = 1;
        let mut current_direction: Direction = Direction::NORTH;
        let mut next_direction: Direction = Direction::NORTH;
        for i in 0..number_step {
            let (rotation, length): (Rotation, u16) = *data.get(i).unwrap_or(&(Rotation::LEFT, 0));
            for _ in 0..length {
                if current_direction == Direction::NORTH {
                    if rotation == Rotation::RIGHT {
                        next_direction = Direction::EAST;
                        stdout.queue(style::PrintStyledContent("█".red()))?;
                        pos_x += 1;
                    } else {
                        next_direction = Direction::WEST;
                        stdout.queue(cursor::MoveLeft(2))?.queue(style::PrintStyledContent("█".red()))?;
                        pos_x -= 1;
                    }
                } else if current_direction == Direction::SOUTH {
                    if rotation == Rotation::LEFT {
                        next_direction = Direction::EAST;
                        stdout.queue(style::PrintStyledContent("█".red()))?;
                        pos_x += 1;
                    } else {
                        next_direction = Direction::WEST;
                        stdout.queue(cursor::MoveLeft(2))?.queue(style::PrintStyledContent("█".red()))?;
                        pos_x -= 1;
                    }
                } else if current_direction == Direction::EAST {
                    if rotation == Rotation::LEFT {
                        next_direction = Direction::NORTH;
                        stdout.queue(cursor::MoveUp(1))?
                            .queue(cursor::MoveLeft(1))?
                            .queue(style::PrintStyledContent("█".red()))?;
                        pos_y -= 1;
                    } else {
                        next_direction = Direction::SOUTH;
                        stdout.queue(cursor::MoveDown(1))?.queue(cursor::MoveLeft(1))?.queue(style::PrintStyledContent("█".red()))?;
                        pos_y += 1
                    }
                } else if current_direction == Direction::WEST {
                    if rotation == Rotation::RIGHT {
                        next_direction = Direction::NORTH;
                        stdout.queue(cursor::MoveUp(1))?
                            .queue(cursor::MoveLeft(1))?
                            .queue(style::PrintStyledContent("█".red()))?;
                        pos_y -= 1;
                    } else {
                        next_direction = Direction::SOUTH;
                        stdout.queue(cursor::MoveDown(1))?.queue(cursor::MoveLeft(1))?.queue(style::PrintStyledContent("█".red()))?;
                        pos_y += 1
                    }
                }
                if pos_y >= 0 && pos_y <= SIZE_Y as i16 && pos_x >= 0 && pos_x <= SIZE_X as i16 {
                    if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == 'E' {
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        println!("Well done, you found the seed {} ! Exiting in 10s", test_seed);
                        stdout.flush()?;
                        sleep(Duration::new(10, 0));
                        return Ok(());
                    } else if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == '1' {
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        println!("Oops that's a wall ! Exiting in 10s");
                        stdout.flush()?;
                        sleep(Duration::new(10, 0));
                        return Ok(());
                    }
                } else {
                    stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                    println!("                                                                                                                              ");
                    println!("Oops that's the void ! Exiting in 10s");
                    stdout.flush()?;
                    sleep(Duration::new(10, 0));
                    return Ok(());
                }
            }


            current_direction = next_direction;
        }
        stdout.flush()?;
        sleep(Duration::new(10, 0));
        return Ok(());
    }
    enable_raw_mode()?;
    let mut pos_x: i16 = SIZE_X as i16;
    let mut pos_y: i16 = 1;
    stdout.queue(cursor::MoveTo(pos_x as u16 + 1, pos_y as u16))?;
    stdout.queue(cursor::SavePosition)?;
    stdout.queue(cursor::MoveTo(0, SIZE_Y + 1))?;
    stdout.flush()?;
    let mut turn = 0;
    let mut current_advance = 0;
    let mut result: String = String::new();
    println!("Welcome to Maze 3000, the goal? Solve the maze !");
    println!("The catch? The goal is not really to solve it !");
    println!("I will leave those 2 counters here: {}/12, {} move", turn, current_advance);
    stdout.queue(cursor::RestorePosition)?;
    stdout.flush()?;
    let mut current_direction: Direction = Direction::NORTH;
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Up {
                    stdout.queue(cursor::SavePosition)?;
                    pos_y -= 1;
                    if pos_y < 0 {
                        pos_y += 1;
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("Stop it")
                    } else {
                        if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == '1' {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("You are trying to hit a wall, it's not very effective");
                            pos_y += 1;
                        } else {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::RestorePosition)?;
                            stdout.queue(cursor::MoveUp(1))?.queue(cursor::MoveLeft(1))?.queue(style::PrintStyledContent("█".red()))?;
                            stdout.queue(cursor::SavePosition)?;
                            stdout.flush()?;
                            if current_direction == Direction::NORTH {
                                current_advance += 1;
                            } else {
                                turn += 1;
                                current_advance -= 2;
                                result.push_str(&*current_advance.to_string());
                                result.push(',');
                                result.push(if current_direction == Direction::WEST { '1' } else { '0' });
                                result.push(',');
                                current_advance = 1;
                                current_direction = Direction::NORTH;
                            }
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("I will leave those 2 counters here: {}/12, {} move", turn, current_advance);
                        }
                    }
                    stdout.queue(cursor::RestorePosition)?;
                    stdout.flush()?;
                } else if event.code == KeyCode::Down {
                    stdout.queue(cursor::SavePosition)?;
                    pos_y += 1;
                    if pos_y > SIZE_X as i16 {
                        pos_y -= 1;
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("Stop it")
                    } else {
                        if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == '1' {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("You are trying to hit a wall, it's not very effective");
                            pos_y -= 1;
                        } else {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");

                            stdout.queue(cursor::RestorePosition)?;
                            stdout.queue(cursor::MoveDown(1))?.queue(cursor::MoveLeft(1))?.queue(style::PrintStyledContent("█".red()))?;
                            stdout.queue(cursor::SavePosition)?;
                            stdout.flush()?;
                            if current_direction == Direction::SOUTH {
                                current_advance += 1;
                            } else {
                                turn += 1;
                                current_advance -= 2;
                                result.push_str(&*current_advance.to_string());
                                result.push(',');
                                result.push(if current_direction == Direction::EAST { '1' } else { '0' });
                                result.push(',');
                                current_advance = 1;
                                current_direction = Direction::SOUTH;
                            }
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("                                                                                                                              ");

                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("I will leave those 2 counters here: {}/12, {} move", turn, current_advance);
                        }
                    }
                    stdout.queue(cursor::RestorePosition)?;
                    stdout.flush()?;
                } else if event.code == KeyCode::Right {
                    stdout.queue(cursor::SavePosition)?;
                    pos_x += 1;
                    if pos_x > SIZE_X as i16 {
                        pos_x -= 1;
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("Stop it")
                    } else {
                        if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == '1' {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("You are trying to hit a wall, it's not very effective");
                            pos_x -= 1;
                        } else {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");

                            stdout.queue(cursor::RestorePosition)?;
                            stdout.queue(style::PrintStyledContent("█".red()))?;
                            stdout.queue(cursor::SavePosition)?;
                            stdout.flush()?;
                            if current_direction == Direction::WEST {
                                current_advance += 1;
                            } else {
                                turn += 1;
                                current_advance -= 2;
                                result.push_str(&*current_advance.to_string());
                                result.push(',');
                                result.push(if current_direction == Direction::SOUTH { '1' } else { '0' });
                                result.push(',');
                                current_advance = 1;
                                current_direction = Direction::WEST;
                            }
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("                                                                                                                              ");

                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("I will leave those 2 counters here: {}/12, {} move", turn, current_advance);
                        }
                    }
                    stdout.queue(cursor::RestorePosition)?;
                    stdout.flush()?;
                } else if event.code == KeyCode::Left {
                    stdout.queue(cursor::SavePosition)?;
                    pos_x -= 1;
                    if pos_x < 0 {
                        pos_x += 1;
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("                                                                                                                              ");
                        stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                        println!("Stop it")
                    } else {
                        if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == '1' {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("You are trying to hit a wall, it's not very effective");
                            pos_x += 1;
                        } else {
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 4))?;
                            println!("                                                                                                                              ");
                            stdout.queue(cursor::RestorePosition)?;
                            stdout.queue(cursor::MoveLeft(2))?.queue(style::PrintStyledContent("█".red()))?;
                            stdout.queue(cursor::SavePosition)?;
                            stdout.flush()?;
                            if current_direction == Direction::EAST {
                                current_advance += 1;
                            } else {
                                turn += 1;
                                current_advance -= 2;
                                result.push_str(&*current_advance.to_string());
                                result.push(',');
                                result.push(if current_direction == Direction::NORTH { '1' } else { '0' });
                                result.push(',');
                                current_advance = 1;
                                current_direction = Direction::EAST;
                            }
                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("                                                                                                                              ");

                            stdout.queue(cursor::MoveTo(0, SIZE_Y + 3))?;
                            println!("I will leave those 2 counters here: {}/12, {} move", turn, current_advance);
                        }
                    }
                    stdout.queue(cursor::RestorePosition)?;
                    stdout.flush()?;
                }
                if event.code==KeyCode::Esc{
                    disable_raw_mode()?;
                    return Ok(());
                }
            }
            _ => {}
        }
        if turn > 12 {
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            stdout.flush()?;
            stdout.queue(cursor::MoveTo(0, 1))?;
            println!("You lost, exiting in 3s");
            stdout.flush()?;
            sleep(Duration::new(3, 0));
            disable_raw_mode()?;
            return Ok(());
        }
        if pos_y >= 0 && pos_y <= SIZE_Y as i16 && pos_x >= 0 && pos_x <= SIZE_X as i16 {
            if GRID.get(pos_y as usize).unwrap_or(&"").chars().nth(pos_x as usize).unwrap() == 'E' {
                stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                stdout.flush()?;
                stdout.queue(cursor::MoveTo(0, 1))?;
                println!("You won, exiting in 10s");
                println!("You might need that to recover the proper seed ;)");
                current_advance -= 1;
                result.push_str(&*current_advance.to_string());
                result.push(',');
                let mut iter = result.chars();
                iter.by_ref().nth(2); // eat up start values
                let slice = iter.as_str(); // get back a slice of the rest of the iterator
                println!("{}", slice);
                stdout.flush()?;
                sleep(Duration::new(10, 0));
                disable_raw_mode()?;
                return Ok(());
            }
        }
    }
}