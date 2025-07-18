use crate::node::repository::NodeRepository;
use crate::painter::cvt;
use core::str;
use math2::box_fit::BoxFit;
use math2::rect::Rectangle;
use math2::transform::AffineTransform;
use serde::Deserialize;

pub type NodeId = String;

/// A 2D point with x and y coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Subtracts a scaled vector from this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The point to subtract
    /// * `scale` - The scale factor to apply to the other point
    ///
    /// # Returns
    ///
    /// A new point representing the result of the vector operation
    pub fn subtract_scaled(&self, other: Point, scale: f32) -> Point {
        Point {
            x: self.x - other.x * scale,
            y: self.y - other.y * scale,
        }
    }
}

/// Boolean path operation.
#[derive(Debug, Clone, Copy)]
pub enum BooleanPathOperation {
    Union,        // A ∪ B
    Intersection, // A ∩ B
    Difference,   // A - B
    Xor,          // A ⊕ B
}

impl From<BooleanPathOperation> for skia_safe::PathOp {
    fn from(op: BooleanPathOperation) -> Self {
        match op {
            BooleanPathOperation::Union => skia_safe::PathOp::Union,
            BooleanPathOperation::Intersection => skia_safe::PathOp::Intersect,
            BooleanPathOperation::Difference => skia_safe::PathOp::Difference,
            BooleanPathOperation::Xor => skia_safe::PathOp::XOR,
        }
    }
}

/// Stroke alignment.
///
/// - [Flutter](https://api.flutter.dev/flutter/painting/BorderSide/strokeAlign.html)  
/// - [Figma](https://www.figma.com/plugin-docs/api/properties/nodes-strokealign/)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeAlign {
    Inside,
    Center,
    Outside,
}

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

/// Represents filter effects inspired by SVG `<filter>` primitives.
///
/// See also:
/// - https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow
/// - https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur
#[derive(Debug, Clone)]
pub enum FilterEffect {
    /// Drop shadow filter: offset + blur + color
    DropShadow(FeDropShadow),

    /// Gaussian blur filter: blur only
    GaussianBlur(FeGaussianBlur),

    /// Background blur filter: blur only
    BackdropBlur(FeBackdropBlur),
}

/// A background blur effect, similar to CSS `backdrop-filter: blur(...)`
#[derive(Debug, Clone, Copy)]
pub struct FeBackdropBlur {
    /// Blur radius in logical pixels.
    pub radius: f32,
}

/// A drop shadow filter effect (`<feDropShadow>`)
#[derive(Debug, Clone, Copy)]
pub struct FeDropShadow {
    /// Horizontal shadow offset in px
    pub dx: f32,

    /// Vertical shadow offset in px
    pub dy: f32,

    /// Blur radius (`stdDeviation` in SVG)
    pub blur: f32,

    /// Shadow color (includes alpha)
    pub color: Color,
}

/// A standalone blur filter effect (`<feGaussianBlur>`)
#[derive(Debug, Clone, Copy)]
pub struct FeGaussianBlur {
    /// Blur radius (`stdDeviation` in SVG)
    pub radius: f32,
}

/// Blend modes for compositing layers, compatible with Skia and SVG/CSS.
///
/// - SVG: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mix-blend-mode
/// - Skia: https://skia.org/docs/user/api/SkBlendMode_Reference/
/// - Figma: https://help.figma.com/hc/en-us/articles/360039956994
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    // Skia: kSrcOver, CSS: normal
    Normal,

    // Skia: kMultiply
    Multiply,
    // Skia: kScreen
    Screen,
    // Skia: kOverlay
    Overlay,
    // Skia: kDarken
    Darken,
    // Skia: kLighten
    Lighten,
    // Skia: kColorDodge
    ColorDodge,
    // Skia: kColorBurn
    ColorBurn,
    // Skia: kHardLight
    HardLight,
    // Skia: kSoftLight
    SoftLight,
    // Skia: kDifference
    Difference,
    // Skia: kExclusion
    Exclusion,
    // Skia: kHue
    Hue,
    // Skia: kSaturation
    Saturation,
    // Skia: kColor
    Color,
    // Skia: kLuminosity
    Luminosity,

    /// Like `Normal`, but means no blending at all (pass-through).
    /// This is Figma-specific, and typically treated the same as `Normal`.
    PassThrough,
}

impl From<BlendMode> for skia_safe::BlendMode {
    fn from(mode: BlendMode) -> Self {
        use skia_safe::BlendMode::*;
        match mode {
            BlendMode::Normal => SrcOver,
            BlendMode::Multiply => Multiply,
            BlendMode::Screen => Screen,
            BlendMode::Overlay => Overlay,
            BlendMode::Darken => Darken,
            BlendMode::Lighten => Lighten,
            BlendMode::ColorDodge => ColorDodge,
            BlendMode::ColorBurn => ColorBurn,
            BlendMode::HardLight => HardLight,
            BlendMode::SoftLight => SoftLight,
            BlendMode::Difference => Difference,
            BlendMode::Exclusion => Exclusion,
            BlendMode::Hue => Hue,
            BlendMode::Saturation => Saturation,
            BlendMode::Color => Color,
            BlendMode::Luminosity => Luminosity,
            BlendMode::PassThrough => SrcOver, // fallback
        }
    }
}

/// Text Transform (Text Case)
/// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform)
#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub enum TextTransform {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "capitalize")]
    Capitalize,
}

/// Supported text decoration modes.
///
/// Only `Underline` and `None` are supported in the current version.
///
/// - [Flutter](https://api.flutter.dev/flutter/dart-ui/TextDecoration-class.html)  
/// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration)
#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub enum TextDecoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "overline")]
    Overline,
    #[serde(rename = "line-through")]
    LineThrough,
}

impl From<TextDecoration> for skia_safe::textlayout::TextDecoration {
    fn from(mode: TextDecoration) -> Self {
        match mode {
            TextDecoration::None => skia_safe::textlayout::TextDecoration::NO_DECORATION,
            TextDecoration::Underline => skia_safe::textlayout::TextDecoration::UNDERLINE,
            TextDecoration::Overline => skia_safe::textlayout::TextDecoration::OVERLINE,
            TextDecoration::LineThrough => skia_safe::textlayout::TextDecoration::LINE_THROUGH,
        }
    }
}

/// Supported horizontal text alignment.
///
/// Does not include `Start` or `End`, as they are not supported currently.
///
/// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/text-align)  
/// - [Flutter](https://api.flutter.dev/flutter/dart-ui/TextAlign.html)
#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub enum TextAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "justify")]
    Justify,
}

impl From<TextAlign> for skia_safe::textlayout::TextAlign {
    fn from(mode: TextAlign) -> Self {
        use skia_safe::textlayout::TextAlign::*;
        match mode {
            TextAlign::Left => Left,
            TextAlign::Right => Right,
            TextAlign::Center => Center,
            TextAlign::Justify => Justify,
        }
    }
}

/// Supported vertical alignment values for text.
///
/// In CSS, this maps to `align-content`.
///
/// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)  
/// - [Konva](https://konvajs.org/api/Konva.Text.html#verticalAlign)
#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub enum TextAlignVertical {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "bottom")]
    Bottom,
}

/// Font weight value (1-1000).
///
/// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight)  
/// - [Flutter](https://api.flutter.dev/flutter/dart-ui/FontWeight-class.html)  
/// - [OpenType spec](https://learn.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass)
#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub struct FontWeight(pub u32);

impl FontWeight {
    /// Creates a new font weight value.
    ///
    /// # Arguments
    ///
    /// * `value` - The font weight value (1-1000)
    ///
    /// # Panics
    ///
    /// Panics if the value is not between 1 and 1000.
    pub fn new(value: u32) -> Self {
        assert!(
            value >= 1 && value <= 1000,
            "Font weight must be between 1 and 1000"
        );
        Self(value)
    }

    /// Returns the font weight value.
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn default() -> Self {
        Self(400)
    }
}

/// A set of style properties that can be applied to a text or text span.
#[derive(Debug, Clone)]
pub struct TextStyle {
    /// Text decoration (e.g. underline or none).
    pub text_decoration: TextDecoration,

    /// Optional font family name (e.g. "Roboto").
    pub font_family: String,

    /// Font size in logical pixels.
    pub font_size: f32,

    /// Font weight (100–900).
    pub font_weight: FontWeight,

    /// Font italic style.
    pub italic: bool,

    /// Additional spacing between characters, in logical pixels.  
    /// Default is `0.0`.
    pub letter_spacing: Option<f32>,

    /// Line height
    pub line_height: Option<f32>,

    /// Text transform (e.g. uppercase, lowercase, capitalize)
    pub text_transform: TextTransform,
}

#[derive(Debug, Clone, Copy)]
pub struct GradientStop {
    /// 0.0 = start, 1.0 = end
    pub offset: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum Paint {
    Solid(SolidPaint),
    LinearGradient(LinearGradientPaint),
    RadialGradient(RadialGradientPaint),
    Image(ImagePaint),
}

#[derive(Debug, Clone)]
pub struct SolidPaint {
    pub color: Color,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct LinearGradientPaint {
    pub transform: AffineTransform,
    pub stops: Vec<GradientStop>,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct RadialGradientPaint {
    pub transform: AffineTransform,
    pub stops: Vec<GradientStop>,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct ImagePaint {
    pub transform: AffineTransform,
    pub _ref: String,
    pub fit: BoxFit,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct RectangularCornerRadius {
    pub tl: f32,
    pub tr: f32,
    pub bl: f32,
    pub br: f32,
}

impl RectangularCornerRadius {
    pub fn zero() -> Self {
        Self::all(0.0)
    }

    pub fn all(value: f32) -> Self {
        Self {
            tl: value,
            tr: value,
            bl: value,
            br: value,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.tl == 0.0 && self.tr == 0.0 && self.bl == 0.0 && self.br == 0.0
    }

    pub fn is_uniform(&self) -> bool {
        self.tl == self.tr && self.tl == self.bl && self.tl == self.br
    }
}

// region: Scene
#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub transform: AffineTransform,
    pub children: Vec<NodeId>,
    pub nodes: NodeRepository,
    pub background_color: Option<Color>,
}

// endregion

// region: Node Definitions

#[derive(Debug, Clone)]
pub enum Node {
    Error(ErrorNode),
    Group(GroupNode),
    Container(ContainerNode),
    Rectangle(RectangleNode),
    Ellipse(EllipseNode),
    Polygon(PolygonNode),
    RegularPolygon(RegularPolygonNode),
    RegularStarPolygon(RegularStarPolygonNode),
    Line(LineNode),
    TextSpan(TextSpanNode),
    Path(PathNode),
    BooleanOperation(BooleanPathOperationNode),
    Image(ImageNode),
}

// node trait
pub trait NodeTrait {
    fn id(&self) -> NodeId;
    fn name(&self) -> String;
}

impl NodeTrait for Node {
    fn id(&self) -> NodeId {
        match self {
            Node::Error(n) => n.base.id.clone(),
            Node::Group(n) => n.base.id.clone(),
            Node::Container(n) => n.base.id.clone(),
            Node::Rectangle(n) => n.base.id.clone(),
            Node::Ellipse(n) => n.base.id.clone(),
            Node::Polygon(n) => n.base.id.clone(),
            Node::RegularPolygon(n) => n.base.id.clone(),
            Node::RegularStarPolygon(n) => n.base.id.clone(),
            Node::Line(n) => n.base.id.clone(),
            Node::TextSpan(n) => n.base.id.clone(),
            Node::Path(n) => n.base.id.clone(),
            Node::BooleanOperation(n) => n.base.id.clone(),
            Node::Image(n) => n.base.id.clone(),
        }
    }

    fn name(&self) -> String {
        match self {
            Node::Error(n) => n.base.name.clone(),
            Node::Group(n) => n.base.name.clone(),
            Node::Container(n) => n.base.name.clone(),
            Node::Rectangle(n) => n.base.name.clone(),
            Node::Ellipse(n) => n.base.name.clone(),
            Node::Polygon(n) => n.base.name.clone(),
            Node::RegularPolygon(n) => n.base.name.clone(),
            Node::RegularStarPolygon(n) => n.base.name.clone(),
            Node::Line(n) => n.base.name.clone(),
            Node::TextSpan(n) => n.base.name.clone(),
            Node::Path(n) => n.base.name.clone(),
            Node::BooleanOperation(n) => n.base.name.clone(),
            Node::Image(n) => n.base.name.clone(),
        }
    }
}

/// Intrinsic size node is a node that has a fixed size, and can be rendered soley on its own.
#[derive(Debug, Clone)]
pub enum IntrinsicSizeNode {
    Error(ErrorNode),
    Container(ContainerNode),
    Rectangle(RectangleNode),
    Ellipse(EllipseNode),
    Polygon(PolygonNode),
    RegularPolygon(RegularPolygonNode),
    RegularStarPolygon(RegularStarPolygonNode),
    Line(LineNode),
    TextSpan(TextSpanNode),
    Path(PathNode),
    Image(ImageNode),
}

#[derive(Debug, Clone)]
pub enum LeafNode {
    Error(ErrorNode),
    Rectangle(RectangleNode),
    Ellipse(EllipseNode),
    Polygon(PolygonNode),
    RegularPolygon(RegularPolygonNode),
    RegularStarPolygon(RegularStarPolygonNode),
    Line(LineNode),
    TextSpan(TextSpanNode),
    Path(PathNode),
    Image(ImageNode),
}

#[derive(Debug, Clone)]
pub struct BaseNode {
    pub id: NodeId,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct ErrorNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub error: String,
    pub opacity: f32,
}

impl ErrorNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub children: Vec<NodeId>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone)]
pub struct ContainerNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub corner_radius: RectangularCornerRadius,
    pub children: Vec<NodeId>,
    pub fill: Paint,
    pub stroke: Option<Paint>,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
    pub clip: bool,
}

impl ContainerNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectangleNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub corner_radius: RectangularCornerRadius,
    pub fill: Paint,
    pub stroke: Paint,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
}

impl RectangleNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size, // height is always 0 (ignored)
    pub stroke: Paint,
    pub stroke_width: f32,
    pub _data_stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

impl LineNode {
    /// line's stoke align is no-op, it's always center. this value is ignored, but will be affected when line transforms to a path.
    pub fn get_stroke_align(&self) -> StrokeAlign {
        StrokeAlign::Center
    }
}

#[derive(Debug, Clone)]
pub struct ImageNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub corner_radius: RectangularCornerRadius,
    pub fill: Paint,
    pub stroke: Paint,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
    pub _ref: String,
}

impl ImageNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }
}

/// A node representing an ellipse shape.
///
/// Like RectangleNode, uses a top-left based coordinate system (x,y,width,height).
/// The ellipse is drawn within the bounding box defined by these coordinates.
#[derive(Debug, Clone)]
pub struct EllipseNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub fill: Paint,
    pub stroke: Paint,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
}

impl EllipseNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BooleanPathOperationNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub op: BooleanPathOperation,
    pub children: Vec<NodeId>,
    pub fill: Paint,
    pub stroke: Option<Paint>,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
}

///
/// SVG Path compatible path node.
///
#[derive(Debug, Clone)]
pub struct PathNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub fill: Paint,
    pub data: String,
    pub stroke: Paint,
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    pub stroke_dash_array: Option<Vec<f32>>,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
}

/// A polygon shape defined by a list of absolute 2D points, following the SVG `<polygon>` model.
///
/// ## Characteristics
/// - Always **closed**: The shape is implicitly closed by connecting the last point back to the first.
/// - For **open shapes**, use a different type such as [`PathNode`] or a potential `PolylineNode`.
///
/// ## Reference
/// Mirrors the behavior of the SVG `<polygon>` element:  
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polygon
#[derive(Debug, Clone)]
pub struct PolygonNode {
    /// Common base metadata and identity.
    pub base: BaseNode,

    /// 2D affine transform matrix applied to the shape.
    pub transform: AffineTransform,

    /// The list of points defining the polygon vertices.
    pub points: Vec<Point>,

    /// The corner radius of the polygon.
    pub corner_radius: f32,

    /// The paint used to fill the interior of the polygon.
    pub fill: Paint,

    /// The stroke paint used to outline the polygon.
    pub stroke: Paint,

    /// The stroke width used to outline the polygon.
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,

    /// Opacity applied to the polygon shape (`0.0` - transparent, `1.0` - opaque).
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
    pub stroke_dash_array: Option<Vec<f32>>,
}

impl PolygonNode {
    pub fn to_path(&self) -> skia_safe::Path {
        cvt::sk_polygon_path(&self.points, self.corner_radius)
    }
}

/// A node representing a regular polygon (triangle, square, pentagon, etc.)
/// that fits inside a bounding box defined by `size`, optionally transformed.
///
/// The polygon is defined by `point_count` (number of sides), and is centered
/// within the box, with even and odd point counts having slightly different
/// initial orientations:
/// - Odd `point_count` (e.g. triangle) aligns the top point to the vertical center top.
/// - Even `point_count` aligns the top edge flat.
///
/// The actual rendering is derived, not stored. Rotation should be applied via `transform`.
///
/// For details on regular polygon mathematics, see: <https://mathworld.wolfram.com/RegularPolygon.html> (implementation varies)
#[derive(Debug, Clone)]
pub struct RegularPolygonNode {
    /// Core identity + metadata
    pub base: BaseNode,

    /// Affine transform applied to this node
    pub transform: AffineTransform,

    /// Bounding box size the polygon is fit into
    pub size: Size,

    /// Number of equally spaced points (>= 3)
    pub point_count: usize,

    /// The corner radius of the polygon.
    pub corner_radius: f32,

    /// Fill paint (solid or gradient)
    pub fill: Paint,

    /// The stroke paint used to outline the polygon.
    pub stroke: Paint,

    /// The stroke width used to outline the polygon.
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    /// Overall node opacity (0.0–1.0)
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
    pub stroke_dash_array: Option<Vec<f32>>,
}

impl RegularPolygonNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }

    pub fn to_polygon(&self) -> PolygonNode {
        let w = self.size.width;
        let h = self.size.height;
        let cx = w / 2.0;
        let cy = h / 2.0;
        let r = w.min(h) / 2.0;
        let angle_offset = if self.point_count % 2 == 0 {
            std::f32::consts::PI / self.point_count as f32
        } else {
            -std::f32::consts::PI / 2.0
        };

        let points: Vec<Point> = (0..self.point_count)
            .map(|i| {
                let theta = (i as f32 / self.point_count as f32) * 2.0 * std::f32::consts::PI
                    + angle_offset;
                let x = cx + r * theta.cos();
                let y = cy + r * theta.sin();
                Point { x, y }
            })
            .collect();

        PolygonNode {
            base: self.base.clone(),
            transform: self.transform,
            points,
            corner_radius: self.corner_radius,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            stroke_align: self.stroke_align,
            opacity: self.opacity,
            blend_mode: self.blend_mode,
            effect: self.effect.clone(),
            stroke_dash_array: self.stroke_dash_array.clone(),
        }
    }
}

/// A regular star polygon node rendered within a bounding box.
///
/// This node represents a geometric star shape composed of alternating outer and inner vertices evenly spaced around a center,
/// forming a symmetric star with `point_count` spikes. Each spike is constructed by alternating between an outer point
/// (determined by the bounding box) and an inner point (scaled by `inner_radius`).
///
/// For details on star polygon mathematics, see: <https://mathworld.wolfram.com/StarPolygon.html>
#[derive(Debug, Clone)]
pub struct RegularStarPolygonNode {
    /// Core identity + metadata
    pub base: BaseNode,

    /// Affine transform applied to this node
    pub transform: AffineTransform,

    /// Bounding box size the polygon is fit into
    pub size: Size,

    /// Number of equally spaced points (>= 3)
    pub point_count: usize,

    /// The `inner_radius` defines the radius of the inner vertices of the star, relative to the center.
    ///
    /// It controls the sharpness of the star's angles:
    /// - A smaller value (closer to 0) results in sharper, spikier points.
    /// - A larger value (closer to or greater than the outer radius) makes the shape closer to a regular polygon with 2 × point_count edges.
    ///
    /// The outer radius is defined by the bounding box (`size`), while the `inner_radius` places the inner points on a second concentric circle.
    /// Unlike `corner_radius`, which affects the rounding of outer corners, `inner_radius` controls the depth of the inner angles between the points.
    pub inner_radius: f32,

    /// The corner radius of the polygon.
    pub corner_radius: f32,

    /// Fill paint (solid or gradient)
    pub fill: Paint,

    /// The stroke paint used to outline the polygon.
    pub stroke: Paint,

    /// The stroke width used to outline the polygon.
    pub stroke_width: f32,
    pub stroke_align: StrokeAlign,
    /// Overall node opacity (0.0–1.0)
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub effect: Option<FilterEffect>,
    pub stroke_dash_array: Option<Vec<f32>>,
}

impl RegularStarPolygonNode {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        }
    }

    pub fn to_polygon(&self) -> PolygonNode {
        let w = self.size.width;
        let h = self.size.height;
        let cx = w / 2.0;
        let cy = h / 2.0;
        let outer_r = cx.min(cy);
        let inner_r = outer_r * self.inner_radius;
        let step = std::f32::consts::PI / self.point_count as f32;
        let start_angle = -std::f32::consts::PI / 2.0;

        let mut points = Vec::with_capacity(self.point_count * 2);
        for i in 0..(self.point_count * 2) {
            let angle = start_angle + i as f32 * step;
            let r = if i % 2 == 0 { outer_r } else { inner_r };
            let x = cx + r * angle.cos();
            let y = cy + r * angle.sin();
            points.push(Point { x, y });
        }

        PolygonNode {
            base: self.base.clone(),
            transform: self.transform,
            points,
            corner_radius: self.corner_radius,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_width: self.stroke_width,
            stroke_align: self.stroke_align,
            opacity: self.opacity,
            blend_mode: self.blend_mode,
            effect: self.effect.clone(),
            stroke_dash_array: self.stroke_dash_array.clone(),
        }
    }
}

/// A node representing a plain text block (non-rich).
/// For multi-style content, see `RichTextNode` (not implemented yet).
#[derive(Debug, Clone)]
pub struct TextSpanNode {
    /// Metadata and identity.
    pub base: BaseNode,

    /// Transform applied to the text container.
    pub transform: AffineTransform,

    /// Layout bounds (used for wrapping and alignment).
    pub size: Size,

    /// Text content (plain UTF-8).
    pub text: String,

    /// Font & fill appearance.
    pub text_style: TextStyle,

    /// Horizontal alignment.
    pub text_align: TextAlign,

    /// Vertical alignment.
    pub text_align_vertical: TextAlignVertical,

    /// Fill paint (solid or gradient)
    pub fill: Paint,

    /// Stroke paint (solid or gradient)
    pub stroke: Option<Paint>,

    /// Stroke width
    pub stroke_width: Option<f32>,
    pub stroke_align: StrokeAlign,
    /// Overall node opacity.
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone)]
#[deprecated(note = "Not implemented yet")]
pub struct TextNode {
    pub base: BaseNode,
    pub transform: AffineTransform,
    pub size: Size,
    pub text: String,
    pub font_size: f32,
    pub fill: Paint,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

// endregion
