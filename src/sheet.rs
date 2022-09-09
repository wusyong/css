use std::fmt;

use crate::property::Property;

/// Css stylesheet - contains a parsed CSS stylesheet in "rule blocks",
/// i.e. blocks of key-value pairs associated with a selector path.
///
/// One CSS stylesheet can hold more than one sub-stylesheet:
/// For example, when overriding native styles, the `.sort_by_specificy()` function
/// should not mix the two stylesheets during sorting.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Stylesheet {
    /// The style rules making up the document - for example, de-duplicated CSS rules
    pub rules: Vec<RuleBlock>,
}

impl Stylesheet {
    /// Creates a new stylesheet with no style rules.
    pub fn empty() -> Self {
        Default::default()
    }

    /// Sort the style rules by their weight, so that the rules are applied in the correct order.
    /// Should always be called when a new style is loaded from an external source.
    pub fn sort_by_specificity(&mut self) {
        self.rules.sort_by(|a, b| {
            get_specificity(&a.path)
            .cmp(&get_specificity(&b.path))
        });
    }
}

/// Returns specificity of the given css path. Further information can be found on
/// [the w3 website](http://www.w3.org/TR/selectors/#specificity).
fn get_specificity(path: &Path) -> (usize, usize, usize, usize) {
    let id_count = path.selectors.iter().filter(|x|     if let PathSelector::Id(_) = x {     true } else { false }).count();
    let class_count = path.selectors.iter().filter(|x|  if let PathSelector::Class(_) = x {  true } else { false }).count();
    let div_count = path.selectors.iter().filter(|x|    if let PathSelector::Type(_) = x {   true } else { false }).count();
    (id_count, class_count, div_count, path.selectors.len())
}

/// One block of rules that applies a bunch of rules to a "path" in the style, i.e.
/// `div#myid.myclass -> { ("justify-content", "center") }`
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct RuleBlock {
    /// The css path (full selector) of the style ruleset
    pub path: Path,
    /// `"justify-content: center"` =>
    /// `CssDeclaration::Static(CssProperty::JustifyContent(LayoutJustifyContent::Center))`
    pub declarations: Vec<Declaration>,
}

/// Represents a full CSS path (i.e. the "div#id.class" selector belonging to
///  a CSS "content group" (the following key-value block)).
///
/// ```no_run,ignore
/// "#div > .my_class:focus" ==
/// [
///   PathSelector::Type(NodeTypeTag::Div),
///   PathSelector::PseudoSelector(PathPseudoSelector::LimitChildren),
///   PathSelector::Class("my_class"),
///   PathSelector::PseudoSelector(PathPseudoSelector::Focus),
/// ]
#[derive(Debug, Clone, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    pub selectors: Vec<PathSelector>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for selector in self.selectors.iter() {
            write!(f, "{}", selector)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathSelector {
    /// Represents the `*` selector
    Global,
    /// `div`, `p`, etc.
    Type(NodeTypeTag),
    /// `.something`
    Class(String),
    /// `#something`
    Id(String),
    /// `:something`
    PseudoSelector(PathPseudoSelector),
    /// Represents the `>` selector
    DirectChildren,
    /// Represents the ` ` selector
    Children,
}

impl Default for PathSelector {
    fn default() -> Self {
        PathSelector::Global
    }
}

impl fmt::Display for PathSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PathSelector::*;
        match &self {
            Global => write!(f, "*"),
            Type(n) => write!(f, "{}", n),
            Class(c) => write!(f, ".{}", c),
            Id(i) => write!(f, "#{}", i),
            PseudoSelector(p) => write!(f, ":{}", p),
            DirectChildren => write!(f, ">"),
            Children => write!(f, " "),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathPseudoSelector {
    /// `:first`
    First,
    /// `:last`
    Last,
    /// `:nth-child`
    NthChild(NthChildSelector),
    /// `:hover` - mouse is over element
    Hover,
    /// `:active` - mouse is pressed and over element
    Active,
    /// `:focus` - element has received focus
    Focus,
}

impl fmt::Display for PathPseudoSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PathPseudoSelector::*;
        match &self {
            First => write!(f, "first"),
            Last => write!(f, "last"),
            NthChild(u) => write!(f, "nth-child({})", u),
            Hover => write!(f, "hover"),
            Active => write!(f, "active"),
            Focus => write!(f, "focus"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NthChildSelector {
    Number(u32),
    Even,
    Odd,
    Pattern(NthChildPattern),
}

impl fmt::Display for NthChildSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NthChildSelector::*;
        match &self {
            Number(u) => write!(f, "{}", u),
            Even => write!(f, "even"),
            Odd => write!(f, "odd"),
            Pattern(p) => write!(f, "{}n + {}", p.repeat, p.offset),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NthChildPattern {
    pub repeat: u32,
    pub offset: u32,
}

/// Signifies the type (i.e. the discriminant value) of a DOM node
/// without carrying any of its associated data
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeTypeTag {
    Body,
    Div,
    Br,
    P,
    Img,
    IFrame,
}

/// Parses the node type from a CSS string such as `"div"` => `NodeTypeTag::Div`
impl NodeTypeTag {
    pub fn from_str(css_key: &str) -> Option<Self> {
        match css_key {
            "body" => Some(NodeTypeTag::Body),
            "div" => Some(NodeTypeTag::Div),
            "p" => Some(NodeTypeTag::P),
            "img" => Some(NodeTypeTag::Img),
            _ => None,
        }
    }
}

impl fmt::Display for NodeTypeTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeTypeTag::Body => write!(f, "body"),
            NodeTypeTag::Div => write!(f, "div"),
            NodeTypeTag::Br => write!(f, "br"),
            NodeTypeTag::P => write!(f, "p"),
            NodeTypeTag::Img => write!(f, "img"),
            NodeTypeTag::IFrame => write!(f, "iframe"),
        }
    }
}

/// Contains one parsed `key: value` pair, static or dynamic
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C, u8)]
pub enum Declaration {
    /// Static key-value pair, such as `width: 500px`
    Static(Property),
    /// Dynamic key-value pair with default value, such as `width: [[ my_id | 500px ]]`
    Dynamic(DynamicProperty),
}

impl Declaration {

    pub const fn new_static(prop: Property) -> Self {
        Declaration::Static(prop)
    }

    pub const fn new_dynamic(prop: DynamicProperty) -> Self {
        Declaration::Dynamic(prop)
    }

    /// Determines if the property will be inherited (applied to the children)
    /// during the recursive application of the style on the DOM tree
    pub fn is_inheritable(&self) -> bool {
        use self::Declaration::*;
        match self {
            Static(s) => s.is_inheritable(),
            Dynamic(d) => d.is_inheritable(),
        }
    }

    /// Returns whether this rule affects only styling properties or layout
    /// properties (that could trigger a re-layout)
    pub fn can_relayout(&self) -> bool {
        use self::Declaration::*;
        match self {
            Static(s) => s.can_relayout(),
            Dynamic(d) => d.can_relayout(),
        }
    }

    pub fn to_str(&self) -> String {
        use self::Declaration::*;
        match self {
            Static(s) => format!("{:?}", s),
            Dynamic(d) => format!("var(--{}, {:?})", d.dynamic_id, d.default_value),
        }
    }
}

/// A `DynamicProperty` is a type of css property that can be changed on possibly
/// every frame by the Rust code - for example to implement an `On::Hover` behaviour.
///
/// The syntax for such a property looks like this:
///
/// ```no_run,ignore
/// #my_div {
///    padding: var(--my_dynamic_property_id, 400px);
/// }
/// ```
///
/// Azul will register a dynamic property with the key "my_dynamic_property_id"
/// and the default value of 400px. If the property gets overridden during one frame,
/// the overridden property takes precedence.
///
/// At runtime the style is immutable (which is a performance optimization - if we
/// can assume that the property never changes at runtime), we can do some optimizations on it.
/// Dynamic style properties can also be used for animations and conditional styles
/// (i.e. `hover`, `focus`, etc.), thereby leading to cleaner code, since all of these
/// special cases now use one single API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DynamicProperty {
    /// The stringified ID of this property, i.e. the `"my_id"` in `width: var(--my_id, 500px)`.
    pub dynamic_id: String,
    /// Default values for this properties - one single value can control multiple properties!
    pub default_value: Property,
}

impl DynamicProperty {
    pub fn is_inheritable(&self) -> bool {
        // Dynamic style properties should not be inheritable,
        // since that could lead to bugs - you set a property in Rust, suddenly
        // the wrong UI component starts to react because it was inherited.
        false
    }

    pub fn can_relayout(&self) -> bool {
        self.default_value.can_relayout()
    }
}


#[test]
fn test_specificity() {
    use self::PathSelector::*;
    use std::string::ToString;
    assert_eq!(get_specificity(&Path { selectors: vec![Id("hello".to_string())] }), (1, 0, 0, 1));
    assert_eq!(get_specificity(&Path { selectors: vec![Class("hello".to_string())] }), (0, 1, 0, 1));
    assert_eq!(get_specificity(&Path { selectors: vec![Type(NodeTypeTag::Div)] }), (0, 0, 1, 1));
    assert_eq!(get_specificity(&Path { selectors: vec![Id("hello".to_string()), Type(NodeTypeTag::Div)] }), (1, 0, 1, 2));
}

// Assert that order of the style items is correct
// (in order of CSS path specificity, lowest-to-highest)
#[test]
fn test_specificity_sort() {
    use self::PathSelector::*;
    use self::NodeTypeTag::*;
    use std::string::ToString;

    let mut input_style = Stylesheet {
        rules: vec![
            // Rules are sorted from lowest-specificity to highest specificity
            RuleBlock { path: Path { selectors: vec![Global] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Type(Div), Class("my_class".to_string()), Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Type(Div), Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Type(Div), Class("my_class".to_string()), Class("specific".to_string()), Id("my_id".to_string())] }, declarations: Vec::new() },
        ],
    };
    input_style.sort_by_specificity();

    let expected_style = Stylesheet {
        rules: vec![
            // Rules are sorted from lowest-specificity to highest specificity
            RuleBlock { path: Path { selectors: vec![Global] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Type(Div), Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Global, Type(Div), Class("my_class".to_string()), Id("my_id".to_string())] }, declarations: Vec::new() },
            RuleBlock { path: Path { selectors: vec![Type(Div), Class("my_class".to_string()), Class("specific".to_string()), Id("my_id".to_string())] }, declarations: Vec::new() },
        ],
    };

    assert_eq!(input_style, expected_style);
}

