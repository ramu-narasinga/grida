use super::schema::*;
use math2::transform::AffineTransform;
use uuid::Uuid;

/// Factory for creating nodes with default values
pub struct NodeFactory;

impl NodeFactory {
    pub fn new() -> Self {
        Self {}
    }

    fn id(&self) -> String {
        // random id
        let id = Uuid::new_v4();
        id.to_string()
    }

    // Internal factory defaults
    const DEFAULT_SIZE: Size = Size {
        width: 100.0,
        height: 100.0,
    };

    const DEFAULT_COLOR: Color = Color(255, 255, 255, 255);
    const DEFAULT_STROKE_COLOR: Color = Color(0, 0, 0, 255);
    const DEFAULT_STROKE_WIDTH: f32 = 1.0;
    const DEFAULT_STROKE_ALIGN: StrokeAlign = StrokeAlign::Inside;
    const DEFAULT_OPACITY: f32 = 1.0;

    fn default_base_node(&self) -> BaseNode {
        BaseNode {
            id: self.id(),
            name: String::new(),
            active: true,
        }
    }

    fn default_solid_paint(color: Color) -> Paint {
        Paint::Solid(SolidPaint {
            color,
            opacity: 1.0,
        })
    }

    /// Creates a new rectangle node with default values
    pub fn create_rectangle_node(&self) -> RectangleNode {
        RectangleNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            corner_radius: RectangularCornerRadius::zero(),
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    /// Creates a new ellipse node with default values
    pub fn create_ellipse_node(&self) -> EllipseNode {
        EllipseNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    /// Creates a new line node with default values
    pub fn create_line_node(&self) -> LineNode {
        LineNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Size {
                width: Self::DEFAULT_SIZE.width,
                height: 0.0,
            },
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            _data_stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Creates a new text span node with default values
    pub fn create_text_span_node(&self) -> TextSpanNode {
        TextSpanNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Size {
                width: Self::DEFAULT_SIZE.width,
                height: 20.0,
            },
            text: String::new(),
            text_style: TextStyle {
                text_decoration: TextDecoration::None,
                font_family: String::from("Arial"),
                font_size: 16.0,
                font_weight: FontWeight::default(),
                italic: false,
                letter_spacing: None,
                line_height: None,
                text_transform: TextTransform::None,
            },
            text_align: TextAlign::Left,
            text_align_vertical: TextAlignVertical::Top,
            fill: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke: None,
            stroke_width: None,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Creates a new group node with default values
    pub fn create_group_node(&self) -> GroupNode {
        GroupNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            children: Vec::new(),
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Creates a new container node with default values
    pub fn create_container_node(&self) -> ContainerNode {
        ContainerNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            corner_radius: RectangularCornerRadius::zero(),
            children: Vec::new(),
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: None,
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
            clip: true,
        }
    }

    /// Creates a new path node with default values
    pub fn create_path_node(&self) -> PathNode {
        PathNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            data: String::new(),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    /// Creates a new regular polygon node with default values
    pub fn create_regular_polygon_node(&self) -> RegularPolygonNode {
        RegularPolygonNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            point_count: 3, // Triangle by default
            corner_radius: 0.0,
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    pub fn create_regular_star_polygon_node(&self) -> RegularStarPolygonNode {
        RegularStarPolygonNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            point_count: 5,    // 5-pointed star by default
            inner_radius: 0.4, // Default inner radius
            corner_radius: 0.0,
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    pub fn create_polygon_node(&self) -> PolygonNode {
        PolygonNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            points: Vec::new(),
            corner_radius: 0.0,
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
        }
    }

    /// Creates a new image node with default values
    pub fn create_image_node(&self) -> ImageNode {
        ImageNode {
            base: self.default_base_node(),
            transform: AffineTransform::identity(),
            size: Self::DEFAULT_SIZE,
            corner_radius: RectangularCornerRadius::zero(),
            fill: Self::default_solid_paint(Self::DEFAULT_COLOR),
            stroke: Self::default_solid_paint(Self::DEFAULT_STROKE_COLOR),
            stroke_width: Self::DEFAULT_STROKE_WIDTH,
            stroke_align: Self::DEFAULT_STROKE_ALIGN,
            stroke_dash_array: None,
            opacity: Self::DEFAULT_OPACITY,
            blend_mode: BlendMode::Normal,
            effect: None,
            _ref: String::new(),
        }
    }
}
