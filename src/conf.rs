//! The `conf` module contains functions for loading and saving game
//! configurations.
//!
//! A `Conf` struct is used to specify hardware setup stuff used to create
//! the window and other context information.

use std::io;
use toml;

use GameResult;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FullscreenType {
    Off,
    True,
    Desktop
}

use sdl2::video::FullscreenType as SdlFullscreenType;
impl From<SdlFullscreenType> for FullscreenType {
    fn from(other: SdlFullscreenType) -> Self {
        match other {
            SdlFullscreenType::Off => FullscreenType::Off,
            SdlFullscreenType::True => FullscreenType::True,
            SdlFullscreenType::Desktop => FullscreenType::Desktop,
        }
    }
}

impl From<FullscreenType> for SdlFullscreenType {
    fn from(other: FullscreenType) -> Self {
        match other {
            FullscreenType::Off => SdlFullscreenType::Off,
            FullscreenType::True => SdlFullscreenType::True,
            FullscreenType::Desktop => SdlFullscreenType::Desktop,
        }
    }

}

/// A builder structure containing flags for defining window settings.
///
/// Defaults:
///
/// ```rust,ignore
/// WindowMode {
///     borderless: false,
///     resizable: false,
///     fullscreen_type: FullscreenType::Off,
///     vsync: true,
///     min_dimensions: (0, 0),
///     max_dimensions: (0, 0),
/// }
/// ```
#[derive(Debug, Copy, Clone, SmartDefault, Serialize, Deserialize, PartialEq, Eq)]
pub struct WindowMode {
    #[default = r#"false"#]
    pub borderless: bool,
    #[default = r#"false"#]
    pub resizable: bool,
    #[default = r#"FullscreenType::Off"#]
    pub fullscreen_type: FullscreenType,
    #[default = r#"true"#]
    pub vsync: bool,
    /// Minimum dimensions for resizable windows; (0, 0) means no limit
    #[default = r#"(0, 0)"#]
    pub min_dimensions: (u32, u32),
    /// Maximum dimensions for resizable windows; (0, 0) means no limit
    #[default = r#"(0, 0)"#]
    pub max_dimensions: (u32, u32),
}


impl WindowMode {
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    pub fn fullscreen_type(mut self, fullscreen_type: FullscreenType) -> Self {
        self.fullscreen_type = fullscreen_type;
        self
    }

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn min_dimensions(mut self, width: u32, height: u32) -> Self {
        self.min_dimensions = (width, height);
        self
    }

    pub fn max_dimensions(mut self, width: u32, height: u32) -> Self {
        self.max_dimensions = (width, height);
        self
    }
}


/// A structure containing configuration data
/// for the game engine.
///
/// Defaults:
///
/// ```rust,ignore
/// Conf {
///     window_title: "An easy, good game"
///     window_icon: ""
///     window_height: 600
///     window_width: 800
///     vsync: true
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, SmartDefault)]
pub struct Conf {
    /// The window title.
    #[default = r#""An easy, good game".to_owned()"#]
    pub window_title: String,
    /// A file path to the window's icon.
    /// It is rooted in the `resources` directory (see the `filesystem` module for details),
    /// and an empty string results in a blank/default icon.
    #[default = r#""".to_owned()"#]
    pub window_icon: String,
    /// The window's height
    #[default = "600"]
    pub window_height: u32,
    /// The window's width
    #[default = "800"]
    pub window_width: u32,
    pub window_mode: WindowMode,
}

impl Conf {
    /// Same as Conf::default()
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a TOML file from the given `Read` and attempts to parse
    /// a `Conf` from it.
    pub fn from_toml_file<R: io::Read>(file: &mut R) -> GameResult<Conf> {
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let decoded = toml::from_str(&s)?;
        Ok(decoded)
    }

    /// Saves the `Conf` to the given `Write` object,
    /// formatted as TOML.
    pub fn to_toml_file<W: io::Write>(&self, file: &mut W) -> GameResult<()> {
        let s = toml::to_vec(self)?;
        file.write_all(&s)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use conf;
    use serde_json;

    /// Tries to encode and decode a `Conf` object
    /// and makes sure it gets the same result it had.
    #[test]
    fn encode_round_trip() {
        let c1 = conf::Conf::new();
        let mut writer = Vec::new();
        let s = serde_json::to_string(&c1);
        println!("S is {:?}", s);
        let _c = c1.to_toml_file(&mut writer).unwrap();
        println!("{}", String::from_utf8_lossy(&writer));
        let mut reader = writer.as_slice();
        let c2 = conf::Conf::from_toml_file(&mut reader).unwrap();
        assert_eq!(c1, c2);
    }
}
