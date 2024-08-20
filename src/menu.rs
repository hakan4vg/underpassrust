pub struct MenuOption{
    pub number: u32,
    pub description: &'static str,
}

pub fn get_menu_options() -> Vec<MenuOption>{
    vec![
        MenuOption{number: 1, description: "Guess the number"},
        MenuOption{number: 2, description: "Read file"},
        MenuOption{number: 3, description: "Write to file"},
        MenuOption{number: 4, description: "Basic calculator"},
        MenuOption{number: 5, description: "Hello world"},
        MenuOption{number: 6, description: "Exit"}
    ]
}