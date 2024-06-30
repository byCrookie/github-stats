use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt::Write;
use std::mem;

#[derive(Debug, Clone)]
struct Lang {
    name: String,
    size: f64,
    color: String,
}

struct Card {
    width: usize,
    height: usize,
    border_radius: Option<usize>,
    colors: CardColors,
    hide_border: bool,
    hide_title: bool,
    disable_animations: bool,
    custom_title: Option<String>,
    default_title: String,
}

struct CardColors {
    title_color: String,
    text_color: String,
    icon_color: String,
    bg_color: String,
    border_color: Option<String>,
}

impl Card {
    fn new(
        custom_title: Option<String>,
        default_title: String,
        width: usize,
        height: usize,
        border_radius: Option<usize>,
        colors: CardColors,
    ) -> Self {
        Self {
            width,
            height,
            border_radius,
            colors,
            hide_border: false,
            hide_title: false,
            disable_animations: false,
            custom_title,
            default_title,
        }
    }

    fn set_hide_border(&mut self, hide: bool) {
        self.hide_border = hide;
    }

    fn set_hide_title(&mut self, hide: bool) {
        self.hide_title = hide;
    }

    fn disable_animations(&mut self) {
        self.disable_animations = true;
    }

    fn set_css(&self, css: &str) -> String {
        format!("<style>{}</style>", css)
    }

    fn render(&self, content: String) -> String {
        let border = if self.hide_border {
            "none".to_string()
        } else {
            format!(
                "1px solid {}",
                self.colors
                    .border_color
                    .clone()
                    .unwrap_or("#E4E2E2".to_string())
            )
        };

        let title = if self.hide_title {
            "".to_string()
        } else {
            format!(
                "<text x='25' y='35' class='title'>{}</text>",
                self.custom_title
                    .clone()
                    .unwrap_or_else(|| self.default_title.clone())
            )
        };

        format!(
            "<svg width='{}' height='{}' viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>
                <rect x='0' y='0' width='{}' height='{}' rx='{}' fill='{}' stroke='{}'/>
                {}
                {}
            </svg>",
            self.width,
            self.height,
            self.width,
            self.height,
            self.width,
            self.height,
            self.border_radius.unwrap_or(4),
            self.colors.bg_color,
            border,
            title,
            content
        )
    }
}

fn get_card_colors(
    title_color: Option<String>,
    text_color: Option<String>,
    bg_color: Option<String>,
    border_color: Option<String>,
    theme: Option<String>,
) -> CardColors {
    let default_colors = CardColors {
        title_color: "#000".to_string(),
        text_color: "#333".to_string(),
        icon_color: "#4CAF50".to_string(),
        bg_color: "#fff".to_string(),
        border_color: Some("#E4E2E2".to_string()),
    };

    CardColors {
        title_color: title_color.unwrap_or(default_colors.title_color),
        text_color: text_color.unwrap_or(default_colors.text_color),
        icon_color: default_colors.icon_color,
        bg_color: bg_color.unwrap_or(default_colors.bg_color),
        border_color,
    }
}

fn calculate_circle_length(radius: f64) -> f64 {
    2.0 * PI * radius
}

fn polar_to_cartesian(
    center_x: f64,
    center_y: f64,
    radius: f64,
    angle_in_degrees: f64,
) -> (f64, f64) {
    let angle_in_radians = angle_in_degrees * (PI / 180.0);
    (
        center_x + radius * angle_in_radians.cos(),
        center_y + radius * angle_in_radians.sin(),
    )
}

fn degrees_to_radians(angle_in_degrees: f64) -> f64 {
    angle_in_degrees * (PI / 180.0)
}

fn radians_to_degrees(angle_in_radians: f64) -> f64 {
    angle_in_radians * (180.0 / PI)
}

fn create_progress_node(
    x: f64,
    y: f64,
    color: &str,
    width: f64,
    progress: f64,
    progress_bar_background_color: &str,
    delay: f64,
) -> String {
    format!(
        "<rect x='{:.1}' y='{:.1}' width='{:.1}' height='8' fill='{}' opacity='0.2'/>
        <rect x='{:.1}' y='{:.1}' width='{:.1}' height='8' fill='{}' style='animation-delay: {:.1}ms;'/>",
        x, y, width, progress_bar_background_color, x, y, progress, color, delay
    )
}

fn get_longest_lang(langs: &[Lang]) -> &Lang {
    static DEFAULT_LANG: Lang = Lang {
        name: String::new(),
        size: 0.0,
        color: String::new(),
    };

    langs
        .iter()
        .max_by_key(|lang| lang.name.len())
        .unwrap_or(&DEFAULT_LANG)
}

fn create_progress_text_node(
    width: f64,
    color: &str,
    name: &str,
    progress: f64,
    index: usize,
) -> String {
    let stagger_delay = (index + 3) * 150;
    let padding_right = 95;
    let progress_text_x = width - padding_right as f64 + 10.0;
    let progress_width = width - padding_right as f64;

    format!(
        "<g class='stagger' style='animation-delay: {}ms'>
            <text x='2' y='15' class='lang-name'>{}</text>
            <text x='{:.1}' y='34' class='lang-name'>{:.2}%</text>
            {}
        </g>",
        stagger_delay,
        name,
        progress_text_x,
        progress,
        create_progress_node(
            0.0,
            25.0,
            color,
            progress_width,
            progress,
            "#ddd",
            stagger_delay as f64 + 300.0
        )
    )
}

fn create_compact_lang_node(
    lang: &Lang,
    total_size: f64,
    hide_progress: bool,
    index: usize,
) -> String {
    let percentage = (lang.size / total_size * 100.0).to_string();
    let stagger_delay = (index + 3) * 150;
    let color = if !lang.color.is_empty() {
        &lang.color
    } else {
        "#858585"
    };

    format!(
        "<g class='stagger' style='animation-delay: {}ms'>
            <circle cx='5' cy='6' r='5' fill='{}' />
            <text x='15' y='10' class='lang-name'>
                {} {}
            </text>
        </g>",
        stagger_delay,
        color,
        lang.name,
        if hide_progress {
            "".to_string()
        } else {
            format!("{}%", percentage)
        }
    )
}

fn render_compact_layout(
    langs: &[Lang],
    width: f64,
    total_language_size: f64,
    hide_progress: bool,
) -> String {
    let padding_right = 50.0;
    let offset_width = width - padding_right;
    let mut progress_offset = 0.0;
    let compact_progress_bar = langs
        .iter()
        .map(|lang| {
            let percentage =
                (lang.size / total_language_size * offset_width * 100.0).round() / 100.0;
            let progress = if percentage < 10.0 {
                percentage + 10.0
            } else {
                percentage
            };
            let output = format!(
                "<rect mask='url(#rect-mask)' x='{:.1}' y='0' width='{:.1}' height='8' fill='{}'/>",
                progress_offset, progress, lang.color
            );
            progress_offset += percentage;
            output
        })
        .collect::<Vec<String>>()
        .join("");

    format!(
        "{}<g transform='translate(0, {})'>{}</g>",
        if hide_progress {
            "".to_string()
        } else {
            format!(
                "<mask id='rect-mask'><rect x='0' y='0' width='{:.1}' height='8' fill='white' rx='5'/></mask>{}",
                offset_width, compact_progress_bar
            )
        },
        if hide_progress { 0 } else { 25 },
        langs
            .iter()
            .enumerate()
            .map(|(index, lang)| {
                create_compact_lang_node(lang, total_language_size, hide_progress, index)
            })
            .collect::<Vec<String>>()
            .join("")
    )
}

fn render_top_languages(
    top_langs: HashMap<String, Lang>,
    options: HashMap<String, String>,
) -> String {
    let hide_title = options.get("hide_title").map_or(false, |v| v == "true");
    let hide_border = options.get("hide_border").map_or(false, |v| v == "true");
    let card_width = options
        .get("card_width")
        .and_then(|v| v.parse::<usize>().ok());
    let title_color = options.get("title_color").cloned();
    let text_color = options.get("text_color").cloned();
    let bg_color = options.get("bg_color").cloned();
    let hide = options
        .get("hide")
        .map_or(vec![], |v| v.split(',').map(String::from).collect());
    let hide_progress = options.get("hide_progress").map_or(false, |v| v == "true");
    let theme = options.get("theme").cloned();
    let layout = options.get("layout").cloned();
    let custom_title = options.get("custom_title").cloned();
    let locale = options.get("locale").cloned();
    let langs_count = options
        .get("langs_count")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(5);
    let border_radius = options
        .get("border_radius")
        .and_then(|v| v.parse::<usize>().ok());
    let border_color = options.get("border_color").cloned();
    let disable_animations = options
        .get("disable_animations")
        .map_or(false, |v| v == "true");

    let colors = get_card_colors(title_color, text_color, bg_color, border_color, theme);

    let mut langs: Vec<Lang> = top_langs.values().cloned().collect();
    langs.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    let total_language_size: f64 = langs.iter().map(|lang| lang.size).sum();
    let layout_fn = match layout.as_deref() {
        Some("compact") => render_compact_layout,
        _ => render_compact_layout, // Default to compact layout
    };

    let height = 150; // You can adjust this based on the layout

    let mut card = Card::new(
        custom_title,
        "Top Languages".to_string(),
        card_width.unwrap_or(300),
        height,
        border_radius,
        colors,
    );

    card.set_hide_border(hide_border);
    card.set_hide_title(hide_title);
    if disable_animations {
        card.disable_animations();
    }

    let final_layout = layout_fn(
        &langs,
        card.width as f64,
        total_language_size,
        hide_progress,
    );

    card.render(final_layout)
}

pub fn test() {
    let mut top_langs = HashMap::new();
    top_langs.insert(
        "Rust".to_string(),
        Lang {
            name: "Rust".to_string(),
            size: 40.0,
            color: "#DEA584".to_string(),
        },
    );
    top_langs.insert(
        "JavaScript".to_string(),
        Lang {
            name: "JavaScript".to_string(),
            size: 30.0,
            color: "#F0DB4F".to_string(),
        },
    );
    top_langs.insert(
        "Python".to_string(),
        Lang {
            name: "Python".to_string(),
            size: 20.0,
            color: "#306998".to_string(),
        },
    );
    top_langs.insert(
        "Go".to_string(),
        Lang {
            name: "Go".to_string(),
            size: 10.0,
            color: "#00ADD8".to_string(),
        },
    );

    let mut options = HashMap::new();
    options.insert("theme".to_string(), "dark".to_string());
    options.insert("hide_title".to_string(), "false".to_string());
    options.insert("hide_border".to_string(), "false".to_string());
    options.insert("hide_progress".to_string(), "false".to_string());

    let svg = render_top_languages(top_langs, options);
    println!("{}", svg);
}
