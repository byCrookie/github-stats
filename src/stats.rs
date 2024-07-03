use crate::icons::{icon_commits, icon_star};

fn render_stats_card(total_stars: u32, total_commits: u32, title: &str) -> String {
    let stars_text_node = create_text_node(&icon_star(), "Total Stars", total_stars, 0);
    let commits_text_node = create_text_node(&icon_commits(), "Total Commits", total_commits, 1);
    let nodes: u64 = 2;

    let title_color = "#fff";
    let text_color = "#9f9f9f";
    let icon_color = "#79ff97";
    let bg_color = "#151515";
    let width: u64 = 250;
    let height: u64 = nodes * 41 + 28;

    let css_styles = format!(
        "
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
    "
    );

    let svg = format!(
        "
        <svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg'>
            <style>
                {css_styles}
            </style>
            <rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{bg_color}' stroke='{title_color}'/>
            <text x='25' y='30' class='title'>{title}</text>
            <g transform='translate(25, 50)'>
                {stars_text_node}
                {commits_text_node}
            </g>
        </svg>
    ");

    svg
}

fn create_text_node(icon: &str, label: &str, value: u32, index: usize) -> String {
    let y_position = 25 * index as i32;
    let stagger_delay = (index + 3) * 150;
    format!(
        "
        <g class='stagger' style='animation-delay: {stagger_delay}ms' transform='translate(0, {y_position})'>
            <svg class='icon' viewBox='0 0 16 16' version='1.1' width='16' height='16'>
                {icon}
            </svg>
            <text class='stat' x='25' y='12.5'>{label}</text>
            <text class='stat' x='140' y='12.5'>{value}</text>
        </g>
    ")
}

pub fn test() {
    let total_stars = 1234;
    let total_commits = 5678;
    let title = "GitHub Stats";

    let svg = render_stats_card(total_stars, total_commits, title);
    println!("{:#?}", svg);
}
