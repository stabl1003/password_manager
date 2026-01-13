use::tui::text::Span;
use::tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph};


const APP_KEYS_DECS: &str = r#" 
L:          List
U:          on list, its`s copy the username
P:          on list, it`s copy the password
D:          on lsit, it`s delete
E:          on list,  edit
S:          search
Insert Btn: insert new password
Tab:        go to next field 
Shift+Tab:  go to previous field
Esc:        exit insert mode
"#;

