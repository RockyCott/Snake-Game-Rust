use k_board::{keyboard::Keyboard, keys::Keys};
use rand::Rng;

const COLS: usize = 25;
const ROWS: usize = 25;

const FOODS: usize = 50;

static mut BOARD: [char; ROWS * COLS] = [' '; ROWS * COLS];

static mut IS_GAME_OVER: bool = false;

fn fill_board() {
    for y in 0..ROWS {
        for x in 0..COLS {
            let index = y * COLS + x;
            unsafe {
                BOARD[index] = if x == 0 || x == COLS - 1 || y == 0 || y == ROWS - 1 {
                    '#'
                } else {
                    ' '
                }
            }
        }
    }
}

fn clear_screen() {
    // Linux and MacOS
    std::process::Command::new("clear").status().unwrap();

    // Windows
    // std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
}

fn print_board() {
    for y in 0..ROWS {
        for x in 0..COLS {
            let index = y * COLS + x;
            unsafe {
                print!("{}", BOARD[index]);
            }
        }
        println!();
    }
}

const SNAKE_MAX_LEN: usize = 256;

#[derive(Debug, Clone, Copy)]
struct SnakePart {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Snake {
    length: usize,
    parts: [SnakePart; SNAKE_MAX_LEN],
}

impl Snake {
    fn new() -> Self {
        Snake {
            length: 1,
            parts: [SnakePart { x: 0, y: 0 }; SNAKE_MAX_LEN],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Food {
    x: i32,
    y: i32,
    consumed: bool,
}

#[derive(Debug)]
struct Foods {
    items: [Food; FOODS],
}

impl Foods {
    fn new() -> Self {
        Self {
            items: [Food {
                x: 0,
                y: 0,
                consumed: false,
            }; FOODS],
        }
    }
}

fn draw_snake(snake: &Snake) {
    unsafe {
        for i in (1..snake.length).rev() {
            println!("{}", i);
            let part = snake.parts[i];
            BOARD[(part.y * COLS as i32 + part.x) as usize] = '*';
        }

        let part_0 = snake.parts[0];
        BOARD[(part_0.y * COLS as i32 + part_0.x) as usize] = '@';
    }
}

fn move_snake(dx: i32, dy: i32, snake: &mut Snake) {
    for i in (1..snake.length).rev() {
        snake.parts[i] = snake.parts[i - 1];
    }

    snake.parts[0] = SnakePart {
        x: snake.parts[0].x + dx,
        y: snake.parts[0].y + dy,
    };
}

fn read_keyboard(snake: &mut Snake) {
    for key in Keyboard::new() {
        match key {
            Keys::Char('w') | Keys::Char('W') => {
                move_snake(0, -1, snake);
                break;
            }
            Keys::Char('s') | Keys::Char('S') => {
                move_snake(0, 1, snake);
                break;
            }
            Keys::Char('a') | Keys::Char('A') => {
                move_snake(-1, 0, snake);
                break;
            }
            Keys::Char('d') | Keys::Char('D') => {
                move_snake(1, 0, snake);
                break;
            }
            Keys::Char('q') | Keys::Char('Q') => unsafe {
                IS_GAME_OVER = true;
                break;
            },
            _ => {}
        }
    }
}

fn draw_food(food: &Foods) {
    unsafe {
        for i in 0..FOODS {
            if !food.items[i].consumed {
                BOARD[(food.items[i].y * COLS as i32 + food.items[i].x) as usize] = 'o';
            }
        }
    }
}

fn setup_food(foods: &mut Foods) {
    let mut rng = rand::thread_rng();
    for i in 0..FOODS {
        foods.items[i] = Food {
            x: rng.gen_range(1..COLS as i32 - 1),
            y: rng.gen_range(1..ROWS as i32 - 1),
            consumed: false,
        };
    }
}

fn setup_snake(snake: &mut Snake) {
    let mut rng = rand::thread_rng();
    snake.length = 1;
    snake.parts[0] = SnakePart {
        x: rng.gen_range(1..COLS as i32 - 1),
        y: rng.gen_range(1..ROWS as i32 - 1),
    };
}

fn game_rules(snake: &mut Snake, food: &mut Foods) {
    for i in 0..FOODS {
        if !food.items[i].consumed
            && food.items[i].x == snake.parts[0].x
            && food.items[i].y == snake.parts[0].y
        {
            food.items[i].consumed = true;
            snake.length += 1;
        }
    }

    if snake.parts[0].x == 0
        || snake.parts[0].x == (COLS - 1) as i32
        || snake.parts[0].y == 0
        || snake.parts[0].y == (ROWS - 1) as i32
    {
        unsafe {
            IS_GAME_OVER = true;
        }
    }

    for i in 1..snake.length {
        if snake.parts[0].x == snake.parts[i].x && snake.parts[0].y == snake.parts[i].y {
            unsafe {
                IS_GAME_OVER = true;
            }
        }
    }
}

fn main() {
    let mut snake = Snake::new();

    setup_snake(&mut snake);

    let mut food = Foods::new();

    setup_food(&mut food);

    unsafe {
        while !IS_GAME_OVER {
            fill_board();
            draw_food(&food);
            draw_snake(&snake);
            game_rules(&mut snake, &mut food);
            clear_screen();
            println!("Snake Game, Score: {}", snake.length * 100);
            print_board();
            if !IS_GAME_OVER {
                read_keyboard(&mut snake);
            }
        }
    }

    println!("Game Over!, Final Score: {}", snake.length * 100);
}
