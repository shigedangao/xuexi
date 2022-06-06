#[derive(Debug, Clone)]
pub struct Colors {
   pub class_name: String
}

impl Colors {
    fn from_light_theme() -> Self {
        Colors {
            class_name: "light".to_owned()
        }
    }

    pub fn from_dark_theme() -> Self {
        Colors {
            class_name: "dark".to_owned()
        }
    }
}

pub fn get_theme_color(selected_theme: &str) -> Colors {
    match selected_theme {
        "light" => Colors::from_light_theme(),
        _ => Colors::from_dark_theme()
    }
}