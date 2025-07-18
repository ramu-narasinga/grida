use crate::node::schema::*;
use math2::transform::AffineTransform;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct IOCanvasFile {
    pub version: String,
    pub document: IODocument,
}

#[derive(Debug, Deserialize)]
pub struct IODocument {
    pub bitmaps: HashMap<String, serde_json::Value>,
    pub properties: HashMap<String, serde_json::Value>,
    pub nodes: HashMap<String, IONode>,
    pub scenes: HashMap<String, IOScene>,
    pub entry_scene_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IOScene {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub children: Vec<String>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<RGBA>,
    pub guides: Option<Vec<serde_json::Value>>,
    pub constraints: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum IONode {
    #[serde(rename = "container")]
    Container(IOContainerNode),
    #[serde(rename = "text")]
    Text(IOTextNode),
    #[serde(rename = "vector")]
    Vector(IOVectorNode),
    #[serde(rename = "path")]
    Path(IOPathNode),
    #[serde(rename = "ellipse")]
    Ellipse(IOEllipseNode),
    #[serde(rename = "rectangle")]
    Rectangle(IORectangleNode),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct IOContainerNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub width: serde_json::Value,
    pub height: serde_json::Value,
    pub children: Vec<String>,
    pub expanded: Option<bool>,
    pub fill: Option<Fill>,
    pub border: Option<Border>,
    pub style: Option<HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "cornerRadius",
        deserialize_with = "deserialize_corner_radius",
        default = "default_corner_radius"
    )]
    pub corner_radius: Option<RectangularCornerRadius>,
    pub padding: Option<serde_json::Value>,
    pub layout: Option<String>,
    pub direction: Option<String>,
    #[serde(rename = "mainAxisAlignment")]
    pub main_axis_alignment: Option<String>,
    #[serde(rename = "crossAxisAlignment")]
    pub cross_axis_alignment: Option<String>,
    #[serde(rename = "mainAxisGap")]
    pub main_axis_gap: Option<f32>,
    #[serde(rename = "crossAxisGap")]
    pub cross_axis_gap: Option<f32>,
}

fn deserialize_corner_radius<'de, D>(
    deserializer: D,
) -> Result<Option<RectangularCornerRadius>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(v) => match v {
            serde_json::Value::Number(n) => {
                let radius = n.as_f64().unwrap_or(0.0) as f32;
                Ok(Some(RectangularCornerRadius::all(radius)))
            }
            serde_json::Value::Array(arr) => {
                if arr.len() == 4 {
                    let values: Vec<f32> = arr
                        .into_iter()
                        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                        .collect();
                    Ok(Some(RectangularCornerRadius {
                        tl: values[0],
                        tr: values[1],
                        bl: values[2],
                        br: values[3],
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct IOTextNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub width: serde_json::Value,
    pub height: serde_json::Value,
    pub fill: Option<Fill>,
    pub style: Option<HashMap<String, serde_json::Value>>,
    pub text: String,
    #[serde(rename = "textAlign", default = "default_text_align")]
    pub text_align: TextAlign,
    #[serde(rename = "textAlignVertical", default = "default_text_align_vertical")]
    pub text_align_vertical: TextAlignVertical,
    #[serde(rename = "textDecoration", default = "default_text_decoration")]
    pub text_decoration: TextDecoration,
    #[serde(rename = "lineHeight")]
    pub line_height: Option<f32>,
    #[serde(rename = "letterSpacing")]
    pub letter_spacing: Option<f32>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<f32>,
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(rename = "fontWeight", default = "default_font_weight")]
    pub font_weight: FontWeight,
}

#[derive(Debug, Deserialize)]
pub struct IOVectorNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Fill>,
    pub paths: Option<Vec<IOPath>>,
}

#[derive(Debug, Deserialize)]
pub struct IOVectorNetworkVertex {
    pub p: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct IOVectorNetworkSegment {
    pub a: usize,
    pub b: usize,
    pub ta: [f32; 2],
    pub tb: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct IOVectorNetwork {
    #[serde(default)]
    pub vertices: Vec<IOVectorNetworkVertex>,
    #[serde(default)]
    pub segments: Vec<IOVectorNetworkSegment>,
}

#[derive(Debug, Deserialize)]
pub struct IOPathNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    #[serde(rename = "vectorNetwork")]
    pub vector_network: Option<IOVectorNetwork>,
    pub fill: Option<Fill>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct IOEllipseNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Fill>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f32>,
    #[serde(rename = "strokeCap")]
    pub stroke_cap: Option<String>,
    pub effects: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct IORectangleNode {
    pub id: String,
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_rotation")]
    pub rotation: f32,
    #[serde(rename = "zIndex", default = "default_z_index")]
    pub z_index: i32,
    pub position: Option<String>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Fill>,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f32>,
    #[serde(rename = "strokeCap")]
    pub stroke_cap: Option<String>,
    pub effects: Option<Vec<serde_json::Value>>,
    #[serde(
        rename = "cornerRadius",
        deserialize_with = "deserialize_corner_radius",
        default = "default_corner_radius"
    )]
    pub corner_radius: Option<RectangularCornerRadius>,
}

#[derive(Debug, Deserialize)]
pub struct IOGradientStop {
    pub offset: f32,
    pub color: RGBA,
}

impl From<IOGradientStop> for GradientStop {
    fn from(stop: IOGradientStop) -> Self {
        GradientStop {
            offset: stop.offset,
            color: stop.color.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Fill {
    #[serde(rename = "solid")]
    Solid { color: Option<RGBA> },
    #[serde(rename = "linear_gradient")]
    LinearGradient {
        id: Option<String>,
        transform: Option<[[f32; 3]; 2]>,
        stops: Vec<IOGradientStop>,
    },
    #[serde(rename = "radial_gradient")]
    RadialGradient {
        id: Option<String>,
        transform: Option<[[f32; 3]; 2]>,
        stops: Vec<IOGradientStop>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Border {
    #[serde(rename = "borderWidth")]
    pub border_width: Option<f32>,
    #[serde(rename = "borderColor")]
    pub border_color: Option<RGBA>,
    #[serde(rename = "borderStyle")]
    pub border_style: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IOPath {
    pub d: String,
    #[serde(rename = "fillRule")]
    pub fill_rule: String,
    pub fill: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

// Default value functions
fn default_active() -> bool {
    true
}
fn default_locked() -> bool {
    false
}
fn default_opacity() -> f32 {
    1.0
}
fn default_rotation() -> f32 {
    0.0
}
fn default_z_index() -> i32 {
    0
}
fn default_text_align() -> TextAlign {
    TextAlign::Left
}
fn default_text_align_vertical() -> TextAlignVertical {
    TextAlignVertical::Top
}
fn default_text_decoration() -> TextDecoration {
    TextDecoration::None
}
fn default_font_weight() -> FontWeight {
    FontWeight::new(400)
}

fn default_corner_radius() -> Option<RectangularCornerRadius> {
    None
}

pub fn parse(file: &str) -> Result<IOCanvasFile, serde_json::Error> {
    serde_json::from_str(file)
}

impl From<RGBA> for Color {
    fn from(color: RGBA) -> Self {
        Color(color.r, color.g, color.b, (color.a * 255.0) as u8)
    }
}

impl From<Option<Fill>> for Paint {
    fn from(fill: Option<Fill>) -> Self {
        match fill {
            Some(Fill::Solid { color }) => Paint::Solid(SolidPaint {
                color: color.map_or(Color(0, 0, 0, 0), |c| c.into()),
                opacity: 1.0,
            }),
            Some(Fill::LinearGradient {
                transform, stops, ..
            }) => {
                let stops = stops.into_iter().map(|s| s.into()).collect();
                Paint::LinearGradient(LinearGradientPaint {
                    transform: transform
                        .map(|m| AffineTransform { matrix: m })
                        .unwrap_or_else(AffineTransform::identity),
                    stops,
                    opacity: 1.0,
                })
            }
            Some(Fill::RadialGradient {
                transform, stops, ..
            }) => {
                let stops = stops.into_iter().map(|s| s.into()).collect();
                Paint::RadialGradient(RadialGradientPaint {
                    transform: transform
                        .map(|m| AffineTransform { matrix: m })
                        .unwrap_or_else(AffineTransform::identity),
                    stops,
                    opacity: 1.0,
                })
            }
            None => Paint::Solid(SolidPaint {
                color: Color(0, 0, 0, 0),
                opacity: 1.0,
            }),
        }
    }
}

impl From<IOContainerNode> for ContainerNode {
    fn from(node: IOContainerNode) -> Self {
        let width = match node.width {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            _ => 0.0,
        };
        let height = match node.height {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            _ => 0.0,
        };
        ContainerNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform: AffineTransform::new(node.left, node.top, node.rotation),
            size: Size { width, height },
            corner_radius: node
                .corner_radius
                .unwrap_or(RectangularCornerRadius::zero()),
            fill: node.fill.into(),
            stroke: None,
            stroke_width: 0.0,
            stroke_align: StrokeAlign::Inside,
            stroke_dash_array: None,
            effect: None,
            children: node.children,
            opacity: node.opacity,
            clip: true,
        }
    }
}

impl From<IOTextNode> for TextSpanNode {
    fn from(node: IOTextNode) -> Self {
        let width = match node.width {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            _ => 0.0,
        };
        let height = match node.height {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
            _ => 0.0,
        };
        TextSpanNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform: AffineTransform::new(node.left, node.top, node.rotation),
            size: Size { width, height },
            text: node.text,
            text_style: TextStyle {
                text_decoration: node.text_decoration,
                font_family: node.font_family.unwrap_or_else(|| "Inter".to_string()),
                font_size: node.font_size.unwrap_or(14.0),
                font_weight: node.font_weight,
                italic: false,
                letter_spacing: node.letter_spacing,
                line_height: node.line_height,
                text_transform: TextTransform::None,
            },
            text_align: node.text_align,
            text_align_vertical: node.text_align_vertical,
            fill: node.fill.into(),
            stroke: None,
            stroke_width: None,
            stroke_align: StrokeAlign::Inside,
            opacity: node.opacity,
        }
    }
}

impl From<IOEllipseNode> for Node {
    fn from(node: IOEllipseNode) -> Self {
        let transform = AffineTransform::new(node.left, node.top, node.rotation);

        Node::Ellipse(EllipseNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform,
            size: Size {
                width: node.width,
                height: node.height,
            },
            fill: node.fill.into(),
            stroke: Paint::Solid(SolidPaint {
                color: Color(0, 0, 0, 255),
                opacity: 1.0,
            }),
            stroke_width: node.stroke_width.unwrap_or(0.0),
            stroke_align: StrokeAlign::Inside,
            stroke_dash_array: None,
            effect: None,
            opacity: node.opacity,
        })
    }
}

impl From<IORectangleNode> for Node {
    fn from(node: IORectangleNode) -> Self {
        let transform = AffineTransform::new(node.left, node.top, node.rotation);

        Node::Rectangle(RectangleNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform,
            size: Size {
                width: node.width,
                height: node.height,
            },
            corner_radius: node
                .corner_radius
                .unwrap_or(RectangularCornerRadius::zero()),
            fill: node.fill.into(),
            stroke: Paint::Solid(SolidPaint {
                color: Color(0, 0, 0, 255),
                opacity: 1.0,
            }),
            stroke_width: node.stroke_width.unwrap_or(0.0),
            stroke_align: StrokeAlign::Inside,
            stroke_dash_array: None,
            effect: None,
            opacity: node.opacity,
        })
    }
}

impl From<IOVectorNode> for Node {
    fn from(node: IOVectorNode) -> Self {
        let transform = AffineTransform::new(node.left, node.top, node.rotation);

        // For vector nodes, we'll create a path node with the path data
        Node::Path(PathNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform,
            fill: node.fill.into(),
            data: node.paths.map_or("".to_string(), |paths| {
                paths
                    .iter()
                    .map(|path| path.d.clone())
                    .collect::<Vec<String>>()
                    .join(" ")
            }),
            stroke: Paint::Solid(SolidPaint {
                color: Color(0, 0, 0, 255),
                opacity: 1.0,
            }),
            stroke_width: 0.0,
            stroke_align: StrokeAlign::Inside,
            stroke_dash_array: None,
            opacity: node.opacity,
            effect: None,
        })
    }
}

fn vector_network_to_path(vn: &IOVectorNetwork) -> String {
    if vn.vertices.is_empty() {
        return String::new();
    }

    let mut d = String::new();
    let first = vn
        .segments
        .get(0)
        .map(|s| vn.vertices[s.a].p)
        .unwrap_or([0.0, 0.0]);
    d.push_str(&format!("M{} {}", first[0], first[1]));

    for seg in &vn.segments {
        let a = vn.vertices[seg.a].p;
        let b = vn.vertices[seg.b].p;
        let c1 = [a[0] + seg.ta[0], a[1] + seg.ta[1]];
        let c2 = [b[0] + seg.tb[0], b[1] + seg.tb[1]];
        d.push_str(&format!(
            " C{} {},{} {},{} {}",
            c1[0], c1[1], c2[0], c2[1], b[0], b[1]
        ));
    }

    d
}

impl From<IOPathNode> for Node {
    fn from(node: IOPathNode) -> Self {
        let transform = AffineTransform::new(node.left, node.top, node.rotation);

        let data = node
            .vector_network
            .as_ref()
            .map(|vn| vector_network_to_path(vn))
            .unwrap_or_else(String::new);

        Node::Path(PathNode {
            base: BaseNode {
                id: node.id,
                name: node.name,
                active: node.active,
            },
            blend_mode: BlendMode::Normal,
            transform,
            fill: node.fill.into(),
            data,
            stroke: Paint::Solid(SolidPaint {
                color: Color(0, 0, 0, 255),
                opacity: 1.0,
            }),
            stroke_width: node.stroke_width.unwrap_or(0.0),
            stroke_align: StrokeAlign::Inside,
            stroke_dash_array: None,
            opacity: node.opacity,
            effect: None,
        })
    }
}

impl From<IONode> for Node {
    fn from(node: IONode) -> Self {
        match node {
            IONode::Container(container) => Node::Container(container.into()),
            IONode::Text(text) => Node::TextSpan(text.into()),
            IONode::Vector(vector) => vector.into(),
            IONode::Path(path) => path.into(),
            IONode::Ellipse(ellipse) => ellipse.into(),
            IONode::Rectangle(rectangle) => rectangle.into(),
            IONode::Unknown => Node::Error(ErrorNode {
                base: BaseNode {
                    id: "unknown".to_string(),
                    name: "Unknown Node".to_string(),
                    active: false,
                },
                transform: AffineTransform::identity(),
                size: Size {
                    width: 100.0,
                    height: 100.0,
                },
                opacity: 1.0,
                error: "Unknown node".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_canvas_json() {
        let path = "../fixtures/local/document.json";
        let Ok(data) = fs::read_to_string(path) else {
            eprintln!("test resource not found: {}", path);
            return;
        };
        let parsed: IOCanvasFile = serde_json::from_str(&data).expect("failed to parse JSON");

        assert_eq!(parsed.version, "0.0.1-beta.1+20250303");
        assert!(
            !parsed.document.nodes.is_empty(),
            "nodes should not be empty"
        );
    }

    #[test]
    fn corner_radius_optional_and_falls_back_to_zero() {
        // Test JSON without cornerRadius field
        let json_without_corner_radius = r#"{
            "version": "0.0.1-beta.1+20250303",
            "document": {
                "bitmaps": {},
                "properties": {},
                "nodes": {
                    "test-rect": {
                        "type": "rectangle",
                        "id": "test-rect",
                        "name": "Test Rectangle",
                        "left": 0.0,
                        "top": 0.0,
                        "width": 100.0,
                        "height": 100.0,
                        "fill": {
                            "type": "solid",
                            "color": {
                                "r": 255,
                                "g": 0,
                                "b": 0,
                                "a": 1.0
                            }
                        }
                    }
                },
                "scenes": {}
            }
        }"#;

        let parsed: IOCanvasFile = serde_json::from_str(json_without_corner_radius)
            .expect("failed to parse JSON without cornerRadius");

        // Verify that the rectangle node was parsed successfully
        if let Some(IONode::Rectangle(rect_node)) = parsed.document.nodes.get("test-rect") {
            // corner_radius should be None when not present in JSON
            assert!(rect_node.corner_radius.is_none());
        } else {
            panic!("Expected rectangle node not found");
        }

        // Test JSON with cornerRadius field
        let json_with_corner_radius = r#"{
            "version": "0.0.1-beta.1+20250303",
            "document": {
                "bitmaps": {},
                "properties": {},
                "nodes": {
                    "test-rect": {
                        "type": "rectangle",
                        "id": "test-rect",
                        "name": "Test Rectangle",
                        "left": 0.0,
                        "top": 0.0,
                        "width": 100.0,
                        "height": 100.0,
                        "cornerRadius": 10.0,
                        "fill": {
                            "type": "solid",
                            "color": {
                                "r": 255,
                                "g": 0,
                                "b": 0,
                                "a": 1.0
                            }
                        }
                    }
                },
                "scenes": {}
            }
        }"#;

        let parsed: IOCanvasFile = serde_json::from_str(json_with_corner_radius)
            .expect("failed to parse JSON with cornerRadius");

        // Verify that the rectangle node was parsed successfully with cornerRadius
        if let Some(IONode::Rectangle(rect_node)) = parsed.document.nodes.get("test-rect") {
            // corner_radius should be Some when present in JSON
            assert!(rect_node.corner_radius.is_some());
            if let Some(corner_radius) = &rect_node.corner_radius {
                assert_eq!(corner_radius.tl, 10.0);
                assert_eq!(corner_radius.tr, 10.0);
                assert_eq!(corner_radius.bl, 10.0);
                assert_eq!(corner_radius.br, 10.0);
            }
        } else {
            panic!("Expected rectangle node not found");
        }
    }
}
