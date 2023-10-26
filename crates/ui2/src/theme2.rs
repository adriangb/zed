use gpui2::{hsla, rgb, Hsla};
use strum::EnumIter;

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl PlayerColor {
    pub fn new(color: ColorScaleName) -> Self {
        Self {
            cursor: color.value(9),
            selection: color.value(3),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxColor {
    pub comment: Hsla,
    pub string: Hsla,
    pub function: Hsla,
    pub keyword: Hsla,
}

impl SyntaxColor {
    pub fn new() -> Self {
        let color = ThemeColor::new();

        Self {
            comment: color.syntax.comment,
            string: color.syntax.string,
            function: color.syntax.function,
            keyword: color.syntax.keyword,
        }
    }
}

/// ThemeColor is the primary interface for coloring elements in the UI.
///
/// It is a mapping layer between semantic theme colors and colors from the reference library.
///
/// While we are between zed and zed2 we use this to map semantic colors to the old theme.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColor {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    /// The background color of an elevated surface, like a modal, tooltip or toast.
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    /// Window background color of the base app
    pub background: Hsla,
    /// Default background for elements like filled buttons,
    /// text fields, checkboxes, radio buttons, etc.
    /// - TODO: Map to step 3.
    pub filled_element: Hsla,
    /// The background color of a hovered element, like a button being hovered
    /// with a mouse, or hovered on a touch screen.
    /// - TODO: Map to step 4.
    pub filled_element_hover: Hsla,
    /// The background color of an active element, like a button being pressed,
    /// or tapped on a touch screen.
    /// - TODO: Map to step 5.
    pub filled_element_active: Hsla,
    /// The background color of a selected element, like a selected tab,
    /// a button toggled on, or a checkbox that is checked.
    pub filled_element_selected: Hsla,
    pub filled_element_disabled: Hsla,
    pub ghost_element: Hsla,
    /// The background color of a hovered element with no default background,
    /// like a ghost-style button or an interactable list item.
    /// - TODO: Map to step 3.
    pub ghost_element_hover: Hsla,
    /// - TODO: Map to step 4.
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,

    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,

    pub syntax: SyntaxColor,

    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    /// The background of the editor
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub terminal: Hsla,
    pub image_fallback_background: Hsla,

    pub created: Hsla,
    pub modified: Hsla,
    pub deleted: Hsla,
    pub conflict: Hsla,
    pub hidden: Hsla,
    pub ignored: Hsla,
    pub renamed: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub success: Hsla,

    pub git_created: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub git_conflict: Hsla,
    pub git_ignored: Hsla,
    pub git_renamed: Hsla,

    pub player: [PlayerColor; 8],
}

impl ThemeColor {
    pub fn new() -> Self {
        let scales = default_colors();
        let transparent = hsla(0.0, 0.0, 0.0, 0.0);

        let neutral = ColorScaleName::Slate;
        let focus_color = ColorScaleName::Indigo;
        let accent_color = ColorScaleName::Blue;

        let background = neutral.clone().value(3);
        let surface = neutral.clone().value(2);
        let editor = neutral.clone().value(1);

        Self {
            transparent,
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
            border: neutral.clone().value(5),
            border_variant: neutral.clone().value(4),
            border_focused: focus_color.clone().value(6),
            border_transparent: transparent,
            elevated_surface: neutral.clone().value(1),
            surface: surface.clone(),
            background: background.clone(),
            filled_element: neutral.clone().value(3),
            filled_element_hover: neutral.clone().value(4),
            filled_element_active: neutral.clone().value(5),
            filled_element_selected: neutral.clone().value(5),
            filled_element_disabled: transparent,
            ghost_element: transparent,
            ghost_element_hover: neutral.clone().value(4),
            ghost_element_active: neutral.clone().value(5),
            ghost_element_selected: neutral.clone().value(5),
            ghost_element_disabled: transparent,
            text: neutral.clone().value(12),
            text_muted: neutral.clone().value(11),
            text_placeholder: neutral.clone().value(11),
            text_disabled: neutral.clone().value(10),
            text_accent: accent_color.clone().value(11),
            icon: neutral.clone().value(12),
            icon_muted: neutral.clone().value(11),
            icon_placeholder: neutral.clone().value(11),
            icon_disabled: neutral.clone().value(10),
            icon_accent: accent_color.clone().value(11),
            syntax: SyntaxColor {
                comment: neutral.clone().value(11),
                keyword: ColorScaleName::Orange.clone().value(12),
                string: ColorScaleName::Lime.clone().value(12),
                function: ColorScaleName::Amber.clone().value(1),
            },
            status_bar: background.clone(),
            title_bar: background.clone(),
            toolbar: editor.clone(),
            tab_bar: surface.clone(),
            editor: editor.clone(),
            editor_subheader: surface.clone(),
            terminal: editor.clone(),
            editor_active_line: neutral.clone().value(3),

            image_fallback_background: neutral.clone().value(1),

            created: ColorScaleName::Green.clone().value(11),
            modified: ColorScaleName::Amber.clone().value(11),
            deleted: ColorScaleName::Red.clone().value(11),
            conflict: ColorScaleName::Red.clone().value(11),
            hidden: neutral.clone().value(11),
            ignored: neutral.clone().value(11),
            renamed: ColorScaleName::Iris.clone().value(11),
            error: ColorScaleName::Red.clone().value(11),
            warning: ColorScaleName::Amber.clone().value(11),
            info: ColorScaleName::Blue.clone().value(11),
            success: ColorScaleName::Green.clone().value(11),

            git_created: ColorScaleName::Green.clone().value(11),
            git_modified: ColorScaleName::Amber.clone().value(11),
            git_deleted: ColorScaleName::Red.clone().value(11),
            git_conflict: ColorScaleName::Red.clone().value(11),
            git_ignored: neutral.clone().value(11),
            git_renamed: ColorScaleName::Iris.clone().value(11),

            player: [
                PlayerColor::new(ColorScaleName::Blue),
                PlayerColor::new(ColorScaleName::Green),
                PlayerColor::new(ColorScaleName::Red),
                PlayerColor::new(ColorScaleName::Yellow),
                PlayerColor::new(ColorScaleName::Purple),
                PlayerColor::new(ColorScaleName::Cyan),
                PlayerColor::new(ColorScaleName::Orange),
                PlayerColor::new(ColorScaleName::Pink),
            ],
        }
    }
}

pub fn to_gpui_hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    hsla(h / 360.0, s / 100.0, l / 100.0, a)
}

pub fn to_gpui_hue(h: f32) -> f32 {
    h / 360.0
}

pub fn from_gpui_hue(h: f32) -> f32 {
    h * 360.0
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum ColorScaleName {
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
    Black,
    White,
    Custom(String),
}

impl ColorScaleName {
    pub fn label(&self) -> String {
        let label = match *self {
            ColorScaleName::Gray => "Gray",
            ColorScaleName::Mauve => "Mauve",
            ColorScaleName::Slate => "Slate",
            ColorScaleName::Sage => "Sage",
            ColorScaleName::Olive => "Olive",
            ColorScaleName::Sand => "Sand",
            ColorScaleName::Gold => "Gold",
            ColorScaleName::Bronze => "Bronze",
            ColorScaleName::Brown => "Brown",
            ColorScaleName::Yellow => "Yellow",
            ColorScaleName::Amber => "Amber",
            ColorScaleName::Orange => "Orange",
            ColorScaleName::Tomato => "Tomato",
            ColorScaleName::Red => "Red",
            ColorScaleName::Ruby => "Ruby",
            ColorScaleName::Crimson => "Crimson",
            ColorScaleName::Pink => "Pink",
            ColorScaleName::Plum => "Plum",
            ColorScaleName::Purple => "Purple",
            ColorScaleName::Violet => "Violet",
            ColorScaleName::Iris => "Iris",
            ColorScaleName::Indigo => "Indigo",
            ColorScaleName::Blue => "Blue",
            ColorScaleName::Cyan => "Cyan",
            ColorScaleName::Teal => "Teal",
            ColorScaleName::Jade => "Jade",
            ColorScaleName::Green => "Green",
            ColorScaleName::Grass => "Grass",
            ColorScaleName::Lime => "Lime",
            ColorScaleName::Mint => "Mint",
            ColorScaleName::Sky => "Sky",
            ColorScaleName::Black => "Black",
            ColorScaleName::White => "White",
            ColorScaleName::Custom(ref name) => name,
        };
        label.into()
    }
}

#[derive(Debug, Clone)]
pub struct ColorScaleStep {
    pub name: String,
    pub value: Hsla,
    pub scale: ColorScaleName,
}

impl Default for ColorScaleStep {
    fn default() -> Self {
        Self {
            name: "Untitled Color".into(),
            value: hsla(0., 0., 0., 0.),
            scale: ColorScaleName::Custom("Untitled Color Scale".into()),
        }
    }
}

impl ColorScaleStep {
    pub fn new<S: Into<String>>(name: S, hsla: Hsla, scale: ColorScaleName) -> Self {
        Self {
            name: name.into(),
            value: hsla,
            scale,
        }
    }

    pub fn new_name_from_index(index: usize, hsla: Hsla, scale: ColorScaleName) -> String {
        format!("{} {}", scale.label(), index)
    }

    pub fn hsla(&self) -> Hsla {
        self.value
    }

    pub fn scale(&self) -> ColorScaleName {
        self.scale.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ColorScale {
    pub name: ColorScaleName,
    /// Use ColorScale::value() to access the steps.
    ///
    /// Using this directly will result in incorrect values.
    pub steps: [ColorScaleStep; 12],
}

impl ColorScale {
    /// Creating a new ColorScale can be done in a number of ways.
    ///
    /// - `ColorScale::new()` will create a ColorScale with the given name and steps.
    /// - `ColorScaleName.into()` will create a ColorScale from the default set.
    /// - `ColorScale::from_hex()` will create a ColorScale from a hex value.
    /// - `ColorScale::from_hex_arr()` will create a ColorScale from an array of hex values.
    /// - `ColorScale::from_hsla()` will create a ColorScale from an array of Hsla values.
    pub fn new(name: ColorScaleName, hues: [Hsla; 12]) -> Self {
        let steps = hues
            .iter()
            .enumerate()
            .map(|(i, &hue)| {
                let step = i + 1;
                let color_name = format!("{:?} {}", name, step);
                let scale = name.clone();
                ColorScaleStep::new(color_name, hue, scale)
            })
            .collect::<Vec<ColorScaleStep>>();

        Self {
            name,
            steps: match steps.try_into() {
                Ok(array) => array,
                Err(vec) => panic!("Expected a Vec of length 12, but it was {}", vec.len()),
            },
        }
    }

    pub fn from_steps(input_name: Option<String>, input_steps: [Hsla; 12]) -> CustomScale {
        let default = NewCustomScale::default();
        let name = input_name.unwrap_or(default.name.unwrap());

        CustomScale {
            name: name.clone(),
            steps: Self::step_arr_to_colors(input_steps, name.clone()),
        }
    }

    pub fn from_hsla(input_name: Option<String>, input_hsla: Hsla) -> CustomScale {
        let default = NewCustomScale::default();
        let name = input_name.unwrap_or(default.name.unwrap());

        let steps_arr = Self::steps_from_hsla(default.steps, name.clone(), input_hsla);

        CustomScale {
            name: name.clone(),
            steps: Self::step_arr_to_colors(steps_arr, name.clone()),
        }
    }

    pub fn from_hex(input_name: impl Into<String>, hex: &str) -> CustomScale {
        // TODO: gpui probably has better utilities for doing this conversion already.
        let hsla = ColorScaleName::hex_to_hsla(hex).expect("Bad hex value input");
        NewCustomScale::from_hsla(Some(input_name.into()), hsla)
    }

    /// TODO: This can probably be From<[&str; 12]> for ColorScale
    pub fn from_hex_arr(name: String, hex: [&str; 12]) -> Self {
        let mut hslas = [hsla(0., 0., 0., 0.); 12];
        for (i, hex) in hex.iter().enumerate() {
            hslas[i] = Self::hex_to_hsla(hex).unwrap_or(hsla(0., 0., 0., 0.));
        }
        Self {
            name,
            steps: hslas
        }
    }

    /// Converts an 8-value hex array into a custom scale.
    ///
    /// This is a common format for defining neutral scales
    /// used in base16 themes and other theme formats
    pub fn from_8_hex(name: impl Into<String>, values: [&str; 8]) -> CustomScale {
        // TODO: Actually make this work
        // For the moment we will just repeat values

        let extended_hex_colors = [
            values[0], values[0], // Duplicate the first color
            values[1], values[2],
            values[3], values[3], // Duplicate the fourth color
            values[4], values[5],
            values[6], values[6], // Duplicate the seventh color
            values[7], values[7], // Duplicate the eighth color
        ];

        let colors = ColorScaleName::hex_arr_to_hsla(extended_hex_colors);

        Self::from_steps(Some(name.into()), colors)
    }

    pub fn from_default(scale_name: ColorScaleName) -> Self {
        let scale_steps = match scale_name {
            ColorScaleName::Gray => Self::from_hex_arr(
                ColorScaleName::Gray.label(),
                [
                "#111111", "#191919", "#222222", "#2a2a2a", "#313131", "#3a3a3a", "#484848",
                "#606060", "#6e6e6e", "#7b7b7b", "#b4b4b4", "#eeeeee",
            ]),
            ColorScaleName::Mauve => Self::from_hex_arr(
                ColorScaleName::Mauve.label(),
                [
                "#121113", "#1a191b", "#232225", "#2b292d", "#323035", "#3c393f", "#49474e",
                "#625f69", "#6f6d78", "#7c7a85", "#b5b2bc", "#eeeef0",
            ]),
            ColorScaleName::Slate => Self::from_hex_arr([
                "#111113", "#18191b", "#212225", "#272a2d", "#2e3135", "#363a3f", "#43484e",
                "#5a6169", "#696e77", "#777b84", "#b0b4ba", "#edeef0",
            ]),
            ColorScaleName::Sage => Self::from_hex_arr([
                "#101211", "#171918", "#202221", "#272a29", "#2e3130", "#373b39", "#444947",
                "#5b625f", "#63706b", "#717d79", "#adb5b2", "#eceeed",
            ]),
            ColorScaleName::Olive => Self::from_hex_arr([
                "#111210", "#181917", "#212220", "#282a27", "#2f312e", "#383a36", "#454843",
                "#5c625b", "#687066", "#767d74", "#afb5ad", "#eceeec",
            ]),
            ColorScaleName::Sand => Self::from_hex_arr([
                "#111110", "#191918", "#222221", "#2a2a28", "#31312e", "#3b3a37", "#494844",
                "#62605b", "#6f6d66", "#7c7b74", "#b5b3ad", "#eeeeec",
            ]),
            ColorScaleName::Tomato => Self::from_hex_arr([
                "#181111", "#1f1513", "#391714", "#4e1511", "#5e1c16", "#6e2920", "#853a2d",
                "#ac4d39", "#e54d2e", "#ec6142", "#ff977d", "#fbd3cb",
            ]),
            ColorScaleName::Red => Self::from_hex_arr([
                "#191111", "#201314", "#3b1219", "#500f1c", "#611623", "#72232d", "#8c333a",
                "#b54548", "#e5484d", "#ec5d5e", "#ff9592", "#ffd1d9",
            ]),
            ColorScaleName::Ruby => Self::from_hex_arr([
                "#191113", "#1e1517", "#3a141e", "#4e1325", "#5e1a2e", "#6f2539", "#883447",
                "#b3445a", "#e54666", "#ec5a72", "#ff949d", "#fed2e1",
            ]),
            ColorScaleName::Crimson => Self::from_hex_arr([
                "#191114", "#201318", "#381525", "#4d122f", "#5c1839", "#6d2545", "#873356",
                "#b0436e", "#e93d82", "#ee518a", "#ff92ad", "#fdd3e8",
            ]),
            ColorScaleName::Pink => Self::from_hex_arr([
                "#191117", "#21121d", "#37172f", "#4b143d", "#591c47", "#692955", "#833869",
                "#a84885", "#d6409f", "#de51a8", "#ff8dcc", "#fdd1ea",
            ]),
            ColorScaleName::Plum => Self::from_hex_arr([
                "#181118", "#201320", "#351a35", "#451d47", "#512454", "#5e3061", "#734079",
                "#92549c", "#ab4aba", "#b658c4", "#e796f3", "#f4d4f4",
            ]),
            ColorScaleName::Purple => Self::from_hex_arr([
                "#18111b", "#1e1523", "#301c3b", "#3d224e", "#48295c", "#54346b", "#664282",
                "#8457aa", "#8e4ec6", "#9a5cd0", "#d19dff", "#ecd9fa",
            ]),
            ColorScaleName::Violet => Self::from_hex_arr([
                "#14121f", "#1b1525", "#291f43", "#33255b", "#3c2e69", "#473876", "#56468b",
                "#6958ad", "#6e56cf", "#7d66d9", "#baa7ff", "#e2ddfe",
            ]),
            ColorScaleName::Iris => Self::from_hex_arr([
                "#13131e", "#171625", "#202248", "#262a65", "#303374", "#3d3e82", "#4a4a95",
                "#5958b1", "#5b5bd6", "#6e6ade", "#b1a9ff", "#e0dffe",
            ]),
            ColorScaleName::Indigo => Self::from_hex_arr([
                "#11131f", "#141726", "#182449", "#1d2e62", "#253974", "#304384", "#3a4f97",
                "#435db1", "#3e63dd", "#5472e4", "#9eb1ff", "#d6e1ff",
            ]),
            ColorScaleName::Blue => Self::from_hex_arr([
                "#0d1520", "#111927", "#0d2847", "#003362", "#004074", "#104d87", "#205d9e",
                "#2870bd", "#0090ff", "#3b9eff", "#70b8ff", "#c2e6ff",
            ]),
            ColorScaleName::Cyan => Self::from_hex_arr([
                "#0b161a", "#101b20", "#082c36", "#003848", "#004558", "#045468", "#12677e",
                "#11809c", "#00a2c7", "#23afd0", "#4ccce6", "#b6ecf7",
            ]),
            ColorScaleName::Teal => Self::from_hex_arr([
                "#0d1514", "#111c1b", "#0d2d2a", "#023b37", "#084843", "#145750", "#1c6961",
                "#207e73", "#12a594", "#0eb39e", "#0bd8b6", "#adf0dd",
            ]),
            ColorScaleName::Jade => Self::from_hex_arr([
                "#0d1512", "#121c18", "#0f2e22", "#0b3b2c", "#114837", "#1b5745", "#246854",
                "#2a7e68", "#29a383", "#27b08b", "#1fd8a4", "#adf0d4",
            ]),
            ColorScaleName::Green => Self::from_hex_arr([
                "#0e1512", "#121b17", "#132d21", "#113b29", "#174933", "#20573e", "#28684a",
                "#2f7c57", "#30a46c", "#33b074", "#3dd68c", "#b1f1cb",
            ]),
            ColorScaleName::Grass => Self::from_hex_arr([
                "#0e1511", "#141a15", "#1b2a1e", "#1d3a24", "#25482d", "#2d5736", "#366740",
                "#3e7949", "#46a758", "#53b365", "#71d083", "#c2f0c2",
            ]),
            ColorScaleName::Brown => Self::from_hex_arr([
                "#12110f", "#1c1816", "#28211d", "#322922", "#3e3128", "#4d3c2f", "#614a39",
                "#7c5f46", "#ad7f58", "#b88c67", "#dbb594", "#f2e1ca",
            ]),
            ColorScaleName::Bronze => Self::from_hex_arr([
                "#141110", "#1c1917", "#262220", "#302a27", "#3b3330", "#493e3a", "#5a4c47",
                "#6f5f58", "#a18072", "#ae8c7e", "#d4b3a5", "#ede0d9",
            ]),
            ColorScaleName::Gold => Self::from_hex_arr([
                "#121211", "#1b1a17", "#24231f", "#2d2b26", "#38352e", "#444039", "#544f46",
                "#696256", "#978365", "#a39073", "#cbb99f", "#e8e2d9",
            ]),
            ColorScaleName::Sky => Self::from_hex_arr([
                "#0d141f", "#111a27", "#112840", "#113555", "#154467", "#1b537b", "#1f6692",
                "#197cae", "#7ce2fe", "#a8eeff", "#75c7f0", "#c2f3ff",
            ]),
            ColorScaleName::Mint => Self::from_hex_arr([
                "#0e1515", "#0f1b1b", "#092c2b", "#003a38", "#004744", "#105650", "#1e685f",
                "#277f70", "#86ead4", "#a8f5e5", "#58d5ba", "#c4f5e1",
            ]),
            ColorScaleName::Lime => Self::from_hex_arr([
                "#11130c", "#151a10", "#1f2917", "#28211d", "#334423", "#3d522a", "#496231",
                "#577538", "#bdee63", "#d4ff70", "#bde56c", "#e3f7ba",
            ]),
            ColorScaleName::Yellow => Self::from_hex_arr([
                "#14120b", "#1b180f", "#2d2305", "#362b00", "#433500", "#524202", "#665417",
                "#836a21", "#ffe629", "#ffff57", "#f5e147", "#f6eeb4",
            ]),
            ColorScaleName::Amber => Self::from_hex_arr([
                "#16120c", "#1d180f", "#302008", "#3f2700", "#4d3000", "#5c3d05", "#714f19",
                "#8f6424", "#ffc53d", "#ffd60a", "#ffca16", "#ffe7b3",
            ]),
            ColorScaleName::Orange => Self::from_hex_arr([
                "#17120e", "#1e160f", "#331e0b", "#462100", "#562800", "#66350c", "#7e451d",
                "#a35829", "#f76b15", "#ff801f", "#ffa057", "#ffe0c2",
            ]),
            _ => Self::from_hex_arr([
                "#000000", "#000000", "#000000", "#000000", "#000000", "#000000", "#000000",
                "#000000", "#000000", "#000000", "#000000", "#000000",
            ]),
        }
    }

    /// Returns a new instance of `Scale` with the order of the steps reversed.
    ///
    /// This is useful in scales like `neutral` where the dark and light variant
    /// of a theme often uses a reversed variant of the other's Scale.
    pub fn reversed(&self) -> Self {
        let mut reversed_steps = self.steps;
        reversed_steps.reverse();

                Self {
            name: self.name.clone(),
            steps: reversed_steps,
        }
    }

    fn hex_to_u32(hex: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(&hex.trim_start_matches('#'), 16)
    }

    pub fn hex_to_hsla(hex: &str) -> Result<Hsla, std::num::ParseIntError> {
        Self::hex_to_u32(hex).map(|color| rgb::<Hsla>(color))
    }





        /// Returns the One-based value of the given step in the scale.
        ///
        /// We usally reference steps as 1-12 instead of 0-11, so we
        /// automatically subtract 1 from the index.
        pub fn value(self, index: usize) -> Hsla {
            let color_scale = ColorScaleName::from_default(self);
            color_scale.steps[index - 1].value
        }

    pub fn step_hsla(&self, step: usize) -> Hsla {
        if step < 1 || step > 12 {
            panic!("Step must be between 1 and 12, but it was {}", step);
        }

        let color_scale = self.name.clone();

        ColorScaleName::value(color_scale, step)
    }

    pub fn steps_arr_to_vec(steps: [ColorScaleStep; 12]) -> Vec<ColorScaleStep> {
        steps.iter().cloned().collect::<Vec<ColorScaleStep>>()
    }

    pub fn hsla_vec_to_arr(steps: Vec<Hsla>) -> [Hsla; 12] {
        if steps.len() != 12 {
            panic!("Expected a Vec of length 12, but it was {}", steps.len());
        }

        let mut arr = [hsla(0.0, 0.0, 0.0, 0.0); 12];
        for (i, step) in steps.iter().enumerate() {
            arr[i] = *step;
        }
        arr
    }

    pub fn closest_scale_index(scale_colors: [Hsla; 12], hsla_color: Hsla) -> usize {
        let mut best_match = 0;
        let mut smallest_lum_diff = f32::MAX;

        for (index, scale_color) in scale_colors.iter().enumerate() {
            let lum_diff = (hsla_color.l - scale_color.l).abs();

            if lum_diff < smallest_lum_diff {
                smallest_lum_diff = lum_diff;
                best_match = index;
            }
        }
        best_match
    }

    pub fn step_arr_to_colors(steps: [Hsla; 12], name: String) -> [ColorScaleStep; 12] {
        let mut colors_vec = Vec::new();

        for (i, step) in steps.iter().enumerate() {
            let step_name = ColorScaleStep::new_name_from_index(i, *step, ColorScaleName::Custom(name.clone()));
            let color = ColorScaleStep::new(step_name, *step, ColorScaleName::Custom(name.clone()));
            colors_vec.push(color);
        }

        let colors: [ColorScaleStep; 12] = match colors_vec.try_into() {
            Ok(array) => array,
            Err(vec) => panic!("Unexpected vector length {}", vec.len()),
        };

        colors
    }

    fn steps_from_hsla(scales: Option<Vec<Hsla>>, name: String, input_hsla: Hsla) -> [Hsla; 12] {
        let original_hues = scales.expect("Scale doesn't have any hues");
        let original_steps = ColorScale::hsla_vec_to_arr(original_hues);

        // Find the index of the closest matching color from the original scale
        let index = ColorScale::closest_scale_index(original_steps, input_hsla);

        let input_step_name =
            ColorScaleStep::new_name_from_index(index, input_hsla, ColorScaleName::Custom(name.clone()));
        let input_step = ColorScaleStep::new(
            input_step_name,
            input_hsla,
            ColorScaleName::Custom(name.clone()),
        );

        // Initialize array with existing scales
        let mut steps_arr = original_steps.to_vec();

        // Replace the closest color with the input color
        steps_arr[index] = input_hsla;

        // Update the hue of all other colors in the scale to match the input
        for i in 0..12 {
            if i != index {
                steps_arr[i].h = input_hsla.h;
            }
        }

        // Convert to array
        let steps_hsla: [Hsla; 12] = steps_arr.try_into().expect("hue array wrong size");

        steps_hsla
    }
}

impl Default for ColorScale {
    fn default() -> Self {
        Self {
            name: Some("Untitled Scale".into()),
            steps: Some(vec![
                hsla(0.0, 1.00, 0.99, 1.0),
                hsla(0.0, 1.00, 0.98, 1.0),
                hsla(0.0, 0.90, 0.96, 1.0),
                hsla(0.0, 1.00, 0.93, 1.0),
                hsla(0.0, 1.00, 0.90, 1.0),
                hsla(0.0, 0.94, 0.87, 1.0),
                hsla(0.0, 0.77, 0.81, 1.0),
                hsla(0.0, 0.70, 0.74, 1.0),
                hsla(0.0, 0.75, 0.59, 1.0),
                hsla(0.0, 0.69, 0.55, 1.0),
                hsla(0.0, 0.65, 0.49, 1.0),
                hsla(0.0, 0.63, 0.24, 1.0),
            ]),
        }
    }
}

impl From<ColorScaleName> for ColorScale {
    fn from(scale: ColorScaleName) -> ColorScale {
        ColorScale::from_default(scale)
    }
}

impl From<Hsla> for ColorScale {
    fn from(hsla: Hsla) -> ColorScale {
        ColorScale::from_hsla(None, hsla)
    }
}


#[derive(Debug, Default, Clone)]
pub enum Appearance {
    #[default]
    Dark,
    Light,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub url: Option<String>,
    pub appearances: Vec<ThemeVariant>,
    pub default_appearance: usize,
}

impl Theme {
    pub fn new(
        name: impl Into<String>,
        author: Option<impl Into<String>>,
        url: Option<impl Into<String>>,
        appearances: Vec<ThemeVariant>,
        default_appearance: usize,
    ) -> Theme {
        Theme {
            name: name.into(),
            author: author.map(Into::into),
            url: url.map(Into::into),
            appearances,
            default_appearance,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn author(&mut self, author: String) -> &mut Self {
        self.author = Some(author);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn appearances(&mut self, appearances: Vec<ThemeVariant>) -> &mut Self {
        self.appearances = appearances;
        self
    }

    pub fn default_appearance(&mut self, default_appearance: usize) -> &mut Self {
        self.default_appearance = default_appearance;
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Zed".to_string(),
            author: None,
            url: None,
            appearances: vec![
                ThemeVariant::default_dark(),
                ThemeVariant::default_light()
            ],
            default_appearance: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequiredScales {
    pub neutral: ColorScale,
    pub accent: ColorScale,
    pub positive: ColorScale,
    pub negative: ColorScale,
    pub caution: ColorScale,
}

impl RequiredScales {
    pub fn new(
        neutral: ColorScale,
        accent: ColorScale,
        positive: ColorScale,
        negative: ColorScale,
        caution: ColorScale,
    ) -> Self {
        Self {
            neutral,
            accent,
            positive,
            negative,
            caution,
        }
    }
}

type ExtendedScales = Vec<ColorScale>;

#[derive(Debug, Clone)]
pub struct ThemeVariant {
    pub id: usize,
    pub name: String,
    pub appearance: Appearance,
    pub scales: RequiredScales,
    pub extended_scales: ExtendedScales,
    pub color: ThemeColor,
}

impl ThemeVariant {
    pub fn default_dark() -> Self {
        let required_scales = RequiredScales {
            neutral: ColorScaleName::Slate.into(),
            accent: ColorScaleName::Blue.into(),
            positive: ColorScaleName::Green.into(),
            negative: ColorScaleName::Red.into(),
            caution: ColorScaleName::Amber.into()
        };

        Self {
            id: 0,
            name: "Zed Dark".into(),
            appearance: Appearance::Dark,
            scales: required_scales.clone(),
            extended_scales: vec![
                ColorScaleName::Bronze.into(),
                ColorScaleName::Brown.into(),
                ColorScaleName::Crimson.into(),
                ColorScaleName::Cyan.into(),
                ColorScaleName::Gold.into(),
                ColorScaleName::Grass.into(),
                ColorScaleName::Gray.into(),
                ColorScaleName::Indigo.into(),
                ColorScaleName::Iris.into(),
                ColorScaleName::Jade.into(),
                ColorScaleName::Lime.into(),
                ColorScaleName::Mauve.into(),
                ColorScaleName::Mint.into(),
                ColorScaleName::Olive.into(),
                ColorScaleName::Orange.into(),
                ColorScaleName::Pink.into(),
                ColorScaleName::Plum.into(),
                ColorScaleName::Purple.into(),
                ColorScaleName::Sage.into(),
                ColorScaleName::Sand.into(),
                ColorScaleName::Sky.into(),
                ColorScaleName::Teal.into(),
                ColorScaleName::Tomato.into(),
                ColorScaleName::Violet.into(),
                ColorScaleName::Yellow.into(),
            ],
            color: ThemeColor::new(required_scales.clone()),
        }
    }

    pub fn default_light() -> Self {
        let required_scales = RequiredScales {
            neutral: ColorScaleName::Slate.into(),
            accent: ColorScaleName::Blue.into(),
            positive: ColorScaleName::Green.into(),
            negative: ColorScaleName::Red.into(),
            caution: ColorScaleName::Amber.into()
        };

        Self {
            id: 0,
            name: "Zed Light".into(),
            appearance: Appearance::Light,
            scales: required_scales.clone(),
            extended_scales: vec![
                ColorScaleName::Bronze.into(),
                ColorScaleName::Brown.into(),
                ColorScaleName::Crimson.into(),
                ColorScaleName::Cyan.into(),
                ColorScaleName::Gold.into(),
                ColorScaleName::Grass.into(),
                ColorScaleName::Gray.into(),
                ColorScaleName::Indigo.into(),
                ColorScaleName::Iris.into(),
                ColorScaleName::Jade.into(),
                ColorScaleName::Lime.into(),
                ColorScaleName::Mauve.into(),
                ColorScaleName::Mint.into(),
                ColorScaleName::Olive.into(),
                ColorScaleName::Orange.into(),
                ColorScaleName::Pink.into(),
                ColorScaleName::Plum.into(),
                ColorScaleName::Purple.into(),
                ColorScaleName::Sage.into(),
                ColorScaleName::Sand.into(),
                ColorScaleName::Sky.into(),
                ColorScaleName::Teal.into(),
                ColorScaleName::Tomato.into(),
                ColorScaleName::Violet.into(),
                ColorScaleName::Yellow.into(),
            ],
            color: ThemeColor::new(required_scales.clone()),
        }
    }
}
