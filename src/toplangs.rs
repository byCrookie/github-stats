use crate::card::Part;
use crate::github::Language;
use crate::themes::Theme;
use std::collections::HashMap;
use std::io::Write;

fn calculate_height(langs: usize, gap: f64, columns: u64) -> f64 {
    langs as f64 * (gap / columns as f64)
}

fn select_top_languages(
    langs: HashMap<String, Language>,
    langs_count: usize,
) -> (Vec<Language>, f64) {
    let mut langs: Vec<Language> = langs.into_values().collect();
    let langs_count = langs_count.clamp(0, 100);
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
    langs: &Vec<Language>,
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
    langs: Vec<Language>,
    total_language_size: f64,
    gap: f64,
    columns: u64,
) -> String {
    let mut items = vec![];

    for (index, lang) in langs.iter().enumerate() {
        let percent = lang.size / total_language_size * 100.0;
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
    langs: HashMap<String, Language>,
    lang_count: usize,
) -> Part {
    let text_color = &theme.text_color;
    let title_color = &theme.title_color;
    let (langs, total_language_size) = select_top_languages(langs, lang_count);

    if langs.len() == 0 {
        return Part {
            height: 0.0,
            content: String::new(),
        };
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
        "Dockerfile".to_string(),
        Language {
            name: "Dockerfile".to_string(),
            size: 11492.0,
            color: "#384d54".to_string(),
        },
    );
    top_langs.insert(
        "Jupyter Notebook".to_string(),
        Language {
            name: "Jupyter Notebook".to_string(),
            size: 1246588.0,
            color: "#DA5B0B".to_string(),
        },
    );
    top_langs.insert(
        "Vue".to_string(),
        Language {
            name: "Vue".to_string(),
            size: 165370.0,
            color: "#41b883".to_string(),
        },
    );
    top_langs.insert(
        "HTML".to_string(),
        Language {
            name: "HTML".to_string(),
            size: 1879495.0,
            color: "#e34c26".to_string(),
        },
    );
    top_langs.insert(
        "Java".to_string(),
        Language {
            name: "Java".to_string(),
            size: 1117778.0,
            color: "#b07219".to_string(),
        },
    );
    top_langs.insert(
        "Shell".to_string(),
        Language {
            name: "Shell".to_string(),
            size: 18528.0,
            color: "#89e051".to_string(),
        },
    );
    top_langs.insert(
        "CMake".to_string(),
        Language {
            name: "CMake".to_string(),
            size: 1177.0,
            color: "#DA3434".to_string(),
        },
    );
    top_langs.insert(
        "JavaScript".to_string(),
        Language {
            name: "JavaScript".to_string(),
            size: 350724.0,
            color: "#f1e05a".to_string(),
        },
    );
    top_langs.insert(
        "Rust".to_string(),
        Language {
            name: "Rust".to_string(),
            size: 54467.0,
            color: "#dea584".to_string(),
        },
    );
    top_langs.insert(
        "SCSS".to_string(),
        Language {
            name: "SCSS".to_string(),
            size: 7550.0,
            color: "#c6538c".to_string(),
        },
    );
    top_langs.insert(
        "Svelte".to_string(),
        Language {
            name: "Svelte".to_string(),
            size: 1303.0,
            color: "#ff3e00".to_string(),
        },
    );
    top_langs.insert(
        "CSS".to_string(),
        Language {
            name: "CSS".to_string(),
            size: 76630.0,
            color: "#563d7c".to_string(),
        },
    );
    top_langs.insert(
        "C#".to_string(),
        Language {
            name: "C#".to_string(),
            size: 3190695.0,
            color: "#178600".to_string(),
        },
    );
    top_langs.insert(
        "Astro".to_string(),
        Language {
            name: "Astro".to_string(),
            size: 18638.0,
            color: "#ff5a03".to_string(),
        },
    );
    top_langs.insert(
        "PHP".to_string(),
        Language {
            name: "PHP".to_string(),
            size: 124099.0,
            color: "#4F5D95".to_string(),
        },
    );
    top_langs.insert(
        "PLpgSQL".to_string(),
        Language {
            name: "PLpgSQL".to_string(),
            size: 4217.0,
            color: "#336790".to_string(),
        },
    );
    top_langs.insert(
        "TypeScript".to_string(),
        Language {
            name: "TypeScript".to_string(),
            size: 464381.0,
            color: "#3178c6".to_string(),
        },
    );
    top_langs.insert(
        "Makefile".to_string(),
        Language {
            name: "Makefile".to_string(),
            size: 20847.0,
            color: "#427819".to_string(),
        },
    );
    top_langs.insert(
        "C".to_string(),
        Language {
            name: "C".to_string(),
            size: 344871.0,
            color: "#555555".to_string(),
        },
    );
    top_langs.insert(
        "PowerShell".to_string(),
        Language {
            name: "PowerShell".to_string(),
            size: 9618.0,
            color: "#012456".to_string(),
        },
    );
    top_langs.insert(
        "Batchfile".to_string(),
        Language {
            name: "Batchfile".to_string(),
            size: 6331.0,
            color: "#C1F12E".to_string(),
        },
    );
    top_langs.insert(
        "Python".to_string(),
        Language {
            name: "Python".to_string(),
            size: 598997.0,
            color: "#3572A5".to_string(),
        },
    );

    let theme: Theme = crate::themes::dark();
    let x_offset: f64 = 25.0;
    let y_offset: f64 = 35.0;
    let gap: f64 = 20.0;
    let title: &str = "Top Languages";
    let width: f64 = 300.0;
    let part = render_top_languages(&theme, x_offset, width, top_langs, 40);
    let rendered_card = crate::card::render_card(vec![part], x_offset, y_offset, gap, width, title);

    let mut file = std::fs::File::create("toplangs.svg").unwrap();
    write!(&mut file, "{}", rendered_card).unwrap();
}
