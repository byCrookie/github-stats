use actix_web::web::ReqData;
use log::debug;

use crate::{
    icons::{icon_commits, icon_star},
    themes::Theme,
};
use std::io::Write;

fn format_number(num: u32) -> String {
    if num < 1000 {
        return num.to_string();
    } else if num < 1000000 {
        format!("{:.1}k", num as f32 / 1000.0)
    } else {
        format!("{:.1}M", num as f32 / 1000000.0)
    }
}

pub fn render_stats_card(total_stars: u32, total_commits: u32, title: &str) -> (u64, String) {
    let stars_text_node = create_text_node(&icon_star(), "Total Stars", total_stars, 0);
    let commits_text_node = create_text_node(&icon_commits(), "Total Commits", total_commits, 1);
    let nodes: u64 = 2;
    let width: u64 = 300;
    let height: u64 = nodes * 41 + 28;
    let theme: Theme = crate::themes::dark();
    let title_color = theme.title_color;
    let text_color = theme.text_color;
    let icon_color = theme.icon_color;
    let bg_color = theme.background_color;
    let border_color = theme.border_color;

    let css_styles = format!(
        r#"
.title {{
    font: 600 18px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color};
}}
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
<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg'>
    <style>
        {css_styles}
    </style>
    <rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{bg_color}' stroke='{border_color}'/>
    <text x='25' y='30' class='title'>{title}</text>
    <g transform='translate(25, 50)'>
        {stars_text_node}
        {commits_text_node}
    </g>
</svg>
    "#
    );

    (height, svg)
}

fn create_text_node(icon: &str, label: &str, value: u32, index: usize) -> String {
    let y_position = 25 * index as i32;
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

pub fn test() {
    let svg = render_stats_card(34, 1234, "Stats");
    debug!("{}", svg.1);
    let mut file = std::fs::File::create("stats.svg").unwrap();
    write!(&mut file, "{}", svg.1).unwrap();
}