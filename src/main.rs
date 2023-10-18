// TODO(#1): Choose => Option 1: CLI App, Option 2: Terminal App with simple GUI
// TODO(#2): Search for needed dependencies and include them in cargo.toml
// TODO(#3): Implement showing files down the specified path
// TODO(#4): Implement choice of files from printed list
// TODO(#5): Implement file content encryption
// TODO(#6): Implement file content decryption
// TODO(#7): Create option to choose encryption method
// TODO(#8): Store file encryption method, different custom file extensions?

use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

#[derive(Default)]
struct Vec2 {
    row: i32,
    col: i32,
}

impl Vec2 {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Default)]
struct UI {
    pos: Vec2,
    size: Vec2,
}

impl UI {
    fn begin(&mut self, pos: Vec2) {
        // Assert if the pos is empty

        self.pos = pos;
        self.size = Vec2::new(0, 0)
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.pos.col, self.pos.row);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.pos.row += 1;
    }

    fn list_elements() {
        todo!()
    }

    fn end(&mut self) {
        self.pos = Vec2::new(0, 0);
        self.size = Vec2::new(0, 0);
    }
}

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    timeout(16);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    refresh();
    let mut quit = false;
    let mut ui = UI::default();

    while !quit {
        erase();

        ui.begin(Vec2::new(0, 0));
        {
            ui.label("Hello, World", REGULAR_PAIR);
        }
        ui.end();

        let key: i32 = getch();
        match key as u8 as char {
            'q' => {
                quit = true;
            }
            _ => {}
        }

        refresh();
    }
    endwin();
}
