use std::io::Write;

use crate::themes::Theme;

pub struct Part {
    pub height: f64,
    pub content: String,
}

pub fn render_card(
    parts: Vec<Part>,
    x_offset: f64,
    y_offset: f64,
    gap: f64,
    width: f64,
    title: &str,
) -> String {
    let theme: Theme = crate::themes::dark();
    let title_height: f64 = 18.0;
    let height: f64 = parts.len() as f64 * gap
        + parts.iter().map(|p| p.height).sum::<f64>()
        + y_offset
        + title_height
        + y_offset / 2.0;
    let title_color: String = theme.title_color;
    let background_color: String = theme.background_color;
    let border_color: String = theme.border_color;

    let mut part_offset: f64 = title_height;
    let mut translated_parts = String::new();
    for part in parts.iter() {
        let part_content = &part.content;
        translated_parts.push_str(&format!(
            r#"
<g transform='translate(0, {part_offset})'>
    {part_content}
</g>
            "#
        ));
        part_offset = part_offset + gap + &part.height;
    }

    return format!(
        r#"
<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg'>
    <rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{background_color}' stroke='{border_color}'/>
    <g transform='translate({x_offset}, {y_offset})'>
        <text style="font: 600 {title_height}px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color};">{title}</text>
        {translated_parts}
    </g>
</svg>
    "#
    );
}

pub fn test() {
    let part1 = Part {
        height: 100.0,
        content: format!(
            r#"<div style="height: 100px; background-color: red; opacity: 0.5;"></div>"#
        ),
    };

    let part2 = Part {
        height: 150.0,
        content: format!(
            r#"<div style="height: 150px; background-color: red; opacity: 0.5;"></div>"#
        ),
    };

    let width: f64 = 300.0;
    let x_offset: f64 = 25.0;
    let y_offset: f64 = 35.0;
    let gap: f64 = 20.0;

    let svg = render_card(vec![part1, part2], x_offset, y_offset, gap, width, "Stats");
    let mut file = std::fs::File::create("card.svg").unwrap();
    write!(&mut file, "{}", svg).unwrap();
}
