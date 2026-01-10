#[derive(Clone, Debug)]
pub struct Point(pub f64, pub f64);

impl<X: Into<f64>, Y: Into<f64>> Into<Point> for (X, Y) {
    fn into(self) -> Point {
        Point(self.0.into(), self.1.into())
    }
}

impl Default for Point {
    fn default() -> Self {
        Self(0., 0.)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color(red, green, blue, 255)
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color(red, green, blue, alpha)
    }

    pub fn hex(rgb: u32) -> Self {
        let red = rgb & 0x110000 >> 8;
        let green = rgb & 0x001100 >> 4;
        let blue = rgb & 0x000011 >> 0;
        Color(red as u8, green as u8, blue as u8, 255)
    }

    pub fn hexa(rgba: u64) -> Self {
        let red = rgba & 0x11000000 >> 12;
        let green = rgba & 0x00110000 >> 8;
        let blue = rgba & 0x00001100 >> 4;
        let alpha = rgba & 0x00000011 >> 0;
        Color(red as u8, green as u8, blue as u8, alpha as u8)
    }

    pub fn white() -> Self {
        Self::hex(0x00_00_00)
    }

    pub fn black() -> Self {
        Self::hex(0xFF_FF_FF)
    }

    pub fn none() -> Self {
        Self(0, 0, 0, 0)
    }

    pub fn is_none(&self) -> bool {
        let Color(r, g, b, a) = self;
        *r == 0 && *g == 0 && *b == 0 && *a == 0
    }
}

impl Default for Color {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}

pub type ColorMap = std::collections::HashMap<Color, String>;

#[derive(Debug)]
pub struct Canvas {
    drawings: Vec<Drawable>,
}

impl<'a> Canvas {
    pub fn new() -> Self {
        Self {
            drawings: Vec::new(),
        }
    }

    pub fn draw<D: Into<Drawable>>(&mut self, d: D) {
        self.drawings.push(d.into());
    }

    pub fn get_colors(&'a self) -> Vec<&'a Color> {
        let mut colors = Vec::new();
        for drawing in &self.drawings {
            match drawing {
                Drawable::Line(line) => colors.push(&line.stroke.color),
                Drawable::Circle(circle) => {
                    colors.push(&circle.stroke.color);
                    colors.push(&circle.fill);
                }
                Drawable::Rect(rect) => {
                    colors.push(&rect.stroke.color);
                    colors.push(&rect.fill);
                }
                Drawable::Text(text) => {
                    colors.push(&text.stroke.color);
                    colors.push(&text.fill);
                    colors.push(&text.font);
                }
            }
        }
        colors
    }
}

pub trait Artist {
    type Output;
    type Err;
    type PartialOutput;

    fn render(c: Canvas) -> Result<Self::Output, Self::Err>;
    fn render_line(line: Line, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err>;
    fn render_circle(circle: Circle, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err>;
    fn render_rect(rect: Rect, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err>;
    fn render_text(text: Text, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err>;

    fn render_any(d: Drawable, cm: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        match d {
            Drawable::Line(l) => Self::render_line(l, &cm),
            Drawable::Rect(r) => Self::render_rect(r, &cm),
            Drawable::Circle(c) => Self::render_circle(c, &cm),
            Drawable::Text(t) => Self::render_text(t, &cm),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stroke {
    pub color: Color,
    pub thickness: f64,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::black(),
            thickness: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub from: Point,
    pub to: Point,
    pub stroke: Stroke,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            from: (0, 0).into(),
            to: (0, 0).into(),
            stroke: Stroke::default(),
        }
    }
}

impl Line {
    pub fn start<P: Into<Point>>(mut self, point: P) -> Self {
        self.from = point.into();
        self
    }

    pub fn end<P: Into<Point>>(mut self, point: P) -> Self {
        self.to = point.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
    pub stroke: Stroke,
    pub fill: Color,
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            center: (0, 0).into(),
            radius: 1.,
            stroke: Stroke::default(),
            fill: Color::none(),
        }
    }
}

impl Circle {
    pub fn at<P: Into<Point>>(mut self, point: P) -> Self {
        self.center = point.into();
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
}

#[derive(Clone, Debug)]
pub struct Rect {
    from: Point,
    to: Point,
    stroke: Stroke,
    fill: Color,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            from: (0, 0).into(),
            to: (0, 0).into(),
            stroke: Stroke::default(),
            fill: Color::none(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Anchor {
    Center,
    North,
    South,
    NorthEast,
    SouthEast,
    East,
    NorthWest,
    SouthWest,
    West,
}

impl Default for Anchor {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub location: Point,
    pub anchor: Anchor,
    pub stroke: Stroke,
    pub fill: Color,
    pub font: Color,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: String::new(),
            location: Point::default(),
            anchor: Anchor::Center,
            stroke: Stroke::default(),
            fill: Color::none(),
            font: Color::black(),
        }
    }
}

impl Text {
    pub fn content(mut self, s: String) -> Self {
        self.content = s;
        self
    }

    pub fn anchor(mut self, a: Anchor) -> Self {
        self.anchor = a;
        self
    }

    pub fn at<P: Into<Point>>(mut self, point: P) -> Self {
        self.location = point.into();
        self
    }
}

#[derive(Clone, Debug)]
pub enum Drawable {
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Text(Text),
}

impl Into<Drawable> for Line {
    fn into(self) -> Drawable {
        Drawable::Line(self)
    }
}

impl Into<Drawable> for Circle {
    fn into(self) -> Drawable {
        Drawable::Circle(self)
    }
}

impl Into<Drawable> for Rect {
    fn into(self) -> Drawable {
        Drawable::Rect(self)
    }
}

impl Into<Drawable> for Text {
    fn into(self) -> Drawable {
        Drawable::Text(self)
    }
}

pub struct TikZ;

impl Artist for TikZ {
    type Err = ();
    type Output = String;
    type PartialOutput = String;

    fn render(c: Canvas) -> Result<Self::Output, Self::Err> {
        let mut buf = String::new();
        let mut colors: ColorMap = std::collections::HashMap::new();
        // colors
        for color in c.get_colors() {
            let ident = match colors.get(color) {
                Some(_) => continue,
                None => {
                    let s = if color.is_none() {
                        "none".to_string()
                    } else {
                        format!("Color{}", colors.len())
                    };
                    colors.insert(color.clone(), s.clone());
                    s
                }
            };
            let s = format!(
                "\\definecolor{{{}}}{{RGB}}{{{},{},{}}}",
                ident, color.0, color.1, color.2,
            );
            buf.push_str(&s);
            buf.push('\n');
        }
        // drawings
        for drawing in c.drawings {
            let s = Self::render_any(drawing, &colors).unwrap();
            buf.push_str(&s);
            buf.push('\n');
        }
        Ok(buf)
    }

    fn render_line(line: Line, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let color = c.get(&line.stroke.color).unwrap();
        let s = format!(
            "\\path[line width={}, draw={}] ({}, {}) -- ({}, {});",
            line.stroke.thickness, color, line.from.0, line.from.1, line.to.0, line.to.1,
        );
        Ok(s)
    }

    fn render_circle(circle: Circle, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&circle.stroke.color).unwrap();
        let fill = c.get(&circle.fill).unwrap();
        let s = format!(
            "\\path[draw={}, fill={}, line width={}] ({}, {}) circle ({});",
            outline, fill, circle.stroke.thickness, circle.center.0, circle.center.1, circle.radius,
        );
        Ok(s)
    }

    fn render_rect(rect: Rect, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&rect.stroke.color).unwrap();
        let fill = c.get(&rect.fill).unwrap();
        let s = format!(
            "\\path[draw={}, fill={}, line width={}] ({}, {}) rectangle ({}, {});",
            outline, fill, rect.stroke.thickness, rect.from.0, rect.from.1, rect.to.0, rect.to.1,
        );
        Ok(s)
    }

    fn render_text(text: Text, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&text.stroke.color).unwrap();
        let fill = c.get(&text.fill).unwrap();
        let font = c.get(&text.font).unwrap();
        let anchor = match text.anchor {
            Anchor::Center => "center",
            Anchor::North => "north",
            Anchor::South => "south",
            Anchor::NorthEast => "north east",
            Anchor::SouthEast => "south east",
            Anchor::East => "east",
            Anchor::NorthWest => "north west",
            Anchor::SouthWest => "south west",
            Anchor::West => "west",
        };
        let s = format!(
            "\\node[text={}, draw={}, fill={}, line width={}, anchor={}] at ({}, {}) {{{}}};",
            font, outline, fill, text.stroke.thickness, anchor, text.location.0, text.location.1, text.content,
        );
        Ok(s)
    }
}

const TIKZ_PREAMBLE: &str = "\
\\documentclass[tikz]{standalone}
\\usepackage{tikz}
\\begin{document}
\\begin{tikzpicture}
";

const TIKZ_EPILOG: &str = "\
\\end{tikzpicture}
\\end{document}
";

impl TikZ {
    pub fn render_doc(c: Canvas) -> Result<String, ()> {
        let mut buf = TIKZ_PREAMBLE.to_string();
        let rendered = Self::render(c).unwrap();
        buf.push_str(&rendered);
        buf.push_str(TIKZ_EPILOG);
        Ok(buf)
    }
}

pub struct CeTZ;

impl Artist for CeTZ {
    type Err = ();
    type Output = String;
    type PartialOutput = String;

    fn render(c: Canvas) -> Result<Self::Output, Self::Err> {
        let mut buf = String::new();
        let mut colors: ColorMap = std::collections::HashMap::new();
        // colors
        for color in c.get_colors() {
            let ident = match colors.get(color) {
                Some(_) => continue,
                None => {
                    let s = if color.is_none() {
                        "none".to_string()
                    } else {
                        format!("color-{}", colors.len())
                    };
                    colors.insert(color.clone(), s.clone());
                    s
                }
            };

            if ident == "none" {
                continue;
            }

            let s = format!(
                "let {} = color.rgb({}, {}, {})",
                ident, color.0, color.1, color.2,
            );
            buf.push_str(&s);
            buf.push('\n');
        }
        // drawings
        for drawing in c.drawings {
            let s = Self::render_any(drawing, &colors).unwrap();
            buf.push_str(&s);
            buf.push('\n');
        }
        Ok(buf)
    }

    fn render_line(line: Line, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let color = c.get(&line.stroke.color).unwrap();
        let s = format!(
            "line(({}, {}), ({}, {}), stroke: (paint: {}, thickness: {}))",
            line.from.0, line.from.1, line.to.0, line.to.1, color, line.stroke.thickness,
        );
        Ok(s)
    }

    fn render_circle(circle: Circle, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&circle.stroke.color).unwrap();
        let fill = c.get(&circle.fill).unwrap();
        let s = format!(
            "circle(({}, {}), radius: {}, stroke: (paint: {}, thickness: {}), fill: {})",
            circle.center.0, circle.center.1, circle.radius, outline, circle.stroke.thickness, fill,
        );
        Ok(s)
    }

    fn render_rect(rect: Rect, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&rect.stroke.color).unwrap();
        let fill = c.get(&rect.fill).unwrap();
        let s = format!(
            "rect(({}, {}), ({}, {}), stroke: (paint: {}, thickness: {}), fill: {})",
            rect.from.0, rect.from.1, rect.to.0, rect.to.1, outline, rect.stroke.thickness, fill,
        );
        Ok(s)
    }

    fn render_text(text: Text, c: &ColorMap) -> Result<Self::PartialOutput, Self::Err> {
        let outline = c.get(&text.stroke.color).unwrap();
        let fill = c.get(&text.fill).unwrap();
        let font = c.get(&text.font).unwrap();
        let anchor = match text.anchor {
            Anchor::Center => "center",
            Anchor::North => "north",
            Anchor::South => "south",
            Anchor::NorthEast => "north east",
            Anchor::SouthEast => "south east",
            Anchor::East => "east",
            Anchor::NorthWest => "north west",
            Anchor::SouthWest => "south west",
            Anchor::West => "west",
        };
        let s = format!(
            "content(({}, {}), stroke: (paint: {}, thickness: {}), fill: {}, color: {}, anchor: \"{}\")[{}]",
            text.location.0, text.location.1, outline, text.stroke.thickness, fill, font, anchor, text.content,
        );
        Ok(s)
    }
}

const CETZ_PREAMBLE: &str = "\
#import \"@preview/cetz:0.4.2\"
#set page(width: auto, height: auto)
#cetz.canvas({
import cetz.draw: *
";

const CETZ_EPILOG: &str = "})\n";

impl CeTZ {
    pub fn render_doc(c: Canvas) -> Result<String, ()> {
        let mut buf = CETZ_PREAMBLE.to_string();
        let rendered = Self::render(c).unwrap();
        buf.push_str(&rendered);
        buf.push_str(CETZ_EPILOG);
        Ok(buf)
    }
}

pub struct PlotOptions {
    pub second_width: f64,
    pub y: f64,
}

impl Default for PlotOptions {
    fn default() -> Self {
        Self {
            second_width: 1. / 60.,
            y: 0.0,
        }
    }
}
