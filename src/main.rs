use std::sync::{Arc,  Mutex};
use std::{thread,  time};
use getch_rs::{Getch,  Key};
use rand::{
    distributions::{Distribution,  Standard},
    Rng,
};

const FIELD_WIDTH:  usize = 11 + 2;
const FIELD_HEIGHT: usize = 20 + 1;
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone,  Copy)]
enum BlockKind {
    I, O, S, Z, J, L, T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self,  rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
    [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
];

struct Position {
    x: usize,
    y: usize,
}

fn draw(field: &Field,  pos: &Position,  block: BlockKind) {
    let mut field_buf = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] == 1 {
                field_buf[y+pos.y][x+pos.x] = 1;
            }
        }
    }
    println!("\x1b[H");
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn is_collision(field: &Field,  pos: &Position,  block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y+pos.y >= FIELD_HEIGHT || x+pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y+pos.y][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field = Arc::new(Mutex::new([
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));

    let pos = Arc::new(Mutex::new(Position { x: 4,  y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));

    println!("\x1b[2J\x1b[H\x1b[?25l");
    draw(&field.lock().unwrap(),  &pos.lock().unwrap(),  *block.lock().unwrap());

    {
        let pos   = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let block = Arc::clone(&block);
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(1000));
                let mut pos   = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field,  &new_pos,  *block) {
                    *pos = new_pos;
                } else {
                    for y in 0..4 {
                        for x in 0..4 {
                            if BLOCKS[*block as usize][y][x] == 1 {
                                field[y+pos.y][x+pos.x] = 1;
                            }
                        }
                    }
                    for y in 1..FIELD_HEIGHT-1 {
                        let mut can_erase = true;
                        for x in 1..FIELD_WIDTH-1 {
                            if field[y][x] == 0 {
                                can_erase = false;
                                break;
                            }
                        }
                        if can_erase {
                            for y2 in (2..=y).rev() {
                                field[y2] = field[y2-1];
                            }
                        }
                    }
                    *pos = Position { x: 4,  y: 0 };
                    *block = rand::random();
                }
                draw(&field,  &pos,  *block);
            }
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x.checked_sub(1).unwrap_or_else(|| pos.x),
                    y: pos.y,
                };
                if !is_collision(&field,  &new_pos,  *block) {
                    *pos = new_pos;
                }
                draw(&field,  &pos,  *block);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field,  &new_pos,  *block) {
                    *pos = new_pos;
                }
                draw(&field,  &pos,  *block);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field,  &new_pos,  *block) {
                    *pos = new_pos;
                }
                draw(&field,  &pos,  *block);
            }
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h");
                return;
            }
            _ => (),
        }
    }
}
