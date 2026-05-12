use crate::{
    card::Part,
    icons::{icon_commits, icon_star},
    themes::Theme,
};

const ROW_HEIGHT: f64 = 20.0;
const ROW_GAP: f64 = 5.0;

fn format_number(num: u32) -> String {
    if num < 1000 {
        num.to_string()
    } else if num < 1_000_000 {
        format!("{:.1}k", num as f32 / 1000.0)
    } else {
        format!("{:.1}M", num as f32 / 1_000_000.0)
    }
}

pub fn render_stats(
    theme: &Theme,
    total_stars: u32,
    total_commits: u32,
    content_width: f64,
) -> Part {
    let nodes: f64 = 2.0;
    let height: f64 = nodes * ROW_HEIGHT + (nodes - 1.0) * ROW_GAP;
    let text_color = &theme.text_color;
    let title_color = &theme.title_color;
    let icon_color = &theme.icon_color;

    let stars_text_node =
        create_text_node(&icon_star(), "Total Stars", total_stars, 0, content_width);
    let commits_text_node = create_text_node(
        &icon_commits(),
        "Total Commits",
        total_commits,
        1,
        content_width,
    );

    let css_styles = format!(
        r#"
.stat-label {{
    font: 400 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Arial, sans-serif;
    fill: {text_color};
}}
.stat-value {{
    font: 600 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Arial, sans-serif;
    fill: {title_color};
}}
@supports(-moz-appearance: auto) {{
    /* Selector detects Firefox */
    .stat-label, .stat-value {{ font-size: 12px; }}
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
        r#"<svg xmlns='http://www.w3.org/2000/svg'>
    <style>
        {css_styles}
    </style>
    {stars_text_node}
    {commits_text_node}
</svg>"#
    );

    Part {
        height,
        content: svg,
    }
}

fn create_text_node(icon: &str, label: &str, value: u32, index: u64, content_width: f64) -> String {
    // Vertical center of the row in the coordinate space where rows start at y=0.
    let y_center = ROW_HEIGHT / 2.0 + index as f64 * (ROW_HEIGHT + ROW_GAP);
    // Icon is 16 px tall; translate its group so the icon is centered in the row.
    let icon_y = y_center - 8.0;
    let stagger_delay = (index + 3) * 150;
    let formatted_value = format_number(value);
    // Text y=8 within the translated group lands at absolute y=icon_y+8=y_center.
    format!(
        r#"<g class='stagger' style='animation-delay: {stagger_delay}ms' transform='translate(0, {icon_y})'>
    <svg class='icon' viewBox='0 0 16 16' version='1.1' width='16' height='16'>
        {icon}
    </svg>
    <text class='stat-label' x='25' y='8' dominant-baseline='middle'>{label}</text>
    <text class='stat-value' x='{content_width}' y='8' dominant-baseline='middle' text-anchor='end'>{formatted_value}</text>
</g>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_number_below_thousand() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn format_number_thousands() {
        assert_eq!(format_number(1000), "1.0k");
        assert_eq!(format_number(1500), "1.5k");
        assert_eq!(format_number(10_000), "10.0k");
    }

    #[test]
    fn format_number_millions() {
        assert_eq!(format_number(1_000_000), "1.0M");
        assert_eq!(format_number(2_500_000), "2.5M");
    }

    #[test]
    fn render_stats_produces_svg() {
        let theme = crate::themes::dark();
        let part = render_stats(&theme, 42, 1234, 250.0);
        assert!(part.content.contains("<svg"));
        assert!(part.content.contains("42"));
        assert!(part.content.contains("1.2k"));
        assert!(part.height > 0.0);
    }

    #[test]
    fn render_stats_right_aligns_values() {
        let theme = crate::themes::dark();
        let part = render_stats(&theme, 10, 20, 250.0);
        assert!(part.content.contains("text-anchor='end'"));
        assert!(part.content.contains("x='250'"));
    }

    #[test]
    fn render_stats_label_value_classes() {
        let theme = crate::themes::dark();
        let part = render_stats(&theme, 1, 2, 250.0);
        assert!(part.content.contains("stat-label"));
        assert!(part.content.contains("stat-value"));
    }
}
