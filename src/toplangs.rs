use crate::themes::Theme;
use log::debug;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt::format;
use std::{
    fs,
    io::{self, Error, Read, Write},
    path::Path,
    time::{Duration, SystemTime},
};

#[derive(Debug, Clone)]
struct Lang {
    name: String,
    size: f64,
    color: String,
}

fn degrees_to_radians(angle_in_degrees: f64) -> f64 {
    angle_in_degrees * (std::f64::consts::PI / 180.0)
}

fn radians_to_degrees(angle_in_radians: f64) -> f64 {
    angle_in_radians / (std::f64::consts::PI / 180.0)
}

fn polar_to_cartesian(
    center_x: f64,
    center_y: f64,
    radius: f64,
    angle_in_degrees: f64,
) -> (f64, f64) {
    let rads = degrees_to_radians(angle_in_degrees);
    let x = center_x + radius * rads.cos();
    let y = center_y + radius * rads.sin();
    (x, y)
}

fn cartesian_to_polar(center_x: f64, center_y: f64, x: f64, y: f64) -> (f64, f64) {
    let radius = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
    let mut angle_in_degrees = radians_to_degrees((y - center_y).atan2(x - center_x));
    if angle_in_degrees < 0.0 {
        angle_in_degrees += 360.0;
    }
    (radius, angle_in_degrees)
}

fn get_circle_length(radius: f64) -> f64 {
    2.0 * std::f64::consts::PI * radius
}

fn calculate_normal_layout_height(total_langs: usize) -> f64 {
    45.0 + (total_langs + 1) as f64 * 40.0
}

fn trim_top_languages(
    top_langs: HashMap<String, Lang>,
    langs_count: usize,
    hide: Option<Vec<String>>,
) -> (Vec<Lang>, f64) {
    let mut langs: Vec<Lang> = top_langs.into_values().collect();
    let langs_count = langs_count.clamp(1, 20);

    let mut langs_to_hide = std::collections::HashSet::new();
    if let Some(hide) = hide {
        for lang in hide {
            langs_to_hide.insert(lang.trim().to_lowercase());
        }
    }

    langs.retain(|lang| !langs_to_hide.contains(&lang.name.trim().to_lowercase()));
    langs.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    let langs = langs.into_iter().take(langs_count).collect::<Vec<_>>();
    let total_language_size = langs.iter().map(|lang| lang.size).sum();

    (langs, total_language_size)
}

fn render_normal_layout(langs: Vec<Lang>, width: f64, total_language_size: f64) -> String {
    let mut layout = String::new();

    for (index, lang) in langs.iter().enumerate() {
        let progress = (lang.size / total_language_size * 100.0).round();
        let color = &lang.color;
        let name = &lang.name;
        layout.push_str(&format!(
            r#"<g class="stagger" style="animation-delay: {}ms">
                <text x="2" y="15" class="lang-name">{}</text>
                <text x="{}" y="34" class="lang-name">{:.2}%</text>
                <rect x="0" y="25" width="{}" height="8" fill="{}" class="lang-progress" />
            </g>"#,
            (index + 3) * 150,
            name,
            width - 95.0 + 10.0,
            progress,
            width - 95.0,
            color
        ));
    }

    layout
}

fn render_top_languages(top_langs: HashMap<String, Lang>, title: &str) -> String {
    let theme: Theme = crate::themes::dark();
    let text_color = theme.text_color;
    let background_color = theme.background_color;
    let title_color = theme.title_color;

    let langs_count = 5; // Adjust as necessary
    let (langs, total_language_size) = trim_top_languages(top_langs, langs_count, None);
    let width = 300.0; // Default card width
    let height = calculate_normal_layout_height(langs.len());

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
            <style>
                .title {{
                    font: 600 18px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color};
                }}
                @keyframes slideInAnimation {{
                    from {{ width: 0; }}
                    to {{ width: calc(100% - 100px); }}
                }}
                @keyframes growWidthAnimation {{
                    from {{ width: 0; }}
                    to {{ width: 100%; }}
                }}
                .stat {{
                    font: 600 14px 'Segoe UI', Ubuntu, "Helvetica Neue", Sans-Serif;
                    fill: {text_color};
                }}
                @supports(-moz-appearance: auto) {{
                    .stat {{ font-size: 12px; }}
                }}
                .bold {{ font-weight: 700; }}
                .lang-name {{
                    font: 400 11px "Segoe UI", Ubuntu, Sans-Serif;
                    fill: {text_color};
                }}
                .stagger {{
                    opacity: 0;
                    animation: fadeInAnimation 0.3s ease-in-out forwards;
                }}
                .lang-progress {{
                    animation: growWidthAnimation 0.6s ease-in-out forwards;
                }}
            </style>"#
    ));

    svg.push_str(&format!("<text x='25' y='30' class='title'>{title}</text>\n"));
    svg.push_str(&format!("<rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{background_color}' stroke='{title_color}'/>\n"));
    svg.push_str(&render_normal_layout(langs, width, total_language_size));
    svg.push_str("\n</svg>");
    svg
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

    let svg = render_top_languages(top_langs, "Top Languages");
    debug!("{}", svg);
    let mut file = std::fs::File::create("top_langs.svg").unwrap();
    write!(&mut file, "{}", svg).unwrap();
}
