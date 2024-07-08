use crate::card::Part;
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

fn calculate_height(langs: usize, gap: f64, columns: u64) -> f64 {
    50.0 + langs as f64 * (gap / columns as f64) + 25.0
}

fn select_top_languages(langs: HashMap<String, Lang>, langs_count: usize) -> (Vec<Lang>, f64) {
    let mut langs: Vec<Lang> = langs.into_values().collect();
    let langs_count = langs_count.clamp(1, 20);
    langs.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    let langs = langs.into_iter().take(langs_count).collect::<Vec<_>>();
    let total_languages_size = langs.iter().map(|lang| lang.size).sum();
    (langs, total_languages_size)
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

fn render_percent_bar(
    langs: &Vec<Lang>,
    x_offset: f64,
    width: f64,
    total_language_size: f64,
) -> String {
    let width_without_offset: f64 = width - 2.0 * x_offset;
    let mut progress_offset: f64 = 0.0;

    let progress_bar = langs
        .iter()
        .map(|lang| {
            let lang_color = &lang.color;
            let percentage = ((lang.size / total_language_size) * width_without_offset).round();
            let output = format!(
                r#"<rect
                    mask="url(#rect-mask)"
                    x="{progress_offset}"
                    y="0"
                    width="{percentage}"
                    height="8"
                    fill="{lang_color}"
                    class="lang-progress"
                />"#
            );
            progress_offset += percentage;
            output
        })
        .collect::<Vec<String>>()
        .join("");

    let mask = format!(
        r#"
<mask id="rect-mask">
    <rect width="{width_without_offset}px" height="8" fill="white" rx="5"/>
</mask>
    "#
    );

    format!(r#"<svg width="{width_without_offset}px">{mask}{progress_bar}</svg>"#)
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

pub fn render_top_languages(
    theme: &Theme,
    x_offset: f64,
    width: f64,
    langs: HashMap<String, Lang>,
    lang_count: usize,
) -> Part {
    let text_color = &theme.text_color;
    let title_color = &theme.title_color;
    let (langs, total_language_size) = select_top_languages(langs, lang_count);

    if langs.len() == 0 {
        return Part {
            height: 0.0,
            content: String::new()
        }
    }

    let columns: u64 = if langs.len() > 4 { 2 } else { 1 };
    let width: f64 = if langs.len() > 4 { width } else { width / 2.0 };
    let gap = 25.0;
    let height = calculate_height(langs.len(), gap, columns);

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg">
            <style>
                .title {{
                    font: 600 18px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color};
                }}
                @keyframes slideInAnimation {{
                    from {{ width: 0; }}
                    to {{ width: 100%; }}
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

    svg.push_str(&render_percent_bar(
        &langs,
        x_offset,
        width,
        total_language_size,
    ));
    svg.push_str(&format!(
        r#"<g transform="translate(0, {})">{}</g>"#,
        gap,
        render_normal_layout(langs, total_language_size, gap, columns)
    ));
    svg.push_str("\n</svg>");

    return Part {
        height,
        content: svg,
    };
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

    let theme: Theme = crate::themes::dark();
    let x_offset: f64 = 25.0;
    let width: f64 = 300.0;
    let part = render_top_languages(&theme, x_offset, width, top_langs, 20);
    let mut file = std::fs::File::create("toplangs.svg").unwrap();
    write!(&mut file, "{}", part.content).unwrap();
}
