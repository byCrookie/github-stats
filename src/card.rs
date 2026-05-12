use crate::themes::Theme;

/// Escapes characters that are special in XML/SVG text content and attribute values.
pub(crate) fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub struct Part {
    pub height: f64,
    pub content: String,
}

/// Vertical space reserved for the title: 14 px font (baseline at y=16) + 14 px breathing room
/// before content = 30 px total.
const TITLE_SECTION_HEIGHT: f64 = 30.0;

pub fn render_card(
    parts: Vec<Part>,
    x_offset: f64,
    y_offset: f64,
    gap: f64,
    width: f64,
    title: &str,
    theme: &Theme,
) -> String {
    // Gaps only between parts (not before the first or after the last).
    let height: f64 = parts.len().saturating_sub(1) as f64 * gap
        + parts.iter().map(|p| p.height).sum::<f64>()
        + y_offset              // top padding (card edge → title cap-top)
        + TITLE_SECTION_HEIGHT
        + y_offset; // bottom padding (symmetric with top)
    let title_color = &theme.title_color;
    let background_color = &theme.background_color;
    let border_color = &theme.border_color;
    let escaped_title = xml_escape(title);
    let inset_w = width - 1.0;
    let inset_h = height - 1.0;

    let mut part_offset: f64 = TITLE_SECTION_HEIGHT;
    let mut translated_parts = String::new();
    for part in parts.iter() {
        let part_content = &part.content;
        translated_parts.push_str(&format!(
            r#"<g transform='translate(0, {part_offset})'>
    {part_content}
</g>"#
        ));
        part_offset += gap + part.height;
    }

    format!(
        r#"<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg' role='img' aria-labelledby='card-title'>
    <title id='card-title'>{escaped_title}</title>
    <rect x='0.5' y='0.5' width='{inset_w}' height='{inset_h}' rx='8' fill='{background_color}' stroke='{border_color}' stroke-width='1'/>
    <g transform='translate({x_offset}, {y_offset})'>
        <text y='16' style="font: 600 14px 'Segoe UI', Ubuntu, 'Helvetica Neue', Arial, sans-serif; fill: {title_color};">{escaped_title}</text>
        {translated_parts}
    </g>
</svg>"#
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
    let message = xml_escape(message);
    let inset_w = width - 1.0;
    let inset_h = height - 1.0;
    format!(
        r#"<svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' xmlns='http://www.w3.org/2000/svg' role='img' aria-label='Error'>
    <rect x='0.5' y='0.5' width='{inset_w}' height='{inset_h}' rx='8' fill='{background_color}' stroke='{border_color}' stroke-width='1'/>
    <text x='50%' y='50%' dominant-baseline='middle' text-anchor='middle'
          style="font: 600 13px 'Segoe UI', Ubuntu, 'Helvetica Neue', Arial, sans-serif; fill: {text_color};">
        {message}
    </text>
</svg>"#
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
    fn render_card_has_accessibility_attrs() {
        let theme = themes::dark();
        let svg = render_card(vec![], 25.0, 35.0, 20.0, 300.0, "My Card", &theme);
        assert!(svg.contains("role='img'"));
        assert!(svg.contains("aria-labelledby='card-title'"));
        assert!(svg.contains("<title id='card-title'>My Card</title>"));
    }

    #[test]
    fn render_card_title_is_escaped() {
        let theme = themes::dark();
        let svg = render_card(vec![], 25.0, 35.0, 20.0, 300.0, "A & B", &theme);
        assert!(!svg.contains("A & B"));
        assert!(svg.contains("A &amp; B"));
    }

    #[test]
    fn render_card_no_separator_line() {
        let theme = themes::dark();
        let svg = render_card(vec![], 25.0, 35.0, 20.0, 300.0, "T", &theme);
        // Separator line was removed; title separation is achieved through spacing alone.
        assert!(!svg.contains("<line"));
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

    #[test]
    fn render_error_card_escapes_message() {
        let theme = themes::dark();
        let svg = render_error_card("<script>alert(1)</script>", 300.0, &theme);
        assert!(!svg.contains("<script>"));
        assert!(svg.contains("&lt;script&gt;"));
    }

    #[test]
    fn render_error_card_has_accessibility_attrs() {
        let theme = themes::dark();
        let svg = render_error_card("Err", 300.0, &theme);
        assert!(svg.contains("role='img'"));
        assert!(svg.contains("aria-label='Error'"));
    }

    #[test]
    fn xml_escape_special_chars() {
        assert_eq!(xml_escape("Rust"), "Rust");
        assert_eq!(xml_escape("<script>"), "&lt;script&gt;");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
        assert_eq!(xml_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(xml_escape("it's"), "it&#39;s");
    }
}
