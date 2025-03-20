use rand::{Rng, rng};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, cursor};

fn main() {
    let score = Arc::new(Mutex::new(0));
    let score_clone = Arc::clone(&score);
    let board = Arc::new(Mutex::new([
        ['_'; 8], ['_'; 8], ['_'; 8], ['_'; 8], ['_'; 8], ['_'; 8], ['_'; 8], ['_'; 8],
    ]));

    let specx_loc = Arc::new(Mutex::new([4, 7])); // Player starts at bottom center

    // Generate 3 random (x, y) coordinates
    let meteorites = Arc::new(Mutex::new(
        (0..5)
            .map(|_| [rng().random_range(0..7), rng().random_range(0..7)])
            .collect::<Vec<_>>(),
    ));

    let meteorites_clone = Arc::clone(&meteorites);

    // Spawn thread for moving meteorites
    thread::spawn(move || -> ! {
        let mut sleep_mil = 500;
        loop {
            {
                let mut meteor = meteorites_clone.lock().unwrap();
                for m in meteor.iter_mut() {
                    if m[1] < 7 {
                        m[1] += 1;
                    } else {
                        m[0] = rng().random_range(1..8);
                        m[1]=0;
                        *score_clone.lock().unwrap() += 1;
                    }
                }

                if *score_clone.lock().unwrap() > 20 && sleep_mil > 50{
                    sleep_mil -= 2;
                    if  meteor.len() < 8 {
                        meteor.push([rng().random_range(0..7), rng().random_range(0..7)]);
                    }
                }
            }
            thread::sleep(Duration::from_millis(sleep_mil)); // Meteorite moves every second
        }
    });

    // Setup terminal input mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let stdin = async_stdin();
    let mut stdin_keys = stdin.keys(); // Read keyboard input

    'main_loop: loop {

        // Read input (non-blocking)
        if let Some(Ok(key)) = stdin_keys.next() {
            match key {
                Key::Char('q') => break, // Quit game
                Key::Left => move_left(&specx_loc),
                Key::Right => move_right(&specx_loc),
                _ => {}
            }
        }

        // Check if meteorite is touched the x
        for m in meteorites.lock().unwrap().iter() {
            if m[0] == specx_loc.lock().unwrap()[0] && m[1] == specx_loc.lock().unwrap()[1] {
                write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                println!("Game Over!, You got {} scores.", *score.lock().unwrap());
                stdout.flush().unwrap();
                break 'main_loop;
            }
        }

        // Clear screen and move cursor to top
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        draw(&board, &specx_loc, &meteorites, score.clone());
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50)); // Controls refresh rate
    }

    println!("\rExit ...");
}

fn move_right(specx_loc: &Arc<Mutex<[usize; 2]>>) {
    let mut pos = specx_loc.lock().unwrap();
    if pos[0] < 7 {
        pos[0] += 1;
    }
}

fn move_left(specx_loc: &Arc<Mutex<[usize; 2]>>) {
    let mut pos = specx_loc.lock().unwrap();
    if pos[0] > 0 {
        pos[0] -= 1;
    }
}

fn draw(
    board: &Arc<Mutex<[[char; 8]; 8]>>,
    specx_loc: &Arc<Mutex<[usize; 2]>>,
    meteorites: &Arc<Mutex<Vec<[usize; 2]>>>,
    score: Arc<Mutex<i32>>,
) {
    let board = board.lock().unwrap();
    let specx_loc = specx_loc.lock().unwrap();
    let meteorites = meteorites.lock().unwrap();

    for row in 0..8 {
        for col in 0..8 {
            let mut printed = false;

            // Draw meteorites
            for &m in meteorites.iter() {
                if m[0] == col && m[1] == row {
                    print!("o");
                    printed = true;
                    break;
                }
            }

            // Draw player
            if specx_loc[0] == col && specx_loc[1] == row {
                print!("X");
                printed = true;
            }

            // Draw board
            if !printed {
                print!("{}", board[row][col]);
            }
        }
        println!("\r");
    }
    println!("\rScores: {}", *score.lock().unwrap())
}
