use crate::themes::Theme;
use log::debug;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Lang {
    pub name: String,
    pub size: f64,
    pub color: String,
}

fn calculate_height(total_langs: usize, gap: f64, columns: u64) -> f64 {
    50.0 + total_langs as f64 * (gap / columns as f64) + 25.0
}

fn trim_top_languages(top_langs: HashMap<String, Lang>, langs_count: usize) -> (Vec<Lang>, f64) {
    let mut langs: Vec<Lang> = top_langs.into_values().collect();
    let langs_count = langs_count.clamp(1, 20);
    langs.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    let langs = langs.into_iter().take(langs_count).collect::<Vec<_>>();
    let total_language_size = langs.iter().map(|lang| lang.size).sum();

    (langs, total_language_size)
}

fn flex_layout(items: Vec<String>, gap: f64, columns: u64) -> String {
    let mut layout = String::new();
    let translate = |i: usize, gap: f64| format!("translate(0, {})", i as f64 * gap);

    if columns == 1 {
        for (i, item) in items.iter().enumerate() {
            layout.push_str(&format!(
                r#"<g transform="{}">{}</g>"#,
                translate(i, gap),
                item
            ));
        }
    } else {
        let half = (items.len() as f64 / columns as f64).ceil();
        for (i, item) in items.iter().enumerate() {
            let ii = i as f64;
            let (x, y) = if ii < half {
                (0, ii * gap)
            } else {
                (150, (ii - half) * gap)
            };
            layout.push_str(&format!(
                r#"<g transform="translate({}, {})">{}</g>"#,
                x, y, item
            ));
        }
    }

    layout
}

fn render_percent_bar(langs: &Vec<Lang>, width: f64, total_language_size: f64) -> String {
    let padding_right = 100.0;
    let offset_width = width - padding_right;
    let mut progress_offset = 0.0;

    let progress_bar = langs
        .iter()
        .map(|lang| {
            let percentage = ((lang.size / total_language_size) * offset_width).round();
            let output = format!(
                r#"<rect
                    mask="url(#rect-mask)"
                    x="{}"
                    y="0"
                    width="{}"
                    height="8"
                    fill="{}"
                    class="lang-progress"
                />"#,
                progress_offset, percentage, lang.color
            );
            progress_offset += percentage;
            output
        })
        .collect::<Vec<String>>()
        .join("");

    let mask = format!(
        r#"
<mask id="rect-mask">
    <rect x="0" y="0" width="{offset_width}" height="8" fill="white" rx="5"/>
</mask>
    "#
    );

    format!(r#"<svg x="0" y="0">{}{}</svg>"#, mask, progress_bar)
}

fn render_normal_layout(
    langs: Vec<Lang>,
    total_language_size: f64,
    gap: f64,
    columns: u64,
) -> String {
    let mut items = vec![];

    for (index, lang) in langs.iter().enumerate() {
        let percent = (lang.size / total_language_size * 100.0).round();
        let color = &lang.color;
        let name = &lang.name;
        items.push(format!(
            r#"<g class="stagger" style="animation-delay: {}ms">
                <circle cx="5" cy="6" r="5" fill="{}" />
                <text x="15" y="10" class='lang-name'>{} {:.2}%</text>
            </g>"#,
            (index + 3) * 150 / 2,
            color,
            name,
            percent,
        ));
    }

    flex_layout(items, gap, columns)
}

pub fn render_top_languages(top_langs: HashMap<String, Lang>) -> (f64, String) {
    let theme: Theme = crate::themes::dark();
    let text_color = theme.text_color;
    let background_color = theme.background_color;
    let title_color = theme.title_color;

    let langs_count = 25; // Adjust as necessary
    let (langs, total_language_size) = trim_top_languages(top_langs, langs_count);
    let columns: u64 = if langs.len() > 4 { 2 } else { 1 };
    let gap = 25.0;
    let width = 300.0; // Default card width
    let height = calculate_height(langs.len(), gap, columns);

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
                @keyframes fadeInAnimation {{
                    from {{ opacity: 0; }}
                    to {{ opacity: 1; }}
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
                #rect-mask rect {{
                    animation: slideInAnimation 1s ease-in-out forwards;
                }}
            </style>"#
    ));

    svg.push_str(&format!("<rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{background_color}' stroke='{title_color}'/>\n"));
    svg.push_str(&format!(
        r#"<g transform="translate(25, 25)">{}</g>"#,
        render_percent_bar(&langs, width, total_language_size)
    ));
    svg.push_str(&format!(
        r#"<g transform="translate(25, 50)">{}</g>"#,
        render_normal_layout(langs, total_language_size, gap, columns)
    ));
    svg.push_str("\n</svg>");
    (height, svg)
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
    top_langs.insert(
        "Java".to_string(),
        Lang {
            name: "Java".to_string(),
            size: 35.0,
            color: "#b07219".to_string(),
        },
    );
    top_langs.insert(
        "C".to_string(),
        Lang {
            name: "C".to_string(),
            size: 25.0,
            color: "#555555".to_string(),
        },
    );
    top_langs.insert(
        "C++".to_string(),
        Lang {
            name: "C++".to_string(),
            size: 27.0,
            color: "#f34b7d".to_string(),
        },
    );
    top_langs.insert(
        "TypeScript".to_string(),
        Lang {
            name: "TypeScript".to_string(),
            size: 18.0,
            color: "#2b7489".to_string(),
        },
    );
    top_langs.insert(
        "Ruby".to_string(),
        Lang {
            name: "Ruby".to_string(),
            size: 15.0,
            color: "#701516".to_string(),
        },
    );
    top_langs.insert(
        "Swift".to_string(),
        Lang {
            name: "Swift".to_string(),
            size: 12.0,
            color: "#ffac45".to_string(),
        },
    );
    top_langs.insert(
        "Kotlin".to_string(),
        Lang {
            name: "Kotlin".to_string(),
            size: 22.0,
            color: "#A97BFF".to_string(),
        },
    );
    top_langs.insert(
        "PHP".to_string(),
        Lang {
            name: "PHP".to_string(),
            size: 19.0,
            color: "#4F5D95".to_string(),
        },
    );
    top_langs.insert(
        "Perl".to_string(),
        Lang {
            name: "Perl".to_string(),
            size: 14.0,
            color: "#0298c3".to_string(),
        },
    );
    top_langs.insert(
        "Scala".to_string(),
        Lang {
            name: "Scala".to_string(),
            size: 16.0,
            color: "#c22d40".to_string(),
        },
    );
    top_langs.insert(
        "Haskell".to_string(),
        Lang {
            name: "Haskell".to_string(),
            size: 13.0,
            color: "#5e5086".to_string(),
        },
    );
    top_langs.insert(
        "Elixir".to_string(),
        Lang {
            name: "Elixir".to_string(),
            size: 11.0,
            color: "#6e4a7e".to_string(),
        },
    );
    top_langs.insert(
        "Clojure".to_string(),
        Lang {
            name: "Clojure".to_string(),
            size: 9.0,
            color: "#db5855".to_string(),
        },
    );
    top_langs.insert(
        "Lua".to_string(),
        Lang {
            name: "Lua".to_string(),
            size: 8.0,
            color: "#000080".to_string(),
        },
    );
    top_langs.insert(
        "Dart".to_string(),
        Lang {
            name: "Dart".to_string(),
            size: 7.0,
            color: "#00B4AB".to_string(),
        },
    );
    top_langs.insert(
        "R".to_string(),
        Lang {
            name: "R".to_string(),
            size: 5.0,
            color: "#198CE7".to_string(),
        },
    );
    top_langs.insert(
        "MATLAB".to_string(),
        Lang {
            name: "MATLAB".to_string(),
            size: 6.0,
            color: "#e16737".to_string(),
        },
    );

    let svg = render_top_languages(top_langs);
    debug!("{}", svg.1);
    let mut file = std::fs::File::create("top_langs.svg").unwrap();
    write!(&mut file, "{}", svg.1).unwrap();
}
