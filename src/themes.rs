pub struct Theme {
    pub title_color: String,
    pub icon_color: String,
    pub text_color: String,
    pub background_color: String,
    pub border_color: String,
}

pub fn dark() -> Theme {
    return Theme {
        title_color: String::from("#fff"),
        icon_color: String::from("#9f9f9f"),
        text_color: String::from("#79ff97"),
        background_color: String::from("#151515"),
        border_color: String::from("#fff"),
    };
}
