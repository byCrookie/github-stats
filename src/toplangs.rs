use std::collections::HashMap;

use crate::card::Part;
use crate::github::Language;
use crate::themes::Theme;

/// Escapes characters that are special in XML/SVG text content and attribute values.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Computes the total rendered height of the top-languages section.
///
/// The section consists of:
/// - an 8 px progress bar
/// - `gap` px of spacing before the language list
/// - `ceil(langs / columns)` rows each `gap` px tall
fn calculate_height(langs: usize, gap: f64, columns: u64) -> f64 {
    const BAR_HEIGHT: f64 = 8.0;
    let rows = (langs as f64 / columns as f64).ceil();
    BAR_HEIGHT + gap + rows * gap
}

fn select_top_languages(
    langs: &HashMap<String, Language>,
    langs_count: usize,
) -> (Vec<Language>, f64) {
    let mut langs: Vec<Language> = langs.values().cloned().collect();
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
    langs: &[Language],
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
        let name = xml_escape(&lang.name);
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
    langs: &HashMap<String, Language>,
    lang_count: usize,
) -> Part {
    let text_color = &theme.text_color;
    let title_color = &theme.title_color;
    let (langs, total_language_size) = select_top_languages(langs, lang_count);

    if langs.is_empty() {
        return Part {
            height: 0.0,
            content: String::new(),
        };
    }

    let columns: u64 = if langs.len() > 4 { 2 } else { 1 };
    let card_width: f64 = if langs.len() > 4 { width } else { width / 2.0 };
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
        card_width,
        total_language_size,
    ));
    svg.push_str(&format!(
        r#"<g transform="translate(0, {})">{}</g>"#,
        gap,
        render_normal_layout(langs, total_language_size, gap, columns)
    ));
    svg.push_str("\n</svg>");

    Part {
        height,
        content: svg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lang(name: &str, size: f64) -> Language {
        Language {
            name: name.to_string(),
            color: "#ff0000".to_string(),
            size,
        }
    }

    #[test]
    fn calculate_height_single_column() {
        // 3 langs, 1 column, gap=25: 8 + 25 + 3*25 = 108
        assert_eq!(calculate_height(3, 25.0, 1), 8.0 + 25.0 + 3.0 * 25.0);
    }

    #[test]
    fn calculate_height_two_columns() {
        // 6 langs, 2 columns → 3 rows: 8 + 25 + 3*25 = 108
        assert_eq!(calculate_height(6, 25.0, 2), 8.0 + 25.0 + 3.0 * 25.0);
    }

    #[test]
    fn calculate_height_odd_two_columns() {
        // 5 langs, 2 columns → ceil(5/2)=3 rows: 8 + 25 + 3*25 = 108
        assert_eq!(calculate_height(5, 25.0, 2), 8.0 + 25.0 + 3.0 * 25.0);
    }

    #[test]
    fn select_top_languages_sorts_by_size() {
        let mut langs = HashMap::new();
        langs.insert("Rust".to_string(), make_lang("Rust", 100.0));
        langs.insert("Python".to_string(), make_lang("Python", 500.0));
        langs.insert("C".to_string(), make_lang("C", 200.0));

        let (sorted, total) = select_top_languages(&langs, 10);
        assert_eq!(sorted[0].name, "Python");
        assert_eq!(sorted[1].name, "C");
        assert_eq!(sorted[2].name, "Rust");
        assert_eq!(total, 800.0);
    }

    #[test]
    fn select_top_languages_clamps_count() {
        let mut langs = HashMap::new();
        for i in 0..10 {
            langs.insert(
                format!("Lang{i}"),
                make_lang(&format!("Lang{i}"), i as f64 * 100.0),
            );
        }
        let (result, _) = select_top_languages(&langs, 3);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn select_top_languages_empty_map() {
        let langs = HashMap::new();
        let (result, total) = select_top_languages(&langs, 10);
        assert!(result.is_empty());
        assert_eq!(total, 0.0);
    }

    #[test]
    fn render_top_languages_empty_returns_empty_part() {
        let theme = crate::themes::dark();
        let langs = HashMap::new();
        let part = render_top_languages(&theme, 25.0, 300.0, &langs, 10);
        assert_eq!(part.height, 0.0);
        assert!(part.content.is_empty());
    }

    #[test]
    fn render_top_languages_produces_svg() {
        let theme = crate::themes::dark();
        let mut langs = HashMap::new();
        langs.insert("Rust".to_string(), make_lang("Rust", 1000.0));
        langs.insert("Python".to_string(), make_lang("Python", 2000.0));
        let part = render_top_languages(&theme, 25.0, 300.0, &langs, 10);
        assert!(part.content.contains("<svg"));
        assert!(part.content.contains("Rust"));
        assert!(part.height > 0.0);
    }

    #[test]
    fn xml_escape_special_chars() {
        assert_eq!(xml_escape("Rust"), "Rust");
        assert_eq!(xml_escape("C++"), "C++");
        assert_eq!(xml_escape("<script>"), "&lt;script&gt;");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
        assert_eq!(xml_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(xml_escape("it's"), "it&#39;s");
    }

    #[test]
    fn render_top_languages_escapes_name_in_svg() {
        let theme = crate::themes::dark();
        let mut langs = HashMap::new();
        langs.insert("bad".to_string(), make_lang("<b>xss</b>", 1000.0));
        let part = render_top_languages(&theme, 25.0, 300.0, &langs, 10);
        assert!(!part.content.contains("<b>xss</b>"));
        assert!(part.content.contains("&lt;b&gt;xss&lt;/b&gt;"));
    }
}
