// TODO(#6): Implement file content decryption
// TODO(#8): Store file encryption method, different custom file extensions?
// TODO(#15): Add error handling
// TODO(#19): Shorten path string
<<<<<<< HEAD
// TODO(#20): Make sure it is working with Linux and Windows
=======
>>>>>>> c55db28690d71439476602b7e5fbf1c33f25cfa9

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

    fn list_elements(&mut self, list: &mut Vec<String>, file_focus: usize) {
        for (index, item) in list.iter().enumerate() {
            self.label(
                &format!("{}", item),
                if index == file_focus {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                },
            )
        }
    }

    fn list_up(&mut self, file_focus: &mut i32) {
        if *file_focus > 0 {
            *file_focus -= 1;
        }
    }

    fn list_down(&mut self, file_focus: &mut i32, list: &[String]) {
        if *file_focus < (list.len() - 1) as i32 {
            *file_focus += 1;
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

impl FileExplorer {
    fn begin(&mut self) {
        let working_dir = env::current_dir().unwrap();
        self.path = working_dir.to_str().unwrap().to_string();
        self.refresh();
    }

    fn refresh(&mut self) {
        self.file_list.clear();
        self.file_list = self.scan_path();
    }

    fn set_path(&mut self, new_path: String) {
        let path = Path::new(&self.path);
        self.path = path.join(new_path).display().to_string();
        self.refresh();
    }

    fn handle_select(&mut self, file_focus: &mut i32) -> Option<String> {
        let path: &Path = Path::new(&self.path);
        let new_path = path.join(self.file_list[*file_focus as usize].clone());
        if new_path.is_dir() {
            self.set_path(new_path.display().to_string());
            return None;
        } else {
            return Some(new_path.display().to_string());
        }
    }

    fn dir_down(&mut self) -> bool {
        let path = Path::new(&self.path);

        if path.parent() != None {
            self.set_path(path.parent().unwrap().display().to_string());
            return true;
        }
        return false;
    }

    fn scan_path(&mut self) -> Vec<String> {
        let curr_path = Path::new(&self.path);
        let mut result: Vec<String> = Vec::<String>::new();

        for entry in curr_path.read_dir().expect("read_dir call failes") {
            if let Ok(entry) = entry {
                result.push(format!("{}", entry.path().display()));
            }
        }
        return result;
    }

    fn end(&mut self) {
        self.file_list.clear();
        self.path.clear();
    }
}

#[derive(Default)]
struct FileHandler {
    filepath: String,
    method: String,
}

impl FileHandler {
    fn handle(&mut self, path: String, method: String) {
        self.set_path(path);
        self.method = method;
        match self.method.as_str() {
            "Read file" => {
                self.open_file();
            }
            _ => {}
        }
    }

    fn open_file(&mut self) {
        let _ = open::that(self.filepath.clone());
    }

    fn set_path(&mut self, path: String) {
        self.filepath = path;
    }
}

fn main() {
    let mut explorer: FileExplorer = FileExplorer::default();
    let mut filehandler: FileHandler = FileHandler::default();
    let mut action_list: Vec<String> = vec![
        "Read file".to_string(),
        "Encrypt".to_string(),
        "Decrypt".to_string(),
    ];
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
    let mut file_focus: i32 = 0;
    let mut action_focus: i32 = 0;
    let mut key_curr = None;
    let mut notification = String::new();

    while !quit {
        let mut file_list: Vec<String> = explorer.file_list.clone();
        erase();

        let mut x = 0;
        let mut y = 0;
        getmaxyx(stdscr(), &mut y, &mut x);

        ui.begin(Vec2::new(0, 0));
        {
            ui.label(&notification, REGULAR_PAIR);
        }
        ui.end();

        ui.begin(Vec2::new(2, 0));
        {
            ui.list_elements(&mut file_list, file_focus as usize);
        }
        ui.end();

        ui.begin(Vec2::new(2, x / 2));
        {
            ui.list_elements(&mut action_list, action_focus as usize);
        }
        ui.end();

        if let Some(key) = key_curr.take() {
            match key {
                constants::KEY_UP => ui.list_up(&mut file_focus),
                constants::KEY_DOWN => ui.list_down(&mut file_focus, &file_list),
                100 => {
                    if explorer.dir_down() {
                        notification.push_str("Moved directory down");
                        file_focus = 0;
                    } else {
                        notification.push_str("You are at the lowest directory")
                    }
                }
                10 => {
                    if let Some(filepath) = explorer.handle_select(&mut file_focus) {
                        // TODO(#5): Implement file content encryption
                        // TODO(#7): Create option to choose encryption method
                        filehandler.handle(filepath, action_list[action_focus as usize].clone());
                    } else {
                        notification.push_str("Moved directory up");
                        file_focus = 0;
                    }
                }
                119 => {
                    if action_focus > 0 {
                        action_focus -= 1;
                    }
                }
                115 => {
                    if action_focus < (action_list.len() - 1) as i32 {
                        action_focus += 1;
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
