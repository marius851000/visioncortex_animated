use visioncortex::Color;

pub struct SvgFill {
    path_string: String,
    color: Color,
}

pub struct SvgBuilder {
    fill: Vec<SvgFill>,
    width: usize,
    height: usize,
}

impl SvgBuilder {
    pub fn new(width: usize, height: usize) -> SvgBuilder {
        SvgBuilder {
            fill: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_fill(&mut self, path: String, color: Color) {
        self.fill.push(SvgFill {
            path_string: path,
            color,
        })
    }

    pub fn to_vector_file(&self) -> String {
        let mut result = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <svg width="{}" height="{}">
        "#,
            self.width, self.height
        );

        for fill in &self.fill {
            let color = fill.color;
            result.push_str(&format!(
                "<path d=\"{}\" fill=\"#{:02x}{:02x}{:02x}\"/>\n",
                fill.path_string, color.r, color.g, color.b
            ));
        }

        result.push_str("</svg>");
        result
    }
}
