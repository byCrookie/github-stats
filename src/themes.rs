use log::warn;

pub struct Theme {
    pub title_color: String,
    pub icon_color: String,
    pub text_color: String,
    pub background_color: String,
    pub border_color: String,
}

pub fn dark() -> Theme {
    Theme {
        title_color: String::from("#fff"),
        icon_color: String::from("#79ff97"),
        text_color: String::from("#9f9f9f"),
        background_color: String::from("#151515"),
        border_color: String::from("#fff"),
    }
}

pub fn light() -> Theme {
    Theme {
        title_color: String::from("#2f2f2f"),
        icon_color: String::from("#4078c0"),
        text_color: String::from("#555555"),
        background_color: String::from("#fffefe"),
        border_color: String::from("#e4e2e2"),
    }
}

/// Returns the theme matching `name`, falling back to dark for unknown names.
pub fn from_name(name: &str) -> Theme {
    match name {
        "dark" => dark(),
        "light" => light(),
        _ => {
            warn!("Unknown theme '{}', falling back to dark", name);
            dark()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_has_expected_background() {
        let theme = dark();
        assert_eq!(theme.background_color, "#151515");
    }

    #[test]
    fn light_theme_has_light_background() {
        let theme = light();
        assert_eq!(theme.background_color, "#fffefe");
    }

    #[test]
    fn from_name_dark() {
        let theme = from_name("dark");
        assert_eq!(theme.background_color, "#151515");
    }

    #[test]
    fn from_name_light() {
        let theme = from_name("light");
        assert_eq!(theme.background_color, "#fffefe");
    }

    #[test]
    fn from_name_unknown_falls_back_to_dark() {
        let theme = from_name("nonexistent");
        assert_eq!(theme.background_color, "#151515");
    }
}
