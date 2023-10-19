// TODO(#6): Implement file content decryption
// TODO(#8): Store file encryption method, different custom file extensions?
// TODO(#15): Add error handling

use ncurses::*;
use std::{env, path::Path};

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
}

// TODO(#14): Restructure and optimize FileExplorer struct
impl FileExplorer {
    fn begin(&mut self) {
        let pwd = env::current_dir().unwrap();
        self.path = pwd.to_str().unwrap().to_string();
        self.file_list = self.get_file_list();
    }

    fn refresh(&mut self) {
        self.file_list.clear();
        self.get_file_list();
    }

    fn change_path(&mut self, new_path: String) {
        let path = Path::new(&self.path);
        self.path = path.join(new_path).display().to_string();
        self.refresh();
    }

    fn handle_select(&mut self, focus: &mut i32) -> Option<String> {
        let path = Path::new(&self.path);
        let new_path = path.join(self.file_list[*focus as usize].clone());
        if new_path.is_dir() {
            self.change_path(new_path.display().to_string());
            return None;
        } else {
            return Some(new_path.display().to_string());
        }
    }

    // TODO(#16): Create error handling for moving down from lowest level folder
    fn dir_down(&mut self) {
        let path = Path::new(&self.path);
        self.change_path(path.parent().unwrap().display().to_string());
    }

    fn get_files(&mut self) {
        let curr_path = Path::new(&self.path);

        for entry in curr_path.read_dir().expect("read_dir call failes") {
            if let Ok(entry) = entry {
                self.file_list.push(format!("{}", entry.path().display()));
            }
        }
    }

    fn get_file_list(&mut self) -> Vec<String> {
        self.get_files();
        return self.file_list.clone();
    }

    fn end(&mut self) {
        self.file_list.clear();
        self.path.clear();
    }
}

fn main() {
    let mut explorer: FileExplorer = FileExplorer::default();
    explorer.begin();

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
    let mut key_curr = None;
    let mut notification: String = String::new();

    while !quit {
        let mut file_list: Vec<String> = explorer.file_list.clone();
        erase();

        ui.begin(Vec2::new(0, 0));
        {
            ui.label(&notification, REGULAR_PAIR);
        }
        ui.end();

        ui.begin(Vec2::new(2, 0));
        {
            ui.list_elements(&mut file_list, focus as usize)
        }
        ui.end();

        if let Some(key) = key_curr.take() {
            match key {
                constants::KEY_UP => ui.list_up(&mut focus),
                constants::KEY_DOWN => ui.list_down(&mut focus, &file_list),
                100 => {
                    explorer.dir_down();
                    notification.push_str("Moved directory down");
                    focus = 0;
                }
                10 => {
                    if let Some(res) = explorer.handle_select(&mut focus) {
                        // TODO(#5): Implement file content encryption
                        // TODO(#7): Create option to choose encryption method
                        notification.push_str("This is a file");
                    } else {
                        notification.push_str("Moved directory up");
                        focus = 0;
                    }
                }
                113 => {
                    quit = true;
                }
                _ => {
                    key_curr = Some(key);
                }
            }
        }

        if let Some('q') = key_curr.take().map(|x| x as u8 as char) {
            quit = true;
        }

        refresh();

        let key = getch();
        if key != ERR {
            notification.clear();
            key_curr = Some(key);
        }
    }
    explorer.end();
    endwin();
}
