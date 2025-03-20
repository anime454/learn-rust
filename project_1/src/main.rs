use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, cursor};

fn main() {
    let board = Arc::new(Mutex::new([
        ['o'; 8], ['o'; 8], ['o'; 8], ['o'; 8], ['o'; 8], ['o'; 8], ['o'; 8], ['o'; 8],
    ]));

    let specx_loc = Arc::new(Mutex::new([4, 7])); // Player starts at bottom center
    let meteorites = Arc::new(Mutex::new(vec![[1, 1], [4, 1], [7, 1]]));

    let meteorites_clone = Arc::clone(&meteorites);
    // let board_clone = Arc::clone(&board);

    // Spawn thread for moving meteorites
    thread::spawn(move || {
        loop {
            {
                let mut meteor = meteorites_clone.lock().unwrap();
                for m in meteor.iter_mut() {
                    if m[1] < 7 {
                        m[1] += 1;
                    } else {
                        m[1] = 0; // Reset meteorite position
                    }
                }
            }
            thread::sleep(Duration::from_secs(1)); // Meteorite moves every second
        }
    });

    // Setup terminal input mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let stdin = async_stdin();
    let mut stdin_keys = stdin.keys(); // Read keyboard input

    loop {

        // Read input (non-blocking)
        if let Some(Ok(key)) = stdin_keys.next() {
            match key {
                Key::Char('q') => break, // Quit game
                Key::Left => move_left(&specx_loc),
                Key::Right => move_right(&specx_loc),
                _ => {}
            }
        }

        // Clear screen and move cursor to top
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        draw(&board, &specx_loc, &meteorites);
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50)); // Controls refresh rate
    }

    println!("\nGame Over!");
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
                    print!("U");
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
}

// // Convert byte to Key type
// fn byte_to_key(byte: u8) -> Option<Key> {
//     match byte {
//         b'q' => Some(Key::Char('q')),
//         b'1' => Some(Key::Left),
//         b'd' => Some(Key::Right),
//         _ => None,
//     }
// }
