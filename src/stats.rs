use std::io::Write;

use crate::{
    card::Part,
    icons::{icon_commits, icon_star},
    themes::Theme,
};

fn format_number(num: u32) -> String {
    if num < 1000 {
        return num.to_string();
    } else if num < 1000000 {
        format!("{:.1}k", num as f32 / 1000.0)
    } else {
        format!("{:.1}M", num as f32 / 1000000.0)
    }
}

pub fn render_stats(theme: &Theme, total_stars: u32, total_commits: u32) -> Part {
    let stars_text_node: String = create_text_node(&icon_star(), "Total Stars", total_stars, 0);
    let commits_text_node: String =
        create_text_node(&icon_commits(), "Total Commits", total_commits, 1);
    let nodes: f64 = 2.0;
    let height: f64 = nodes * 16.0;
    let text_color = &theme.text_color;
    let icon_color = &theme.icon_color;

    let css_styles = format!(
        r#"
.stat {{
    font: 600 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {text_color};
}}
@supports(-moz-appearance: auto) {{
    /* Selector detects Firefox */
    .stat {{ font-size:12px; }}
}}
.stagger {{
    opacity: 0;
    animation: fadeInAnimation 0.3s ease-in-out forwards;
}}
.icon {{
    fill: {icon_color};
    display: block;
}}
@keyframes fadeInAnimation {{
    to {{
        opacity: 1;
    }}
}}
    "#
    );

    let svg = format!(
        r#"
<svg xmlns='http://www.w3.org/2000/svg'>
    <style>
        {css_styles}
    </style>
    {stars_text_node}
    {commits_text_node}
</svg>
    "#
    );

    return Part {
        height,
        content: svg,
    };
}

fn create_text_node(icon: &str, label: &str, value: u32, index: u64) -> String {
    let y_position = 25 * index;
    let stagger_delay = (index + 3) * 150;
    let formatted_value = format_number(value);
    format!(
        r#"
<g class='stagger' style='animation-delay: {stagger_delay}ms' transform='translate(0, {y_position})'>
    <svg class='icon' viewBox='0 0 16 16' version='1.1' width='16' height='16'>
        {icon}
    </svg>
    <text class='stat' x='25' y='12.5'>{label}</text>
    <text class='stat' x='140' y='12.5'>{formatted_value}</text>
</g>
    "#
    )
}

#[allow(dead_code)]
pub fn test() {
    let theme: Theme = crate::themes::dark();
    let x_offset: f64 = 25.0;
    let y_offset: f64 = 35.0;
    let gap: f64 = 20.0;
    let title: &str = "Stats";
    let width: f64 = 300.0;
    let part = render_stats(&theme, 34, 1234);
    let rendered_card = crate::card::render_card(vec![part], x_offset, y_offset, gap, width, title);

    let mut file = std::fs::File::create("stats.svg").unwrap();
    write!(&mut file, "{}", rendered_card).unwrap();
}
