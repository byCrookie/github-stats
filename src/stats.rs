use std::fmt::Write;

fn render_stats_card(total_stars: u32, total_commits: u32, title: &str) -> String {
    let title_color = "#fff"; // Title color for dark theme
    let text_color = "#9f9f9f"; // Text color for dark theme
    let icon_color = "#79ff97"; // Icon color for dark theme
    let bg_color = "#151515"; // Background color for dark theme

    let css_styles = format!(
        "
        .title {{
            font: 600 18px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {};
        }}
        .stat {{
            font: 600 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {};
        }}
        .icon {{
            fill: {};
            display: block;
        }}
    ",
        title_color, text_color, icon_color
    );

    let stars_text_node = create_text_node("⭐", "Total Stars", total_stars, 0);
    let commits_text_node = create_text_node("🔄", "Total Commits", total_commits, 1);

    let svg = format!(
        "
        <svg width='300' height='150' viewBox='0 0 300 150' xmlns='http://www.w3.org/2000/svg'>
            <style>
                {}
            </style>
            <rect x='0' y='0' width='300' height='150' rx='4.5' fill='{}' stroke='#E4E2E2'/>
            <text x='25' y='30' class='title'>{}</text>
            <g transform='translate(25, 50)'>
                {}
                {}
            </g>
        </svg>
    ",
        css_styles, bg_color, title, stars_text_node, commits_text_node
    );

    svg
}

fn create_text_node(icon: &str, label: &str, value: u32, index: usize) -> String {
    let y_position = 25 * index as i32; // Adjust the y position based on the index
    format!(
        "
        <g class='stagger' transform='translate(0, {})'>
            <text class='stat' x='25' y='12.5'>{} {}</text>
            <text class='stat' x='140' y='12.5'>{}</text>
        </g>
    ",
        y_position, icon, label, value
    )
}

pub fn test() {
    let total_stars = 1234;
    let total_commits = 5678;
    let title = "GitHub Stats";

    let svg = render_stats_card(total_stars, total_commits, title);
    println!("{}", svg);
}
