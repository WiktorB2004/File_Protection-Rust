// TODO(#4): Implement navigation and choice of files from printed list
// TODO(#5): Implement file content encryption
// TODO(#6): Implement file content decryption
// TODO(#7): Create option to choose encryption method
// TODO(#8): Store file encryption method, different custom file extensions?

use ncurses::*;
use std::path::Path;

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
        // Assert if the pos and size is empty

        self.pos = pos;
        self.size = Vec2::new(0, 0)
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.pos.row, self.pos.col);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.pos.row += 1;
    }

    fn list_elements(&mut self, list: &mut Vec<String>, focus: usize) {
        for (index, item) in list.iter().enumerate() {
            self.label(
                &format!("{}", item),
                if index == focus {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                },
            )
        }
    }

    fn list_up(&mut self, focus: &mut i32) {
        if *focus > 0 {
            *focus -= 1;
        }
    }

    fn list_down(&mut self, focus: &mut i32, list: &[String]) {
        if *focus < (list.len() - 1) as i32 {
            *focus += 1;
        }
    }

    fn end(&mut self) {
        self.pos = Vec2::new(0, 0);
        self.size = Vec2::new(0, 0);
    }
}

#[derive(Default)]
struct FileExplorer {
    path: String,
    file_list: Vec<String>,
    curr_dir: Vec<String>,
}

// TODO(#14): Restructure and optimize FileExplorer struct
impl FileExplorer {
    fn begin(&mut self, path: String) {
        self.path = path;
        self.file_list = Vec::<String>::new();
    }

    fn change_path(&mut self, new_path: String) {
        self.file_list.clear();
        self.curr_dir.clear();
        self.path = new_path;
    }

    fn dir_up(&mut self, focus: i32) {}

    fn dir_down(&mut self) {
        self.file_list.clear();
        self.curr_dir.clear();
        self.path = format!("../{}", self.path)
    }

    fn get_files(&mut self) {
        let curr_path = Path::new(&self.path);

        for entry in curr_path.read_dir().expect("read_dir call failes") {
            if let Ok(entry) = entry {
                self.file_list.push(format!("{:?}", entry.path()));
                self.curr_dir.push(format!("{:?}", entry.file_name()))
            }
        }
    }

    fn get_file_list(&mut self, stripped: bool) -> Vec<String> {
        self.get_files();
        if stripped {
            return self.curr_dir.clone();
        }
        return self.file_list.clone();
    }

    fn end() {
        return;
    }
}

fn main() {
    let mut explorer: FileExplorer = FileExplorer::default();
    explorer.begin("./".to_string());
    let mut file_list: Vec<String> = explorer.get_file_list(true);

    initscr();
    noecho();
    keypad(stdscr(), true);
    timeout(16);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    refresh();
    let mut quit: bool = false;
    let mut ui: UI = UI::default();
    let mut focus: i32 = 0;

    while !quit {
        erase();

        ui.begin(Vec2::new(0, 0));
        {
            ui.list_elements(&mut file_list, focus as usize)
        }
        ui.end();

        let key: i32 = getch();
        match key {
            constants::KEY_UP => ui.list_up(&mut focus),
            constants::KEY_DOWN => ui.list_down(&mut focus, &file_list),
            100 => {
                explorer.dir_down();
                file_list = explorer.get_file_list(true);
            }
            10 => {
                explorer.dir_up(focus);
                file_list = explorer.get_file_list(true);
            }
            113 => {
                quit = true;
            }
            _ => {}
        }

        refresh();
    }
    // explorer.end();
    endwin();
}
