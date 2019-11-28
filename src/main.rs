use state::Board;
use termion::cursor;
mod state;

fn main() {
    let mut board = Board::make_rand(20, 20);
    loop {
        print!("{}", cursor::Save);
        print!("{}\n\n\n", board);
        print!("{}", cursor::Restore);
        std::thread::sleep(std::time::Duration::from_secs(1));
        board.next_mut_inplace();
        // print!("{}", cursor::Hide);
    }
}
