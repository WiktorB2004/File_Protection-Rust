// TODO(#15): Add error handling
// TODO(#20): Make sure it is working with Linux and Windows
// TODO(#25): Create documentation, README.md file

use ncurses::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

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
    short_file_list: Vec<String>,
}

impl FileExplorer {
    fn begin(&mut self) {
        let working_dir = env::current_dir().unwrap();
        self.path = working_dir.to_str().unwrap().to_string();
        self.refresh_dir();
    }

    fn refresh_dir(&mut self) {
        erase();
        self.file_list.clear();
        self.short_file_list.clear();
        self.file_list = self.scan_path();
        self.short_file_list = self.short_file_list();
    }

    fn set_path(&mut self, new_path: String) {
        let path = Path::new(&self.path);
        self.path = path.join(new_path).display().to_string();
        self.refresh_dir();
    }

    fn handle_select(&mut self, file_focus: &mut i32, action_focus: i32) -> Option<String> {
        let path: &Path = Path::new(&self.path);
        let new_path = path.join(self.file_list[*file_focus as usize].clone());
        if new_path.is_dir() && action_focus != 3 {
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

    fn short_file_list(&mut self) -> Vec<String> {
        let mut res: Vec<String> = Vec::<String>::new();

        for filepath in self.file_list.clone() {
            let path = Path::new(&filepath);
            let filename = path.file_name().unwrap();
            res.push(format!("../{}", filename.to_str().unwrap().to_string()));
        }
        return res;
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
    fn handle_action(
        &mut self,
        path: String,
        method: String,
        notification: &mut String,
        path_mode: &mut bool,
    ) {
        self.set_path(path);
        self.method = method;
        // TODO: Add notification about decrypting using incorrect method
        match self.method.as_str() {
            "Read file" => {
                notification.push_str("Opening selected file");
                self.open_file();
            }
            "Encrypt Caesar-1" => self.caesar_encrypt(1),
            "Encrypt Caesar-5" => self.caesar_encrypt(5),
            "Encrypt Caesar-12" => self.caesar_encrypt(12),
            "Decrypt Caesar-1" => self.caesar_decrypt(1),
            "Decrypt Caesar-5" => self.caesar_decrypt(5),
            "Decrypt Caesar-12" => self.caesar_decrypt(12),
            "Switch between full/short path" => {
                *path_mode = !*path_mode;
            }
            _ => {}
        }
    }

    fn caesar_encrypt(&mut self, mut shift: u8) {
        let mut file_content: Vec<u8> = self.read_file_vec().expect("Error while reading file");
        shift = shift % 26;
        for (idx, chr) in file_content.clone().iter().enumerate() {
            let mut swap_char: u8 = chr + shift;
            if swap_char > 126 {
                swap_char = 31 + shift;
            }
            file_content[idx] = swap_char;
        }
        let filename = Path::new(&self.filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        self.write_file_vec(file_content);
        fs::rename(filename.clone(), format!("{}c.{}", shift, filename))
            .expect("Couldnt rename a file");
    }

    fn caesar_decrypt(&mut self, mut shift: u8) {
        let mut file_content: Vec<u8> = self.read_file_vec().expect("Error while reading file");
        shift = shift % 26;
        for (idx, chr) in file_content.clone().iter().enumerate() {
            let mut swap_char: u8 = chr - shift;
            if swap_char < 32 {
                swap_char = 127 - shift;
            }
            file_content[idx] = swap_char;
        }
        let filename = Path::new(&self.filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        self.write_file_vec(file_content);
        let mut filename_vec: Vec<&str> = filename.split(".").collect();
        let _ = filename_vec.remove(0);
        let decrypted_filename = filename_vec.join(".");
        fs::rename(filename.clone(), format!("{}", decrypted_filename))
            .expect("Couldnt rename a file");
    }

    fn write_file_vec(&mut self, content: Vec<u8>) {
        File::create(&self.filepath)
            .unwrap()
            .write_all(&content)
            .expect("Couldnt write to file");
    }

    fn read_file_vec(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let data = fs::read(self.filepath.clone())?;
        Ok(data)
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
        "Switch between full/short path".to_string(),
    ];
    let method_list: Vec<String> = vec![
        "Caesar-1".to_string(),
        "Caesar-5".to_string(),
        "Caesar-12".to_string(),
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
    let mut encryption_select: i32 = 0;
    let mut decryption_select: i32 = 0;
    let mut path_mode = true;
    let mut key_curr = None;
    let mut notification = String::new();

    while !quit {
        explorer.refresh_dir();
        let mut file_list: Vec<String> = if path_mode {
            explorer.short_file_list.clone()
        } else {
            explorer.file_list.clone()
        };
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
            action_list[1] = format!("Encrypt {}", method_list[encryption_select as usize]);
            action_list[2] = format!("Decrypt {}", method_list[decryption_select as usize]);
            ui.list_elements(&mut action_list, action_focus as usize);
        }
        ui.end();

        if let Some(key) = key_curr.take() {
            match key {
                constants::KEY_UP => {
                    if action_focus > 0 {
                        action_focus -= 1;
                    }
                }
                constants::KEY_DOWN => {
                    if action_focus < (action_list.len() - 1) as i32 {
                        action_focus += 1;
                    }
                }
                constants::KEY_LEFT => {
                    if action_focus == 1 {
                        if encryption_select > 0 {
                            encryption_select -= 1;
                        }
                    } else if action_focus == 2 {
                        if decryption_select > 0 {
                            decryption_select -= 1;
                        }
                    }
                }
                constants::KEY_RIGHT => {
                    if action_focus == 1 {
                        if encryption_select < (method_list.len() - 1) as i32 {
                            encryption_select += 1;
                        }
                    } else if action_focus == 2 {
                        if decryption_select < (method_list.len() - 1) as i32 {
                            decryption_select += 1;
                        }
                    }
                }
                100 => {
                    if explorer.dir_down() {
                        notification.push_str("Moved directory down");
                        file_focus = 0;
                    } else {
                        notification.push_str("You are at the lowest directory")
                    }
                }
                10 => {
                    // TODO(#7): Create option to choose encryption method
                    if let Some(filepath) = explorer.handle_select(&mut file_focus, action_focus) {
                        filehandler.handle_action(
                            filepath,
                            action_list[action_focus as usize].clone(),
                            &mut notification,
                            &mut path_mode,
                        );
                    } else {
                        notification.push_str("Moved directory up");
                        file_focus = 0;
                    }
                }
                119 => ui.list_up(&mut file_focus),
                115 => ui.list_down(&mut file_focus, &file_list),
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
