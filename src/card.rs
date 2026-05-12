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
    theme: &Theme,
) -> String {
    let title_height: f64 = 18.0;
    let height: f64 = parts.len() as f64 * gap
        + parts.iter().map(|p| p.height).sum::<f64>()
        + y_offset
        + title_height
        + y_offset / 2.0;
    let title_color = &theme.title_color;
    let background_color = &theme.background_color;
    let border_color = &theme.border_color;

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
        part_offset += gap + part.height;
    }

    format!(
        r#"
<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg'>
    <rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{background_color}' stroke='{border_color}'/>
    <g transform='translate({x_offset}, {y_offset})'>
        <text style="font: 600 {title_height}px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {title_color};">{title}</text>
        {translated_parts}
    </g>
</svg>
    "#
    )
}

/// Returns a minimal SVG card displaying an error message.
/// Used as a fallback when GitHub API requests fail so that README badge
/// consumers see a meaningful image rather than a broken link.
pub fn render_error_card(message: &str, width: f64, theme: &Theme) -> String {
    let background_color = &theme.background_color;
    let border_color = &theme.border_color;
    let text_color = &theme.text_color;
    let height: f64 = 80.0;
    format!(
        r#"
<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg'>
    <rect x='0' y='0' width='{width}' height='{height}' rx='4.5' fill='{background_color}' stroke='{border_color}'/>
    <text x='50%' y='50%' dominant-baseline='middle' text-anchor='middle'
          style="font: 600 13px 'Segoe UI', Ubuntu, 'Helvetica Neue', Sans-Serif; fill: {text_color};">
        {message}
    </text>
</svg>
    "#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes;

    #[test]
    fn render_card_produces_valid_svg() {
        let theme = themes::dark();
        let part = Part {
            height: 50.0,
            content: "<text>hello</text>".to_string(),
        };
        let svg = render_card(vec![part], 25.0, 35.0, 20.0, 300.0, "Test Title", &theme);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Test Title"));
        assert!(svg.contains("hello"));
    }

    #[test]
    fn render_card_uses_theme_colors() {
        let theme = themes::dark();
        let svg = render_card(vec![], 25.0, 35.0, 20.0, 300.0, "T", &theme);
        assert!(svg.contains(&theme.background_color));
        assert!(svg.contains(&theme.border_color));
    }

    #[test]
    fn render_card_light_theme() {
        let theme = themes::light();
        let svg = render_card(vec![], 25.0, 35.0, 20.0, 300.0, "T", &theme);
        assert!(svg.contains(&theme.background_color));
    }

    #[test]
    fn render_error_card_produces_valid_svg() {
        let theme = themes::dark();
        let svg = render_error_card("Error occurred", 300.0, &theme);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Error occurred"));
    }

    #[test]
    fn render_error_card_uses_theme_colors() {
        let theme = themes::dark();
        let svg = render_error_card("Err", 300.0, &theme);
        assert!(svg.contains(&theme.background_color));
        assert!(svg.contains(&theme.border_color));
    }
}
