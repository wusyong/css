use std::{fmt, sync::Arc};

/// Currently hard-coded: Height of one em in pixels
pub const EM_HEIGHT: f32 = 16.0;
pub const PT_TO_PX: f32 = 96.0 / 72.0;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CombinedPropertyType {
    BorderRadius,
    Overflow,
    Margin,
    Border,
    BorderLeft,
    BorderRight,
    BorderTop,
    BorderBottom,
    Padding,
    BoxShadow,
    BackgroundColor,
    BackgroundImage,
}

impl CombinedPropertyType {
    /// Parses a CSS key, such as `width` from a string:
    pub fn from_str(input: &str) -> Option<Self> {
        let input = input.trim();
        match input {
            "border-radius" => Some(CombinedPropertyType::BorderRadius),
            "overflow" => Some(CombinedPropertyType::Overflow),
            "padding" => Some(CombinedPropertyType::Padding),
            "margin" => Some(CombinedPropertyType::Margin),
            "border" => Some(CombinedPropertyType::Border),
            "border-left" => Some(CombinedPropertyType::BorderLeft),
            "border-right" => Some(CombinedPropertyType::BorderRight),
            "border-top" => Some(CombinedPropertyType::BorderTop),
            "border-bottom" => Some(CombinedPropertyType::BorderBottom),
            "box-shadow" => Some(CombinedPropertyType::BoxShadow),
            "background-color" => Some(CombinedPropertyType::BackgroundColor),
            "background-image" => Some(CombinedPropertyType::BackgroundImage),
            _ => None,
        }
    }

    /// Returns the original string that was used to construct this `CssPropertyType`.
    pub const fn to_str(&self) -> &'static str {
        match self {
            CombinedPropertyType::BorderRadius => "border-radius",
            CombinedPropertyType::Overflow => "overflow",
            CombinedPropertyType::Margin => "margin",
            CombinedPropertyType::Border => "border",
            CombinedPropertyType::BorderLeft => "border-left",
            CombinedPropertyType::BorderRight => "border-right",
            CombinedPropertyType::BorderTop => "border-top",
            CombinedPropertyType::BorderBottom => "border-bottom",
            CombinedPropertyType::Padding => "padding",
            CombinedPropertyType::BoxShadow => "box-shadow",
            CombinedPropertyType::BackgroundColor => "background-color",
            CombinedPropertyType::BackgroundImage => "background-image",
        }
    }
}

/// Represents a CSS key (for example `"border-radius"` => `BorderRadius`).
/// You can also derive this key from a `Property` by calling `Property::get_type()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PType {
    TextColor,
    FontSize,
    FontFamily,
    TextAlign,
    LetterSpacing,
    LineHeight,
    WordSpacing,
    TabWidth,
    Cursor,
    Display,
    Float,
    BoxSizing,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    Position,
    Top,
    Right,
    Left,
    Bottom,
    FlexWrap,
    FlexDirection,
    FlexGrow,
    FlexShrink,
    JustifyContent,
    AlignItems,
    AlignContent,
    BackgroundContent,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    OverflowX,
    OverflowY,
    PaddingTop,
    PaddingLeft,
    PaddingRight,
    PaddingBottom,
    MarginTop,
    MarginLeft,
    MarginRight,
    MarginBottom,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderTopColor,
    BorderRightColor,
    BorderLeftColor,
    BorderBottomColor,
    BorderTopStyle,
    BorderRightStyle,
    BorderLeftStyle,
    BorderBottomStyle,
    BorderTopWidth,
    BorderRightWidth,
    BorderLeftWidth,
    BorderBottomWidth,
    BoxShadowLeft,
    BoxShadowRight,
    BoxShadowTop,
    BoxShadowBottom,
    ScrollbarStyle,
    Opacity,
    Transform,
    TransformOrigin,
    PerspectiveOrigin,
    BackfaceVisibility,
    MixBlendMode,
    Filter,
    BackdropFilter,
    TextShadow,
}

impl Property {
    // /// Returns the original string that was used to construct this `CssPropertyType`.
    // pub fn from_str(&self, input: &str) -> Option<Self> {
    //     match input {
    //         "color" => Some(PType::TextColor),
    //         "font-size" => Some(PType::FontSize),
    //         "font-family" => Some(PType::FontFamily),
    //         "text-align" => Some(PType::TextAlign),
    //         "letter-spacing" => Some(PType::LetterSpacing),
    //         "line-height" => Some(PType::LineHeight),
    //         "word-spacing" => Some(PType::WordSpacing),
    //         "tab-width" => Some(PType::TabWidth),
    //         "cursor" => Some(PType::Cursor),
    //         "display" => Some(PType::Display),
    //         "float" => Some(PType::Float),
    //         "box-sizing" => Some(PType::BoxSizing),
    //         "width" => Some(PType::Width),
    //         "height" => Some(PType::Height),
    //         "min-width" => Some(PType::MinWidth),
    //         "min-height" => Some(PType::MinHeight),
    //         "max-width" => Some(PType::MaxWidth),
    //         "max-height" => Some(PType::MaxHeight),
    //         "position" => Some(PType::Position),
    //         "top" => Some(PType::Top),
    //         "right" => Some(PType::Right),
    //         "left" => Some(PType::Left),
    //         "bottom" => Some(PType::Bottom),
    //         "flex-wrap" => Some(PType::FlexWrap),
    //         "flex-direction" => Some(PType::FlexDirection),
    //         "flex-grow" => Some(PType::FlexGrow),
    //         "flex-shrink" => Some(PType::FlexShrink),
    //         "justify-content" => Some(PType::JustifyContent),
    //         "align-items" => Some(PType::AlignItems),
    //         "align-content" => Some(PType::AlignContent),
    //         "background" => Some(PType::BackgroundContent),
    //         "background-position" => Some(PType::BackgroundPosition),
    //         "background-size" => Some(PType::BackgroundSize),
    //         "background-repeat" => Some(PType::BackgroundRepeat),
    //         "overflow-x" => Some(PType::OverflowX),
    //         "overflow-y" => Some(PType::OverflowY),
    //         "padding-top" => Some(PType::PaddingTop),
    //         "padding-left" => Some(PType::PaddingLeft),
    //         "padding-right" => Some(PType::PaddingRight),
    //         "padding-bottom" => Some(PType::PaddingBottom),
    //         "margin-top" => Some(PType::MarginTop),
    //         "margin-left" => Some(PType::MarginLeft),
    //         "margin-right" => Some(PType::MarginRight),
    //         "margin-bottom" => Some(PType::MarginBottom),
    //         "border-top-left-radius" => Some(PType::BorderTopLeftRadius),
    //         "border-top-right-radius" => Some(PType::BorderTopRightRadius),
    //         "border-bottom-left-radius" => Some(PType::BorderBottomLeftRadius),
    //         "border-bottom-right-radius" => Some(PType::BorderBottomRightRadius),
    //         "border-top-color" => Some(PType::BorderTopColor),
    //         "border-right-color" => Some(PType::BorderRightColor),
    //         "border-left-color" => Some(PType::BorderLeftColor),
    //         "border-bottom-color" => Some(PType::BorderBottomColor),
    //         "border-top-style" => Some(PType::BorderTopStyle),
    //         "border-right-style" => Some(PType::BorderRightStyle),
    //         "border-left-style" => Some(PType::BorderLeftStyle),
    //         "border-bottom-style" => Some(PType::BorderBottomStyle),
    //         "border-top-width" => Some(PType::BorderTopWidth),
    //         "border-right-width" => Some(PType::BorderRightWidth),
    //         "border-left-width" => Some(PType::BorderLeftWidth),
    //         "border-bottom-width" => Some(PType::BorderBottomWidth),
    //         "-box-shadow-left" => Some(PType::BoxShadowLeft),
    //         "-box-shadow-right" => Some(PType::BoxShadowRight),
    //         "-box-shadow-top" => Some(PType::BoxShadowTop),
    //         "-box-shadow-bottom" => Some(PType::BoxShadowBottom),
    //         "-scrollbar-style" => Some(PType::ScrollbarStyle),
    //         "opacity" => Some(PType::Opacity),
    //         "transform" => Some(PType::Transform),
    //         "transform-origin" => Some(PType::TransformOrigin),
    //         "perspective-origin" => Some(PType::PerspectiveOrigin),
    //         "backface-visibility" => Some(PType::BackfaceVisibility),
    //         "mix-blend-mode" => Some(PType::MixBlendMode),
    //         "filter" => Some(PType::Filter),
    //         "backdrop-filter" => Some(PType::BackdropFilter),
    //         "text-shadow" => Some(PType::TextShadow),
    //         _ => None,
    //     }
    // }
    //
    // /// Returns the original string that was used to construct this `CssPropertyType`.
    // pub const fn to_str(&self) -> &'static str {
    //     match self {
    //         PType::TextColor => "color",
    //         PType::FontSize => "font-size",
    //         PType::FontFamily => "font-family",
    //         PType::TextAlign => "text-align",
    //         PType::LetterSpacing => "letter-spacing",
    //         PType::LineHeight => "line-height",
    //         PType::WordSpacing => "word-spacing",
    //         PType::TabWidth => "tab-width",
    //         PType::Cursor => "cursor",
    //         PType::Display => "display",
    //         PType::Float => "float",
    //         PType::BoxSizing => "box-sizing",
    //         PType::Width => "width",
    //         PType::Height => "height",
    //         PType::MinWidth => "min-width",
    //         PType::MinHeight => "min-height",
    //         PType::MaxWidth => "max-width",
    //         PType::MaxHeight => "max-height",
    //         PType::Position => "position",
    //         PType::Top => "top",
    //         PType::Right => "right",
    //         PType::Left => "left",
    //         PType::Bottom => "bottom",
    //         PType::FlexWrap => "flex-wrap",
    //         PType::FlexDirection => "flex-direction",
    //         PType::FlexGrow => "flex-grow",
    //         PType::FlexShrink => "flex-shrink",
    //         PType::JustifyContent => "justify-content",
    //         PType::AlignItems => "align-items",
    //         PType::AlignContent => "align-content",
    //         PType::BackgroundContent => "background",
    //         PType::BackgroundPosition => "background-position",
    //         PType::BackgroundSize => "background-size",
    //         PType::BackgroundRepeat => "background-repeat",
    //         PType::OverflowX => "overflow-x",
    //         PType::OverflowY => "overflow-y",
    //         PType::PaddingTop => "padding-top",
    //         PType::PaddingLeft => "padding-left",
    //         PType::PaddingRight => "padding-right",
    //         PType::PaddingBottom => "padding-bottom",
    //         PType::MarginTop => "margin-top",
    //         PType::MarginLeft => "margin-left",
    //         PType::MarginRight => "margin-right",
    //         PType::MarginBottom => "margin-bottom",
    //         PType::BorderTopLeftRadius => "border-top-left-radius",
    //         PType::BorderTopRightRadius => "border-top-right-radius",
    //         PType::BorderBottomLeftRadius => "border-bottom-left-radius",
    //         PType::BorderBottomRightRadius => "border-bottom-right-radius",
    //         PType::BorderTopColor => "border-top-color",
    //         PType::BorderRightColor => "border-right-color",
    //         PType::BorderLeftColor => "border-left-color",
    //         PType::BorderBottomColor => "border-bottom-color",
    //         PType::BorderTopStyle => "border-top-style",
    //         PType::BorderRightStyle => "border-right-style",
    //         PType::BorderLeftStyle => "border-left-style",
    //         PType::BorderBottomStyle => "border-bottom-style",
    //         PType::BorderTopWidth => "border-top-width",
    //         PType::BorderRightWidth => "border-right-width",
    //         PType::BorderLeftWidth => "border-left-width",
    //         PType::BorderBottomWidth => "border-bottom-width",
    //         PType::BoxShadowLeft => "-box-shadow-left",
    //         PType::BoxShadowRight => "-box-shadow-right",
    //         PType::BoxShadowTop => "-box-shadow-top",
    //         PType::BoxShadowBottom => "-box-shadow-bottom",
    //         PType::ScrollbarStyle => "-scrollbar-style",
    //         PType::Opacity => "opacity",
    //         PType::Transform => "transform",
    //         PType::TransformOrigin => "transform-origin",
    //         PType::PerspectiveOrigin => "perspective-origin",
    //         PType::BackfaceVisibility => "backface-visibility",
    //         PType::MixBlendMode => "mix-blend-mode",
    //         PType::Filter => "filter",
    //         PType::BackdropFilter => "backdrop-filter",
    //         PType::TextShadow => "text-shadow",
    //     }
    // }

    /// Returns whether this property will be inherited during cascading
    pub const fn is_inheritable(&self) -> bool {
        use self::Property::*;
        match self {
            TextColor(_) | FontFamily(_) | FontSize(_) | LineHeight(_) | TextAlign(_) => true,
            _ => false,
        }
    }

    /// Returns whether this property can trigger a re-layout (important for incremental layout and caching layouted DOMs).
    pub const fn can_relayout(&self) -> bool {
        use self::Property::*;

        // Since the border can be larger than the content,
        // in which case the content needs to be re-layouted, assume true for Border

        // FontFamily, FontSize, LetterSpacing and LineHeight can affect
        // the text layout and therefore the screen layout

        match self {
            TextColor(_)
            | Cursor(_)
            | BackgroundContent(_)
            | BackgroundPosition(_)
            | BackgroundSize(_)
            | BackgroundRepeat(_)
            | BorderTopLeftRadius(_)
            | BorderTopRightRadius(_)
            | BorderBottomLeftRadius(_)
            | BorderBottomRightRadius(_)
            | BorderTopColor(_)
            | BorderRightColor(_)
            | BorderLeftColor(_)
            | BorderBottomColor(_)
            | BorderTopStyle(_)
            | BorderRightStyle(_)
            | BorderLeftStyle(_)
            | BorderBottomStyle(_)
            | BoxShadowLeft(_)
            | BoxShadowRight(_)
            | BoxShadowTop(_)
            | BoxShadowBottom(_)
            | ScrollbarStyle(_)
            | Opacity(_)
            | Transform(_)
            | TransformOrigin(_)
            | PerspectiveOrigin(_)
            | BackfaceVisibility(_)
            | MixBlendMode(_)
            | Filter(_)
            | BackdropFilter(_)
            | TextShadow(_) => false,
            _ => true,
        }
    }

    /// Returns whether the property is a GPU property (currently only opacity and transforms)
    pub const fn is_gpu_property(&self) -> bool {
        match self {
            Property::Opacity(_) |
            Property::Transform(_) /* | CssPropertyType::Color */ => true,
            _ => false
        }
    }
}

/// A CSS property
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value<T> {
    Auto,
    None,
    Initial,
    Inherit,
    Exact(T),
}

impl<T> From<T> for Value<T> {
    fn from(c: T) -> Self {
        Value::Exact(c)
    }
}

impl<T> Value<T> {
    /// Transforms a `CssPropertyValue<T>` into a `CssPropertyValue<U>` by applying a mapping function
    pub fn map_property<F: Fn(T) -> U, U>(self, map_fn: F) -> Value<U> {
        match self {
            Value::Exact(c) => Value::Exact(map_fn(c)),
            Value::Auto => Value::Auto,
            Value::None => Value::None,
            Value::Initial => Value::Initial,
            Value::Inherit => Value::Inherit,
        }
    }

    #[inline]
    pub const fn as_property(&self) -> Option<&T> {
        match self {
            Value::Exact(c) => Some(c),
            _ => None,
        }
    }

    #[inline]
    pub fn to_property(self) -> Option<T> {
        match self {
            Value::Exact(c) => Some(c),
            _ => None,
        }
    }

    #[inline]
    pub const fn is_auto(&self) -> bool {
        match self {
            Value::Auto => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn is_none(&self) -> bool {
        match self {
            Value::None => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn is_initial(&self) -> bool {
        match self {
            Value::Initial => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn is_inherit(&self) -> bool {
        match self {
            Value::Inherit => true,
            _ => false,
        }
    }
}

impl<T: Default> Value<T> {
    #[inline]
    pub fn to_property_or_default(self) -> Option<T> {
        match self {
            Value::Auto | Value::Initial => Some(T::default()),
            Value::Exact(c) => Some(c),
            Value::None | Value::Inherit => None,
        }
    }
}

impl<T: Default> Default for Value<T> {
    #[inline]
    fn default() -> Self {
        Value::Exact(T::default())
    }
}

/// Represents one parsed CSS key-value pair, such as `"width: 20px"` => `CssProperty::Width(LayoutWidth::px(20.0))`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Property {
    TextColor(Value<StyleTextColor>),
    FontSize(Value<StyleFontSize>),
    FontFamily(Value<Vec<StyleFontFamily>>),
    TextAlign(Value<StyleTextAlign>),
    LetterSpacing(Value<StyleLetterSpacing>),
    LineHeight(Value<StyleLineHeight>),
    WordSpacing(Value<StyleWordSpacing>),
    TabWidth(Value<StyleTabWidth>),
    Cursor(Value<StyleCursor>),
    Display(Value<LayoutDisplay>),
    Float(Value<LayoutFloat>),
    BoxSizing(Value<LayoutBoxSizing>),
    Width(Value<LayoutWidth>),
    Height(Value<LayoutHeight>),
    MinWidth(Value<LayoutMaxWidth>),
    MinHeight(Value<LayoutMinHeight>),
    MaxWidth(Value<LayoutMaxWidth>),
    MaxHeight(Value<LayoutMaxHeight>),
    Position(Value<LayoutPosition>),
    Top(Value<LayoutTop>),
    Right(Value<LayoutRight>),
    Left(Value<LayoutLeft>),
    Bottom(Value<LayoutBottom>),
    FlexWrap(Value<LayoutFlexWrap>),
    FlexDirection(Value<LayoutFlexDirection>),
    FlexGrow(Value<LayoutFlexGrow>),
    FlexShrink(Value<LayoutFlexShrink>),
    JustifyContent(Value<LayoutJustifyContent>),
    AlignItems(Value<LayoutAlignItems>),
    AlignContent(Value<LayoutAlignContent>),
    BackgroundContent(Value<Vec<StyleBackgroundContent>>),
    BackgroundPosition(Value<Vec<StyleBackgroundPosition>>),
    BackgroundSize(Value<Vec<StyleBackgroundSize>>),
    BackgroundRepeat(Value<Vec<StyleBackgroundRepeat>>),
    OverflowX(Value<LayoutOverflow>),
    OverflowY(Value<LayoutOverflow>),
    PaddingTop(Value<LayoutPaddingTop>),
    PaddingLeft(Value<LayoutPaddingLeft>),
    PaddingRight(Value<LayoutPaddingRight>),
    PaddingBottom(Value<LayoutPaddingBottom>),
    MarginTop(Value<LayoutMarginTop>),
    MarginLeft(Value<LayoutMarginLeft>),
    MarginRight(Value<LayoutMarginRight>),
    MarginBottom(Value<LayoutMarginBottom>),
    BorderTopLeftRadius(Value<StyleBorderTopLeftRadius>),
    BorderTopRightRadius(Value<StyleBorderTopRightRadius>),
    BorderBottomLeftRadius(Value<StyleBorderBottomLeftRadius>),
    BorderBottomRightRadius(Value<StyleBorderBottomRightRadius>),
    BorderTopColor(Value<StyleBorderTopColor>),
    BorderRightColor(Value<StyleBorderRightColor>),
    BorderLeftColor(Value<StyleBorderLeftColor>),
    BorderBottomColor(Value<StyleBorderBottomColor>),
    BorderTopStyle(Value<StyleBorderTopStyle>),
    BorderRightStyle(Value<StyleBorderRightStyle>),
    BorderLeftStyle(Value<StyleBorderLeftStyle>),
    BorderBottomStyle(Value<StyleBorderBottomStyle>),
    BorderTopWidth(Value<LayoutBorderTopWidth>),
    BorderRightWidth(Value<LayoutBorderRightWidth>),
    BorderLeftWidth(Value<LayoutBorderLeftWidth>),
    BorderBottomWidth(Value<LayoutBorderBottomWidth>),
    BoxShadowLeft(Value<StyleBoxShadow>),
    BoxShadowRight(Value<StyleBoxShadow>),
    BoxShadowTop(Value<StyleBoxShadow>),
    BoxShadowBottom(Value<StyleBoxShadow>),
    ScrollbarStyle(Value<ScrollbarStyle>),
    Opacity(Value<StyleOpacity>),
    Transform(Value<Vec<StyleTransform>>),
    TransformOrigin(Value<StyleTransformOrigin>),
    PerspectiveOrigin(Value<StylePerspectiveOrigin>),
    BackfaceVisibility(Value<StyleBackfaceVisibility>),
    MixBlendMode(Value<StyleMixBlendMode>),
    Filter(Value<Vec<StyleFilter>>),
    BackdropFilter(Value<Vec<StyleFilter>>),
    TextShadow(Value<StyleBoxShadow>),
}

/// Only used for calculations: Point coordinate (x, y) in layout space.
#[derive(Debug, Copy, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct LayoutPoint {
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for LayoutPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl LayoutPoint {
    #[inline(always)]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }
}

/// Only used for calculations: Size (width, height) in layout space.
#[derive(Debug, Copy, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct LayoutSize {
    pub width: isize,
    pub height: isize,
}

impl fmt::Display for LayoutSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl LayoutSize {
    #[inline(always)]
    pub const fn new(width: isize, height: isize) -> Self {
        Self { width, height }
    }
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }
    #[inline]
    pub fn round(width: f32, height: f32) -> Self {
        Self {
            width: libm::roundf(width) as isize,
            height: libm::roundf(height) as isize,
        }
    }
}

/// Only used for calculations: Rectangle (x, y, width, height) in layout space.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct LayoutRect {
    pub origin: LayoutPoint,
    pub size: LayoutSize,
}

impl fmt::Display for LayoutRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.size, self.origin)
    }
}

impl LayoutRect {
    #[inline(always)]
    pub const fn new(origin: LayoutPoint, size: LayoutSize) -> Self {
        Self { origin, size }
    }
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::new(LayoutPoint::zero(), LayoutSize::zero())
    }
    #[inline(always)]
    pub const fn max_x(&self) -> isize {
        self.origin.x + self.size.width
    }
    #[inline(always)]
    pub const fn min_x(&self) -> isize {
        self.origin.x
    }
    #[inline(always)]
    pub const fn max_y(&self) -> isize {
        self.origin.y + self.size.height
    }
    #[inline(always)]
    pub const fn min_y(&self) -> isize {
        self.origin.y
    }
    #[inline(always)]
    pub const fn width(&self) -> isize {
        self.max_x() - self.min_x()
    }
    #[inline(always)]
    pub const fn height(&self) -> isize {
        self.max_y() - self.min_y()
    }

    pub const fn contains(&self, other: &LayoutPoint) -> bool {
        self.min_x() <= other.x
            && other.x < self.max_x()
            && self.min_y() <= other.y
            && other.y < self.max_y()
    }

    pub fn contains_f32(&self, other_x: f32, other_y: f32) -> bool {
        self.min_x() as f32 <= other_x
            && other_x < self.max_x() as f32
            && self.min_y() as f32 <= other_y
            && other_y < self.max_y() as f32
    }

    /// Same as `contains()`, but returns the (x, y) offset of the hit point
    ///
    /// On a regular computer this function takes ~3.2ns to run
    #[inline]
    pub const fn hit_test(&self, other: &LayoutPoint) -> Option<LayoutPoint> {
        let dx_left_edge = other.x - self.min_x();
        let dx_right_edge = self.max_x() - other.x;
        let dy_top_edge = other.y - self.min_y();
        let dy_bottom_edge = self.max_y() - other.y;
        if dx_left_edge > 0 && dx_right_edge > 0 && dy_top_edge > 0 && dy_bottom_edge > 0 {
            Some(LayoutPoint::new(dx_left_edge, dy_top_edge))
        } else {
            None
        }
    }

    /// Faster union for a Vec<LayoutRect>
    #[inline]
    pub fn union<I: Iterator<Item = Self>>(mut rects: I) -> Option<Self> {
        let first = rects.next()?;

        let mut max_width = first.size.width;
        let mut max_height = first.size.height;
        let mut min_x = first.origin.x;
        let mut min_y = first.origin.y;

        while let Some(Self {
            origin: LayoutPoint { x, y },
            size: LayoutSize { width, height },
        }) = rects.next()
        {
            let cur_lower_right_x = x + width;
            let cur_lower_right_y = y + height;
            max_width = max_width.max(cur_lower_right_x - min_x);
            max_height = max_height.max(cur_lower_right_y - min_y);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
        }

        Some(Self {
            origin: LayoutPoint { x: min_x, y: min_y },
            size: LayoutSize {
                width: max_width,
                height: max_height,
            },
        })
    }

    // Returns the scroll rect (not the union rect) of the parent / children
    #[inline]
    pub fn get_scroll_rect<I: Iterator<Item = Self>>(&self, children: I) -> Option<Self> {
        let children_union = Self::union(children)?;
        Self::union([*self, children_union].iter().map(|r| *r))
    }

    // Returns if b overlaps a
    #[inline(always)]
    pub const fn contains_rect(&self, b: &LayoutRect) -> bool {
        let a = self;

        let a_x = a.origin.x;
        let a_y = a.origin.y;
        let a_width = a.size.width;
        let a_height = a.size.height;

        let b_x = b.origin.x;
        let b_y = b.origin.y;
        let b_width = b.size.width;
        let b_height = b.size.height;

        b_x >= a_x
            && b_y >= a_y
            && b_x + b_width <= a_x + a_width
            && b_y + b_height <= a_y + a_height
    }
}

/// u8-based color, range 0 to 255 (similar to webrenders ColorU)
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct ColorU {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for ColorU {
    fn default() -> Self {
        ColorU::BLACK
    }
}

impl fmt::Display for ColorU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.r,
            self.g,
            self.b,
            self.a as f32 / 255.0
        )
    }
}

impl ColorU {
    pub const ALPHA_TRANSPARENT: u8 = 0;
    pub const ALPHA_OPAQUE: u8 = 255;

    pub const RED: ColorU = ColorU {
        r: 255,
        g: 0,
        b: 0,
        a: Self::ALPHA_OPAQUE,
    };
    pub const GREEN: ColorU = ColorU {
        r: 0,
        g: 255,
        b: 0,
        a: Self::ALPHA_OPAQUE,
    };
    pub const BLUE: ColorU = ColorU {
        r: 0,
        g: 0,
        b: 255,
        a: Self::ALPHA_OPAQUE,
    };
    pub const WHITE: ColorU = ColorU {
        r: 255,
        g: 255,
        b: 255,
        a: Self::ALPHA_OPAQUE,
    };
    pub const BLACK: ColorU = ColorU {
        r: 0,
        g: 0,
        b: 0,
        a: Self::ALPHA_OPAQUE,
    };
    pub const TRANSPARENT: ColorU = ColorU {
        r: 0,
        g: 0,
        b: 0,
        a: Self::ALPHA_TRANSPARENT,
    };

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            r: libm::roundf(self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: libm::roundf(self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: libm::roundf(self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            a: libm::roundf(self.a as f32 + (other.a as f32 - self.a as f32) * t) as u8,
        }
    }

    pub const fn has_alpha(&self) -> bool {
        self.a != Self::ALPHA_OPAQUE
    }

    pub fn to_hash(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }

    pub fn write_hash(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r, self.g, self.b, self.a
        )
    }
}

/// Multiplier for floating point accuracy. Elements such as px or %
/// are only accurate until a certain number of decimal points, therefore
/// they have to be casted to isizes in order to make the f32 values
/// hash-able: Css has a relatively low precision here, roughly 5 digits, i.e
/// `1.00001 == 1.0`
const FP_PRECISION_MULTIPLIER: f32 = 1000.0;
const FP_PRECISION_MULTIPLIER_CONST: isize = FP_PRECISION_MULTIPLIER as isize;

/// Wrapper around an f32 value that is internally casted to an isize,
/// in order to provide hash-ability (to avoid numerical instability).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FloatValue(pub isize);

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl FloatValue {
    /// Same as `FloatValue::new()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_new(value: isize) -> Self {
        Self(value * FP_PRECISION_MULTIPLIER_CONST)
    }

    #[inline]
    pub fn new(value: f32) -> Self {
        Self((value * FP_PRECISION_MULTIPLIER) as isize)
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.0 as f32 / FP_PRECISION_MULTIPLIER
    }

    #[inline]
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        let self_val_f32 = self.get();
        let other_val_f32 = other.get();
        let interpolated = self_val_f32 + ((other_val_f32 - self_val_f32) * t);
        Self::new(interpolated)
    }
}

impl From<f32> for FloatValue {
    #[inline]
    fn from(val: f32) -> Self {
        Self::new(val)
    }
}

/// FloatValue, but associated with a certain metric (i.e. px, em, etc.)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AngleValue {
    pub metric: AngleMetric,
    pub number: FloatValue,
}

impl fmt::Display for AngleValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.metric)
    }
}

impl AngleValue {
    #[inline]
    pub const fn zero() -> Self {
        const ZERO_DEG: AngleValue = AngleValue::const_deg(0);
        ZERO_DEG
    }

    /// Same as `PixelValue::px()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_deg(value: isize) -> Self {
        Self::const_from_metric(AngleMetric::Degree, value)
    }

    /// Same as `PixelValue::em()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_rad(value: isize) -> Self {
        Self::const_from_metric(AngleMetric::Radians, value)
    }

    /// Same as `PixelValue::pt()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_grad(value: isize) -> Self {
        Self::const_from_metric(AngleMetric::Grad, value)
    }

    /// Same as `PixelValue::pt()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_turn(value: isize) -> Self {
        Self::const_from_metric(AngleMetric::Turn, value)
    }

    #[inline]
    pub fn const_percent(value: isize) -> Self {
        Self::const_from_metric(AngleMetric::Percent, value)
    }

    #[inline]
    pub const fn const_from_metric(metric: AngleMetric, value: isize) -> Self {
        Self {
            metric,
            number: FloatValue::const_new(value),
        }
    }

    #[inline]
    pub fn deg(value: f32) -> Self {
        Self::from_metric(AngleMetric::Degree, value)
    }

    #[inline]
    pub fn rad(value: f32) -> Self {
        Self::from_metric(AngleMetric::Radians, value)
    }

    #[inline]
    pub fn grad(value: f32) -> Self {
        Self::from_metric(AngleMetric::Grad, value)
    }

    #[inline]
    pub fn turn(value: f32) -> Self {
        Self::from_metric(AngleMetric::Turn, value)
    }

    #[inline]
    pub fn percent(value: f32) -> Self {
        Self::from_metric(AngleMetric::Percent, value)
    }

    #[inline]
    pub fn from_metric(metric: AngleMetric, value: f32) -> Self {
        Self {
            metric,
            number: FloatValue::new(value),
        }
    }

    /// Returns the value of the AngleMetric in degrees
    #[inline]
    pub fn to_degrees(&self) -> f32 {
        let val = match self.metric {
            AngleMetric::Degree => self.number.get(),
            AngleMetric::Radians => self.number.get() / 400.0 * 360.0,
            AngleMetric::Grad => self.number.get() / (2.0 * core::f32::consts::PI) * 360.0,
            AngleMetric::Turn => self.number.get() * 360.0,
            AngleMetric::Percent => self.number.get() / 100.0 * 360.0,
        };

        // clamp the degree to a positive value from 0 to 360 (so 410deg = 50deg)
        let mut val = val % 360.0;
        if val < 0.0 {
            val = 360.0 + val;
        }
        val
    }
}

/// Wrapper around FloatValue, represents a percentage instead
/// of just being a regular floating-point value, i.e `5` = `5%`
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PercentageValue(FloatValue);

impl fmt::Display for PercentageValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.get())
    }
}

impl PercentageValue {
    /// Same as `PercentageValue::new()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_new(value: isize) -> Self {
        Self(FloatValue::const_new(value))
    }

    #[inline]
    pub fn new(value: f32) -> Self {
        Self(value.into())
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.0.get()
    }

    #[inline]
    pub fn normalized(&self) -> f32 {
        self.get() / 100.0
    }

    #[inline]
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self(self.0.interpolate(&other.0, t))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AngleMetric {
    Degree,
    Radians,
    Grad,
    Turn,
    Percent,
}

impl Default for AngleMetric {
    fn default() -> AngleMetric {
        AngleMetric::Degree
    }
}

impl fmt::Display for AngleMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::AngleMetric::*;
        match self {
            Degree => write!(f, "deg"),
            Radians => write!(f, "rad"),
            Grad => write!(f, "grad"),
            Turn => write!(f, "turn"),
            Percent => write!(f, "%"),
        }
    }
}

/// Enum representing the metric associated with a number (px, pt, em, etc.)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SizeMetric {
    Px,
    Pt,
    Em,
    Percent,
}

impl Default for SizeMetric {
    fn default() -> Self {
        SizeMetric::Px
    }
}

impl fmt::Display for SizeMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SizeMetric::*;
        match self {
            Px => write!(f, "px"),
            Pt => write!(f, "pt"),
            Em => write!(f, "pt"),
            Percent => write!(f, "%"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelValue {
    pub metric: SizeMetric,
    pub number: FloatValue,
}

impl fmt::Display for PixelValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.metric)
    }
}

impl PixelValue {
    #[inline]
    pub const fn zero() -> Self {
        const ZERO_PX: PixelValue = PixelValue::const_px(0);
        ZERO_PX
    }

    /// Same as `PixelValue::px()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_px(value: isize) -> Self {
        Self::const_from_metric(SizeMetric::Px, value)
    }

    /// Same as `PixelValue::em()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_em(value: isize) -> Self {
        Self::const_from_metric(SizeMetric::Em, value)
    }

    /// Same as `PixelValue::pt()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_pt(value: isize) -> Self {
        Self::const_from_metric(SizeMetric::Pt, value)
    }

    /// Same as `PixelValue::pt()`, but only accepts whole numbers,
    /// since using `f32` in const fn is not yet stabilized.
    #[inline]
    pub const fn const_percent(value: isize) -> Self {
        Self::const_from_metric(SizeMetric::Percent, value)
    }

    #[inline]
    pub const fn const_from_metric(metric: SizeMetric, value: isize) -> Self {
        Self {
            metric,
            number: FloatValue::const_new(value),
        }
    }

    #[inline]
    pub fn px(value: f32) -> Self {
        Self::from_metric(SizeMetric::Px, value)
    }

    #[inline]
    pub fn em(value: f32) -> Self {
        Self::from_metric(SizeMetric::Em, value)
    }

    #[inline]
    pub fn pt(value: f32) -> Self {
        Self::from_metric(SizeMetric::Pt, value)
    }

    #[inline]
    pub fn percent(value: f32) -> Self {
        Self::from_metric(SizeMetric::Percent, value)
    }

    #[inline]
    pub fn from_metric(metric: SizeMetric, value: f32) -> Self {
        Self {
            metric,
            number: FloatValue::new(value),
        }
    }

    #[inline]
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        if self.metric == other.metric {
            Self {
                metric: self.metric,
                number: self.number.interpolate(&other.number, t),
            }
        } else {
            // TODO: how to interpolate between different metrics
            // (interpolate between % and em? - currently impossible)
            let self_px_interp = self.to_pixels(0.0);
            let other_px_interp = other.to_pixels(0.0);
            Self::from_metric(
                SizeMetric::Px,
                self_px_interp + (other_px_interp - self_px_interp) * t,
            )
        }
    }

    /// Returns the value of the SizeMetric in pixels
    #[inline]
    pub fn to_pixels(&self, percent_resolve: f32) -> f32 {
        match self.metric {
            SizeMetric::Px => self.number.get(),
            SizeMetric::Pt => self.number.get() * PT_TO_PX,
            SizeMetric::Em => self.number.get() * EM_HEIGHT,
            SizeMetric::Percent => self.number.get() / 100.0 * percent_resolve,
        }
    }
}

/// Represents a parsed pair of `5px, 10px` values - useful for border radius calculation
#[derive(Default, Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct PixelSize {
    pub width: PixelValue,
    pub height: PixelValue,
}

impl PixelSize {
    pub const fn new(width: PixelValue, height: PixelValue) -> Self {
        Self { width, height }
    }

    pub const fn zero() -> Self {
        Self::new(PixelValue::const_px(0), PixelValue::const_px(0))
    }
}

/// Offsets of the border-width calculations
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct LayoutSideOffsets {
    pub top: FloatValue,
    pub right: FloatValue,
    pub bottom: FloatValue,
    pub left: FloatValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTextColor(pub ColorU);

impl StyleTextColor {
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self(self.0.interpolate(&other.0, t))
    }
}

/// Represents a `font-size` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct StyleFontSize(pub PixelValue);

impl Default for StyleFontSize {
    fn default() -> Self {
        Self(PixelValue::const_em(1))
    }
}

/// Represents a `font-family` attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleFontFamily {
    /// Native font, such as "Webly Sleeky UI", "monospace", etc.
    System(String),
    /// Font loaded from a file
    File(String),
    // /// Reference-counted, already-decoded font,
    // /// so that specific DOM nodes are required to use this font
    Ref(Arc<FontData>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontData(pub Vec<u8>);

/// Horizontal text alignment enum (left, center, right) - default: `Center`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleTextAlign {
    Left,
    Center,
    Right,
}

impl Default for StyleTextAlign {
    fn default() -> Self {
        StyleTextAlign::Left
    }
}

/// Represents a `letter-spacing` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleLetterSpacing(pub PixelValue);

impl Default for StyleLetterSpacing {
    fn default() -> Self {
        Self(PixelValue::const_px(0))
    }
}

/// Represents a `line-height` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleLineHeight(pub PercentageValue);

impl Default for StyleLineHeight {
    fn default() -> Self {
        Self(PercentageValue::const_new(100))
    }
}

/// Represents a `word-spacing` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleWordSpacing(pub PixelValue);

impl Default for StyleWordSpacing {
    fn default() -> Self {
        Self(PixelValue::const_px(0))
    }
}

/// Represents a `tab-width` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTabWidth(pub PercentageValue);

impl Default for StyleTabWidth {
    fn default() -> Self {
        Self(PercentageValue::const_new(100))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleCursor {
    /// `alias`
    Alias,
    /// `all-scroll`
    AllScroll,
    /// `cell`
    Cell,
    /// `col-resize`
    ColResize,
    /// `context-menu`
    ContextMenu,
    /// `copy`
    Copy,
    /// `crosshair`
    Crosshair,
    /// `default` - note: called "arrow" in winit
    Default,
    /// `e-resize`
    EResize,
    /// `ew-resize`
    EwResize,
    /// `grab`
    Grab,
    /// `grabbing`
    Grabbing,
    /// `help`
    Help,
    /// `move`
    Move,
    /// `n-resize`
    NResize,
    /// `ns-resize`
    NsResize,
    /// `nesw-resize`
    NeswResize,
    /// `nwse-resize`
    NwseResize,
    /// `pointer` - note: called "hand" in winit
    Pointer,
    /// `progress`
    Progress,
    /// `row-resize`
    RowResize,
    /// `s-resize`
    SResize,
    /// `se-resize`
    SeResize,
    /// `text`
    Text,
    /// `unset`
    Unset,
    /// `vertical-text`
    VerticalText,
    /// `w-resize`
    WResize,
    /// `wait`
    Wait,
    /// `zoom-in`
    ZoomIn,
    /// `zoom-out`
    ZoomOut,
}

impl Default for StyleCursor {
    fn default() -> StyleCursor {
        StyleCursor::Default
    }
}

/// Represents a `display` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutDisplay {
    None,
    Flex,
    Block,
    InlineBlock,
}

impl Default for LayoutDisplay {
    fn default() -> Self {
        LayoutDisplay::Flex
    }
}

/// Represents a `float` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutFloat {
    Left,
    Right,
}

impl Default for LayoutFloat {
    fn default() -> Self {
        LayoutFloat::Left
    }
}

/// Represents a `flex-direction` attribute - default: `Column`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutBoxSizing {
    ContentBox,
    BorderBox,
}

impl Default for LayoutBoxSizing {
    fn default() -> Self {
        LayoutBoxSizing::ContentBox
    }
}

/// Represents a `width` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutWidth(pub PixelValue);
/// Represents a `min-width` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMinWidth(pub PixelValue);
/// Represents a `max-width` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMaxWidth(pub PixelValue);
/// Represents a `height` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutHeight(pub PixelValue);
/// Represents a `min-height` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMinHeight(pub PixelValue);
/// Represents a `max-height` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMaxHeight(pub PixelValue);

impl Default for LayoutMaxHeight {
    fn default() -> Self {
        Self(PixelValue::px(core::f32::MAX))
    }
}
impl Default for LayoutMaxWidth {
    fn default() -> Self {
        Self(PixelValue::px(core::f32::MAX))
    }
}

/// Represents a `top` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutTop(pub PixelValue);
/// Represents a `left` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutLeft(pub PixelValue);
/// Represents a `right` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutRight(pub PixelValue);
/// Represents a `bottom` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBottom(pub PixelValue);

/// Represents a `position` attribute - default: `Static`
///
/// NOTE: No inline positioning is supported.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutPosition {
    Static,
    Relative,
    Absolute,
    Fixed,
}

impl LayoutPosition {
    pub fn is_positioned(&self) -> bool {
        *self != LayoutPosition::Static
    }
}

impl Default for LayoutPosition {
    fn default() -> Self {
        LayoutPosition::Static
    }
}

/// Represents a `flex-wrap` attribute - default: `Wrap`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutFlexWrap {
    Wrap,
    NoWrap,
}

impl Default for LayoutFlexWrap {
    fn default() -> Self {
        LayoutFlexWrap::Wrap
    }
}

/// Represents a `flex-direction` attribute - default: `Column`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum LayoutFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for LayoutFlexDirection {
    fn default() -> Self {
        LayoutFlexDirection::Column
    }
}

impl LayoutFlexDirection {
    pub fn get_axis(&self) -> LayoutAxis {
        use self::{LayoutAxis::*, LayoutFlexDirection::*};
        match self {
            Row | RowReverse => Horizontal,
            Column | ColumnReverse => Vertical,
        }
    }

    /// Returns true, if this direction is a `column-reverse` or `row-reverse` direction
    pub fn is_reverse(&self) -> bool {
        *self == LayoutFlexDirection::RowReverse || *self == LayoutFlexDirection::ColumnReverse
    }
}

/// Same as the `LayoutFlexDirection`, but without the `-reverse` properties, used in the layout solver,
/// makes decisions based on horizontal / vertical direction easier to write.
/// Use `LayoutFlexDirection::get_axis()` to get the axis for a given `LayoutFlexDirection`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutAxis {
    Horizontal,
    Vertical,
}

/// Represents a `flex-grow` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutFlexGrow(pub FloatValue);

impl Default for LayoutFlexGrow {
    fn default() -> Self {
        LayoutFlexGrow(FloatValue::const_new(0))
    }
}

/// Represents a `flex-shrink` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutFlexShrink(pub FloatValue);

impl Default for LayoutFlexShrink {
    fn default() -> Self {
        LayoutFlexShrink(FloatValue::const_new(0))
    }
}

/// Represents a `justify-content` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutJustifyContent {
    /// Default value. Items are positioned at the beginning of the container
    Start,
    /// Items are positioned at the end of the container
    End,
    /// Items are positioned at the center of the container
    Center,
    /// Items are positioned with space between the lines
    SpaceBetween,
    /// Items are positioned with space before, between, and after the lines
    SpaceAround,
    /// Items are distributed so that the spacing between any two adjacent alignment subjects,
    /// before the first alignment subject, and after the last alignment subject is the same
    SpaceEvenly,
}

impl Default for LayoutJustifyContent {
    fn default() -> Self {
        LayoutJustifyContent::Start
    }
}

/// Represents a `align-items` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutAlignItems {
    /// Items are stretched to fit the container
    Stretch,
    /// Items are positioned at the center of the container
    Center,
    /// Items are positioned at the beginning of the container
    FlexStart,
    /// Items are positioned at the end of the container
    FlexEnd,
}

impl Default for LayoutAlignItems {
    fn default() -> Self {
        LayoutAlignItems::FlexStart
    }
}

/// Represents a `align-content` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutAlignContent {
    /// Default value. Lines stretch to take up the remaining space
    Stretch,
    /// Lines are packed toward the center of the flex container
    Center,
    /// Lines are packed toward the start of the flex container
    Start,
    /// Lines are packed toward the end of the flex container
    End,
    /// Lines are evenly distributed in the flex container
    SpaceBetween,
    /// Lines are evenly distributed in the flex container, with half-size spaces on either end
    SpaceAround,
}

impl Default for LayoutAlignContent {
    fn default() -> Self {
        LayoutAlignContent::Stretch
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirectionCorner {
    Right,
    Left,
    Top,
    Bottom,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl fmt::Display for DirectionCorner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirectionCorner::Right => "right",
                DirectionCorner::Left => "left",
                DirectionCorner::Top => "top",
                DirectionCorner::Bottom => "bottom",
                DirectionCorner::TopRight => "top right",
                DirectionCorner::TopLeft => "top left",
                DirectionCorner::BottomRight => "bottom right",
                DirectionCorner::BottomLeft => "bottom left",
            }
        )
    }
}

impl DirectionCorner {
    pub const fn opposite(&self) -> Self {
        use self::DirectionCorner::*;
        match *self {
            Right => Left,
            Left => Right,
            Top => Bottom,
            Bottom => Top,
            TopRight => BottomLeft,
            BottomLeft => TopRight,
            TopLeft => BottomRight,
            BottomRight => TopLeft,
        }
    }

    pub const fn combine(&self, other: &Self) -> Option<Self> {
        use self::DirectionCorner::*;
        match (*self, *other) {
            (Right, Top) | (Top, Right) => Some(TopRight),
            (Left, Top) | (Top, Left) => Some(TopLeft),
            (Right, Bottom) | (Bottom, Right) => Some(BottomRight),
            (Left, Bottom) | (Bottom, Left) => Some(BottomLeft),
            _ => None,
        }
    }

    pub const fn to_point(&self, rect: &LayoutRect) -> LayoutPoint {
        use self::DirectionCorner::*;
        match *self {
            Right => LayoutPoint {
                x: rect.size.width,
                y: rect.size.height / 2,
            },
            Left => LayoutPoint {
                x: 0,
                y: rect.size.height / 2,
            },
            Top => LayoutPoint {
                x: rect.size.width / 2,
                y: 0,
            },
            Bottom => LayoutPoint {
                x: rect.size.width / 2,
                y: rect.size.height,
            },
            TopRight => LayoutPoint {
                x: rect.size.width,
                y: 0,
            },
            TopLeft => LayoutPoint { x: 0, y: 0 },
            BottomRight => LayoutPoint {
                x: rect.size.width,
                y: rect.size.height,
            },
            BottomLeft => LayoutPoint {
                x: 0,
                y: rect.size.height,
            },
        }
    }
}

/// CSS direction (necessary for gradients). Can either be a fixed angle or
/// a direction ("to right" / "to left", etc.).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Angle(AngleValue),
    FromTo(DirectionCorner, DirectionCorner),
}

impl Default for Direction {
    fn default() -> Self {
        Direction::FromTo(DirectionCorner::Top, DirectionCorner::Bottom)
    }
}

impl Direction {
    /// Calculates the points of the gradient stops for angled linear gradients
    pub fn to_points(&self, rect: &LayoutRect) -> (LayoutPoint, LayoutPoint) {
        match self {
            Direction::Angle(angle_value) => {
                // note: assumes that the LayoutRect has positive sides

                // see: https://hugogiraudel.com/2013/02/04/css-gradients/

                let deg = angle_value.to_degrees(); // FloatValue -> f32

                let deg = -deg; // negate winding direction

                let width_half = rect.size.width as f32 / 2.0;
                let height_half = rect.size.height as f32 / 2.0;

                // hypotenuse_len is the length of the center of the rect to the corners
                let hypotenuse_len = libm::hypotf(width_half, height_half);

                // The corner also serves to determine what quadrant we're in
                // Get the quadrant (corner) the angle is in and get the degree associated
                // with that corner.

                let angle_to_top_left = libm::atanf(height_half / width_half).to_degrees();

                // We need to calculate the angle from the center to the corner!
                let ending_point_degrees = if deg < 90.0 {
                    // top left corner
                    90.0 - angle_to_top_left
                } else if deg < 180.0 {
                    // bottom left corner
                    90.0 + angle_to_top_left
                } else if deg < 270.0 {
                    // bottom right corner
                    270.0 - angle_to_top_left
                } else
                /* deg > 270.0 && deg < 360.0 */
                {
                    // top right corner
                    270.0 + angle_to_top_left
                };

                // assuming deg = 36deg, then degree_diff_to_corner = 9deg
                let degree_diff_to_corner = ending_point_degrees as f32 - deg;

                // Searched_len is the distance between the center of the rect and the
                // ending point of the gradient
                let searched_len = libm::fabsf(libm::cosf(
                    hypotenuse_len * degree_diff_to_corner.to_radians() as f32,
                ));

                // TODO: This searched_len is incorrect...

                // Once we have the length, we can simply rotate the length by the angle,
                // then translate it to the center of the rect
                let dx = libm::sinf(deg.to_radians() as f32) * searched_len;
                let dy = libm::cosf(deg.to_radians() as f32) * searched_len;

                let start_point_location = LayoutPoint {
                    x: libm::roundf(width_half + dx) as isize,
                    y: libm::roundf(height_half + dy) as isize,
                };
                let end_point_location = LayoutPoint {
                    x: libm::roundf(width_half - dx) as isize,
                    y: libm::roundf(height_half - dy) as isize,
                };

                (start_point_location, end_point_location)
            }
            Direction::FromTo(from, to) => (from.to_point(rect), to.to_point(rect)),
        }
    }
}

/// Whether a `gradient` should be repeated or clamped to the edges.
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum ExtendMode {
    Clamp,
    Repeat,
}

impl Default for ExtendMode {
    fn default() -> Self {
        ExtendMode::Clamp
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinearColorStop {
    // this is set to None if there was no offset that could be parsed
    pub offset: Option<PercentageValue>,
    pub color: ColorU,
}

// normalized linear color stop
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedLinearColorStop {
    pub offset: PercentageValue, // 0 to 100% // -- todo: theoretically this should be PixelValue
    pub color: ColorU,
}

impl LinearColorStop {
    pub fn to_normalized(stops: &[LinearColorStop]) -> Vec<NormalizedLinearColorStop> {
        const MIN_STOP_DEGREE: f32 = 0.0;
        const MAX_STOP_DEGREE: f32 = 100.0;

        if stops.is_empty() {
            return Vec::new();
        }

        let self_stops = stops;

        let mut stops = self_stops
            .iter()
            .map(|s| NormalizedLinearColorStop {
                offset: s
                    .offset
                    .as_ref()
                    .copied()
                    .unwrap_or(PercentageValue::new(MIN_STOP_DEGREE)),
                color: s.color,
            })
            .collect::<Vec<_>>();

        let mut stops_to_distribute = 0;
        let mut last_stop = None;
        let stops_len = stops.len();

        for (stop_id, stop) in self_stops.iter().enumerate() {
            if let Some(s) = stop.offset {
                let current_stop_val = s.get();
                if stops_to_distribute != 0 {
                    let last_stop_val = stops[(stop_id - stops_to_distribute)].offset.get();
                    let value_to_add_per_stop = (current_stop_val.max(last_stop_val)
                        - last_stop_val)
                        / (stops_to_distribute - 1) as f32;
                    for (s_id, s) in stops[(stop_id - stops_to_distribute)..stop_id]
                        .iter_mut()
                        .enumerate()
                    {
                        s.offset = PercentageValue::new(
                            last_stop_val + (s_id as f32 * value_to_add_per_stop),
                        );
                    }
                }
                stops_to_distribute = 0;
                last_stop = Some(s);
            } else {
                stops_to_distribute += 1;
            }
        }

        if stops_to_distribute != 0 {
            let last_stop_val = last_stop
                .unwrap_or(PercentageValue::new(MIN_STOP_DEGREE))
                .get();
            let value_to_add_per_stop = (MAX_STOP_DEGREE.max(last_stop_val) - last_stop_val)
                / (stops_to_distribute - 1) as f32;
            for (s_id, s) in stops[(stops_len - stops_to_distribute)..]
                .iter_mut()
                .enumerate()
            {
                s.offset =
                    PercentageValue::new(last_stop_val + (s_id as f32 * value_to_add_per_stop));
            }
        }

        stops
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RadialColorStop {
    // this is set to None if there was no offset that could be parsed
    pub offset: Option<AngleValue>,
    pub color: ColorU,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedRadialColorStop {
    pub angle: AngleValue, // 0 to 360 degrees
    pub color: ColorU,
}

impl RadialColorStop {
    pub fn to_normalized(stops: &[RadialColorStop]) -> Vec<NormalizedRadialColorStop> {
        const MIN_STOP_DEGREE: f32 = 0.0;
        const MAX_STOP_DEGREE: f32 = 360.0;

        if stops.is_empty() {
            return Vec::new();
        }

        let self_stops = stops;

        let mut stops = self_stops
            .iter()
            .map(|s| NormalizedRadialColorStop {
                angle: s
                    .offset
                    .as_ref()
                    .copied()
                    .unwrap_or(AngleValue::deg(MIN_STOP_DEGREE)),
                color: s.color,
            })
            .collect::<Vec<_>>();

        let mut stops_to_distribute = 0;
        let mut last_stop = None;
        let stops_len = stops.len();

        for (stop_id, stop) in self_stops.iter().enumerate() {
            if let Some(s) = stop.offset {
                let current_stop_val = s.to_degrees();
                if stops_to_distribute != 0 {
                    let last_stop_val = stops[(stop_id - stops_to_distribute)].angle.to_degrees();
                    let value_to_add_per_stop = (current_stop_val.max(last_stop_val)
                        - last_stop_val)
                        / (stops_to_distribute - 1) as f32;
                    for (s_id, s) in stops[(stop_id - stops_to_distribute)..stop_id]
                        .iter_mut()
                        .enumerate()
                    {
                        s.angle =
                            AngleValue::deg(last_stop_val + (s_id as f32 * value_to_add_per_stop));
                    }
                }
                stops_to_distribute = 0;
                last_stop = Some(s);
            } else {
                stops_to_distribute += 1;
            }
        }

        if stops_to_distribute != 0 {
            let last_stop_val = last_stop
                .unwrap_or(AngleValue::deg(MIN_STOP_DEGREE))
                .to_degrees();
            let value_to_add_per_stop = (MAX_STOP_DEGREE.max(last_stop_val) - last_stop_val)
                / (stops_to_distribute - 1) as f32;
            for (s_id, s) in stops[(stops_len - stops_to_distribute)..]
                .iter_mut()
                .enumerate()
            {
                s.angle = AngleValue::deg(last_stop_val + (s_id as f32 * value_to_add_per_stop));
            }
        }

        stops
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RadialGradientSize {
    // The gradient's ending shape meets the side of the box closest to its center
    // (for circles) or meets both the vertical and horizontal sides closest to the
    // center (for ellipses).
    ClosestSide,
    // The gradient's ending shape is sized so that it exactly meets the closest
    // corner of the box from its center
    ClosestCorner,
    // Similar to closest-side, except the ending shape is sized to meet the side
    // of the box farthest from its center (or vertical and horizontal sides)
    FarthestSide,
    // The default value, the gradient's ending shape is sized so that it exactly
    // meets the farthest corner of the box from its center
    FarthestCorner,
}

impl Default for RadialGradientSize {
    fn default() -> Self {
        RadialGradientSize::FarthestCorner
    }
}

impl RadialGradientSize {
    pub fn get_size(
        &self,
        parent_rect: LayoutRect,
        _gradient_center: LayoutPosition,
    ) -> LayoutSize {
        // TODO
        parent_rect.size
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shape {
    Ellipse,
    Circle,
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Ellipse
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinearGradient {
    pub direction: Direction,
    pub extend_mode: ExtendMode,
    pub stops: Vec<NormalizedLinearColorStop>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConicGradient {
    pub extend_mode: ExtendMode,               // default = clamp (no-repeat)
    pub center: StyleBackgroundPosition,       // default = center center
    pub angle: AngleValue,                     // default = 0deg
    pub stops: Vec<NormalizedRadialColorStop>, // default = []
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RadialGradient {
    pub shape: Shape,
    pub size: RadialGradientSize,
    pub position: StyleBackgroundPosition,
    pub extend_mode: ExtendMode,
    pub stops: Vec<NormalizedLinearColorStop>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundPositionHorizontal {
    Left,
    Center,
    Right,
    Exact(PixelValue),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackgroundPositionVertical {
    Top,
    Center,
    Bottom,
    Exact(PixelValue),
}

/// Represents a `background-position` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBackgroundPosition {
    pub horizontal: BackgroundPositionHorizontal,
    pub vertical: BackgroundPositionVertical,
}

impl Default for StyleBackgroundPosition {
    fn default() -> Self {
        StyleBackgroundPosition {
            horizontal: BackgroundPositionHorizontal::Left,
            vertical: BackgroundPositionVertical::Top,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleBackgroundContent {
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    ConicGradient(ConicGradient),
    Image(String),
    Color(ColorU),
}

impl Default for StyleBackgroundContent {
    fn default() -> StyleBackgroundContent {
        StyleBackgroundContent::Color(ColorU::TRANSPARENT)
    }
}

impl From<String> for StyleBackgroundContent {
    fn from(id: String) -> Self {
        StyleBackgroundContent::Image(id)
    }
}

/// Represents a `background-size` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleBackgroundSize {
    ExactSize(PixelValue, PixelValue),
    Contain,
    Cover,
}

impl Default for StyleBackgroundSize {
    fn default() -> Self {
        StyleBackgroundSize::Contain
    }
}

/// Represents a `background-repeat` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleBackgroundRepeat {
    NoRepeat,
    Repeat,
    RepeatX,
    RepeatY,
}

impl Default for StyleBackgroundRepeat {
    fn default() -> Self {
        StyleBackgroundRepeat::Repeat
    }
}

/// Represents a `overflow-x` or `overflow-y` property, see
/// [`TextOverflowBehaviour`](./struct.TextOverflowBehaviour.html) - default: `Auto`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutOverflow {
    /// Always shows a scroll bar, overflows on scroll
    Scroll,
    /// Does not show a scroll bar by default, only when text is overflowing
    Auto,
    /// Never shows a scroll bar, simply clips text
    Hidden,
    /// Doesn't show a scroll bar, simply overflows the text
    Visible,
}

impl Default for LayoutOverflow {
    fn default() -> Self {
        LayoutOverflow::Auto
    }
}

impl LayoutOverflow {
    /// Returns whether this overflow value needs to display the scrollbars.
    ///
    /// - `overflow:scroll` always shows the scrollbar
    /// - `overflow:auto` only shows the scrollbar when the content is currently overflowing
    /// - `overflow:hidden` and `overflow:visible` do not show any scrollbars
    pub fn needs_scrollbar(&self, currently_overflowing: bool) -> bool {
        use self::LayoutOverflow::*;
        match self {
            Scroll => true,
            Auto => currently_overflowing,
            Hidden | Visible => false,
        }
    }

    /// Returns whether this is an `overflow:visible` node
    /// (the only overflow type that doesn't clip its children)
    pub fn is_overflow_visible(&self) -> bool {
        *self == LayoutOverflow::Visible
    }

    pub fn is_overflow_hidden(&self) -> bool {
        *self == LayoutOverflow::Hidden
    }
}

/// Represents a `padding-top` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutPaddingTop(pub PixelValue);
/// Represents a `padding-left` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutPaddingLeft(pub PixelValue);
/// Represents a `padding-right` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutPaddingRight(pub PixelValue);
/// Represents a `padding-bottom` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutPaddingBottom(pub PixelValue);

/// Represents a `margin-top` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMarginTop(pub PixelValue);
/// Represents a `margin-left` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMarginLeft(pub PixelValue);
/// Represents a `margin-right` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMarginRight(pub PixelValue);
/// Represents a `margin-bottom` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutMarginBottom(pub PixelValue);

// TODO: Technically, border-radius can take two values for each corner!
/// Represents a `border-top-left-radius` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderTopLeftRadius(pub PixelValue);
/// Represents a `border-bottom-left-radius` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderBottomLeftRadius(pub PixelValue);
/// Represents a `border-top-right-radius` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderTopRightRadius(pub PixelValue);
/// Represents a `border-bottom-right-radius` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderBottomRightRadius(pub PixelValue);

/// Represents a `border-top-color` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderTopColor(pub ColorU);
/// Represents a `border-left-color` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderLeftColor(pub ColorU);
/// Represents a `border-right-color` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderRightColor(pub ColorU);
/// Represents a `border-bottom-color` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderBottomColor(pub ColorU);

/// Style of a `border`: solid, double, dash, ridge, etc.
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[repr(C)]
pub enum BorderStyle {
    None,
    Solid,
    Double,
    Dotted,
    Dashed,
    Hidden,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BorderStyle::*;
        match self {
            None => write!(f, "none"),
            Solid => write!(f, "solid"),
            Double => write!(f, "double"),
            Dotted => write!(f, "dotted"),
            Dashed => write!(f, "dashed"),
            Hidden => write!(f, "hidden"),
            Groove => write!(f, "groove"),
            Ridge => write!(f, "ridge"),
            Inset => write!(f, "inset"),
            Outset => write!(f, "outset"),
        }
    }
}

impl BorderStyle {
    pub fn is_normalized(&self) -> bool {
        match self {
            BorderStyle::None => false,
            _ => true,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::Solid
    }
}

/// Represents a `border-top-style` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderTopStyle ( pub BorderStyle );
/// Represents a `border-left-style` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderLeftStyle ( pub BorderStyle );
/// Represents a `border-right-style` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderRightStyle ( pub BorderStyle );
/// Represents a `border-bottom-style` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBorderBottomStyle ( pub BorderStyle );

/// Represents a `border-top-width` attribute
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBorderTopWidth ( pub PixelValue );
/// Represents a `border-left-width` attribute
#[derive(Debug,Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBorderLeftWidth ( pub PixelValue );
/// Represents a `border-right-width` attribute
#[derive(Debug,Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBorderRightWidth ( pub PixelValue );
/// Represents a `border-bottom-width` attribute
#[derive(Debug,Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutBorderBottomWidth ( pub PixelValue );

/// What direction should a `box-shadow` be clipped in (inset or outset)
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub enum BoxShadowClipMode {
    Outset,
    Inset,
}

impl fmt::Display for BoxShadowClipMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BoxShadowClipMode::*;
        match self {
            Outset => write!(f, "outset"),
            Inset => write!(f, "inset"),
        }
    }
}

// TODO: missing StyleBorderRadius & LayoutRect
// TODO: PixelValue cannot be percent
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBoxShadow {
    pub offset: (PixelValue, PixelValue),
    pub color: ColorU,
    pub blur_radius: PixelValue,
    pub spread_radius: PixelValue,
    pub clip_mode: BoxShadowClipMode,
}

/// Holds info necessary for layouting / styling scrollbars (-webkit-scrollbar)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScrollbarInfo {
    /// Total width (or height for vertical scrollbars) of the scrollbar in pixels
    pub width: LayoutWidth,
    /// Padding of the scrollbar tracker, in pixels. The inner bar is `width - padding` pixels wide.
    pub padding_left: LayoutPaddingLeft,
    /// Padding of the scrollbar (right)
    pub padding_right: LayoutPaddingRight,
    /// Style of the scrollbar background
    /// (`-webkit-scrollbar` / `-webkit-scrollbar-track` / `-webkit-scrollbar-track-piece` combined)
    pub track: StyleBackgroundContent,
    /// Style of the scrollbar thumbs (the "up" / "down" arrows), (`-webkit-scrollbar-thumb`)
    pub thumb: StyleBackgroundContent,
    /// Styles the directional buttons on the scrollbar (`-webkit-scrollbar-button`)
    pub button: StyleBackgroundContent,
    /// If two scrollbars are present, addresses the (usually) bottom corner
    /// of the scrollable element, where two scrollbars might meet (`-webkit-scrollbar-corner`)
    pub corner: StyleBackgroundContent,
    /// Addresses the draggable resizing handle that appears above the
    /// `corner` at the bottom corner of some elements (`-webkit-resizer`)
    pub resizer: StyleBackgroundContent,
}

impl Default for ScrollbarInfo {
    fn default() -> Self {
        ScrollbarInfo {
            width: LayoutWidth(PixelValue::px(17.0)),
            padding_left: LayoutPaddingLeft(PixelValue::px(2.0)),
            padding_right: LayoutPaddingRight(PixelValue::px(2.0)),
            track: StyleBackgroundContent::Color(ColorU { r: 241, g: 241, b: 241, a: 255 }),
            thumb: StyleBackgroundContent::Color(ColorU { r: 193, g: 193, b: 193, a: 255 }),
            button: StyleBackgroundContent::Color(ColorU { r: 163, g: 163, b: 163, a: 255 }),
            corner: StyleBackgroundContent::default(),
            resizer: StyleBackgroundContent::default(),
        }
    }
}

/// Scrollbar style
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScrollbarStyle {
    /// Vertical scrollbar style, if any
    pub horizontal: ScrollbarInfo,
    /// Horizontal scrollbar style, if any
    pub vertical: ScrollbarInfo,
}

/// Represents an `opacity` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleOpacity(pub PercentageValue);

impl Default for StyleOpacity {
    fn default() -> Self {
        StyleOpacity(PercentageValue::const_new(0))
    }
}

/// Represents an `opacity` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleTransform {
    Matrix(StyleTransformMatrix2D),
    Matrix3D(StyleTransformMatrix3D),
    Translate(StyleTransformTranslate2D),
    Translate3D(StyleTransformTranslate3D),
    TranslateX(PixelValue),
    TranslateY(PixelValue),
    TranslateZ(PixelValue),
    Rotate(AngleValue),
    Rotate3D(StyleTransformRotate3D),
    RotateX(AngleValue),
    RotateY(AngleValue),
    RotateZ(AngleValue),
    Scale(StyleTransformScale2D),
    Scale3D(StyleTransformScale3D),
    ScaleX(PercentageValue),
    ScaleY(PercentageValue),
    ScaleZ(PercentageValue),
    Skew(StyleTransformSkew2D),
    SkewX(PercentageValue),
    SkewY(PercentageValue),
    Perspective(PixelValue),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformMatrix2D {
    pub a: PixelValue,
    pub b: PixelValue,
    pub c: PixelValue,
    pub d: PixelValue,
    pub tx: PixelValue,
    pub ty: PixelValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformMatrix3D {
    pub m11: PixelValue,
    pub m12: PixelValue,
    pub m13: PixelValue,
    pub m14: PixelValue,
    pub m21: PixelValue,
    pub m22: PixelValue,
    pub m23: PixelValue,
    pub m24: PixelValue,
    pub m31: PixelValue,
    pub m32: PixelValue,
    pub m33: PixelValue,
    pub m34: PixelValue,
    pub m41: PixelValue,
    pub m42: PixelValue,
    pub m43: PixelValue,
    pub m44: PixelValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformTranslate2D {
    pub x: PixelValue,
    pub y: PixelValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformTranslate3D {
    pub x: PixelValue,
    pub y: PixelValue,
    pub z: PixelValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformRotate3D {
    pub x: PercentageValue,
    pub y: PercentageValue,
    pub z: PercentageValue,
    pub angle: AngleValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformScale2D {
    pub x: PercentageValue,
    pub y: PercentageValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformScale3D {
    pub x: PercentageValue,
    pub y: PercentageValue,
    pub z: PercentageValue,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformSkew2D {
    pub x: PercentageValue,
    pub y: PercentageValue,
}

/// Represents a `transform-origin` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTransformOrigin {
    pub x: PixelValue,
    pub y: PixelValue,
}

impl StyleTransformOrigin {
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(&other.x, t),
            y: self.y.interpolate(&other.y, t),
        }
    }
}

impl Default for StyleTransformOrigin {
    fn default() -> Self {
        StyleTransformOrigin {
            x: PixelValue::const_percent(50),
            y: PixelValue::const_percent(50),
        }
    }
}

/// Represents a `perspective-origin` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StylePerspectiveOrigin {
    pub x: PixelValue,
    pub y: PixelValue,
}

impl StylePerspectiveOrigin {
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(&other.x, t),
            y: self.y.interpolate(&other.y, t),
        }
    }
}

impl Default for StylePerspectiveOrigin {
    fn default() -> Self {
        StylePerspectiveOrigin { x: PixelValue::const_px(0), y: PixelValue::const_px(0) }
    }
}

/// Represents a `backface-visibility` attribute
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleBackfaceVisibility {
    Hidden,
    Visible,
}

impl Default for StyleBackfaceVisibility {
    fn default() -> Self { StyleBackfaceVisibility::Visible }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleMixBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl Default for StyleMixBlendMode {
    fn default() -> StyleMixBlendMode {
        StyleMixBlendMode::Normal
    }
}

impl fmt::Display for StyleMixBlendMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::StyleMixBlendMode::*;
        write!(f, "{}", match self {
            Normal => "normal",
            Multiply => "multiply",
            Screen => "screen",
            Overlay => "overlay",
            Darken => "darken",
            Lighten => "lighten",
            ColorDodge => "color-dodge",
            ColorBurn => "color-burn",
            HardLight => "hard-light",
            SoftLight => "soft-light",
            Difference => "difference",
            Exclusion => "exclusion",
            Hue => "hue",
            Saturation => "saturation",
            Color => "color",
            Luminosity => "luminosity",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleFilter {
    Blend(StyleMixBlendMode),
    Flood(ColorU),
    Blur(StyleBlur),
    Opacity(PercentageValue),
    ColorMatrix(StyleColorMatrix),
    DropShadow(StyleBoxShadow),
    ComponentTransfer,
    Offset(StyleFilterOffset),
    Composite(StyleCompositeFilter),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleBlur {
    pub width: PixelValue,
    pub height: PixelValue,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleColorMatrix {
    pub matrix: [FloatValue; 20],
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleFilterOffset {
    pub x: PixelValue,
    pub y: PixelValue,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StyleCompositeFilter {
    Over,
    In,
    Atop,
    Out,
    Xor,
    Lighter,
    Arithmetic([FloatValue; 4]),
}

