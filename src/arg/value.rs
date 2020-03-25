use crate::arg;
use huelib::{CoordinateModifierType, ModifierType};

fn parse_with_suffix<T: std::str::FromStr>(
    value_str: &str,
    error: arg::ParseError,
) -> Result<(ModifierType, T), arg::ParseError> {
    let modifier_type = match value_str.chars().next() {
        Some(v) => match v {
            '+' => ModifierType::Increment,
            '-' => ModifierType::Decrement,
            _ => ModifierType::Override,
        },
        None => return Err(error),
    };
    let value =
        if modifier_type == ModifierType::Increment || modifier_type == ModifierType::Decrement {
            value_str.split_at(1).1
        } else {
            value_str
        };
    Ok((modifier_type, value.parse().map_err(|_| error)?))
}

#[derive(Debug)]
pub struct Brightness {
    pub modifier_type: ModifierType,
    pub value: u8,
}

impl std::str::FromStr for Brightness {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let max_value = 100;
        let error = arg::ParseError::from_integer_value(&max_value);
        let (modifier_type, value) = parse_with_suffix::<u8>(s, error.clone())?;
        if value > max_value {
            return Err(error);
        }
        Ok(Self {
            modifier_type,
            value: (value as f32 * (u8::max_value() as f32 / max_value as f32)) as u8,
        })
    }
}

#[derive(Debug)]
pub struct Hue {
    pub modifier_type: ModifierType,
    pub value: u16,
}

impl std::str::FromStr for Hue {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let error = arg::ParseError::from_integer_value(&u16::max_value());
        let (modifier_type, value) = parse_with_suffix::<u16>(s, error)?;
        Ok(Self {
            modifier_type,
            value,
        })
    }
}

#[derive(Debug)]
pub struct Saturation {
    pub modifier_type: ModifierType,
    pub value: u8,
}

impl std::str::FromStr for Saturation {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let max_value = 100;
        let error = arg::ParseError::from_integer_value(&max_value);
        let (modifier_type, value) = parse_with_suffix::<u8>(s, error.clone())?;
        if value > max_value {
            return Err(error);
        }
        Ok(Self {
            modifier_type,
            value: (value as f32 * (u8::max_value() as f32 / max_value as f32)) as u8,
        })
    }
}

#[derive(Debug)]
pub struct ColorSpaceCoordinates {
    pub modifier_type: CoordinateModifierType,
    pub value: (f32, f32),
}

impl std::str::FromStr for ColorSpaceCoordinates {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let values: Vec<&str> = s.split(',').collect();
        let (x, y) = match values[..] {
            [x, y] => {
                let error =
                    arg::ParseError::new("The value must be a floating point number with 32 bits.");
                (
                    parse_with_suffix::<f32>(x, error.clone())?,
                    parse_with_suffix::<f32>(y, error)?,
                )
            }
            _ => {
                return Err(arg::ParseError::new(
                    "The value must be a comma seperated list of floating point numbers.",
                ))
            }
        };
        let modifier_type = match (x.0, y.0) {
            (ModifierType::Override, ModifierType::Override) => CoordinateModifierType::Override,
            (ModifierType::Increment, ModifierType::Increment) => CoordinateModifierType::Increment,
            (ModifierType::Decrement, ModifierType::Decrement) => CoordinateModifierType::Decrement,
            (ModifierType::Increment, ModifierType::Decrement) => {
                CoordinateModifierType::IncrementDecrement
            }
            (ModifierType::Decrement, ModifierType::Increment) => {
                CoordinateModifierType::DecrementIncrement
            }
            _ => {
                return Err(arg::ParseError::new(
                    "Either both coordinates must have a prefix or both coordinates must have no prefix"
                ));
            }
        };
        let max_value = match modifier_type {
            CoordinateModifierType::Override => 1.0,
            _ => 0.5,
        };
        if x.1 > max_value || x.1 < 0.0 || y.1 > max_value || y.1 < 0.0 {
            Err(arg::ParseError::from_float_value(&max_value))
        } else {
            Ok(Self {
                modifier_type,
                value: (x.1, y.1),
            })
        }
    }
}

#[derive(Debug)]
pub struct ColorTemperature {
    pub modifier_type: ModifierType,
    pub value: u16,
}

impl std::str::FromStr for ColorTemperature {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let error = arg::ParseError::from_integer_value(&u16::max_value());
        let (modifier_type, value) = parse_with_suffix::<u16>(s, error)?;
        Ok(Self {
            modifier_type,
            value,
        })
    }
}

#[derive(Debug)]
pub struct Alert {
    pub value: huelib::Alert,
}

impl Alert {
    pub fn variants() -> &'static [&'static str] {
        &["select", "lselect", "none"]
    }
}

impl std::str::FromStr for Alert {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let value = match s.to_lowercase().as_ref() {
            "select" => huelib::Alert::Select,
            "lselect" => huelib::Alert::LSelect,
            "none" => huelib::Alert::None,
            _ => return Err(arg::ParseError::new("Invalid value for alert")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct Effect {
    pub value: huelib::Effect,
}

impl Effect {
    pub fn variants() -> &'static [&'static str] {
        &["colorloop", "none"]
    }
}

impl std::str::FromStr for Effect {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        let value = match s.to_lowercase().as_ref() {
            "colorloop" => huelib::Effect::Colorloop,
            "none" => huelib::Effect::None,
            _ => return Err(arg::ParseError::new("Invalid value for effect")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct GroupTypeCreator {
    pub value: huelib::group::TypeCreator,
}

impl GroupTypeCreator {
    pub fn variants() -> &'static [&'static str] {
        &["lightgroup", "room", "entertainment", "zone"]
    }
}

impl std::str::FromStr for GroupTypeCreator {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use huelib::group::TypeCreator;
        let value = match s.to_lowercase().as_ref() {
            "lightgroup" => TypeCreator::LightGroup,
            "room" => TypeCreator::Room,
            "entertainment" => TypeCreator::Entertainment,
            "zone" => TypeCreator::Zone,
            _ => return Err(arg::ParseError::new("Invalid value for kind")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct GroupClass {
    pub value: huelib::group::Class,
}

impl GroupClass {
    pub fn variants() -> &'static [&'static str] {
        &[
            "attic",
            "balcony",
            "barbecue",
            "bathroom",
            "bedroom",
            "carport",
            "closet",
            "computer",
            "dining",
            "downstairs",
            "driveway",
            "frontdoor",
            "garage",
            "garden",
            "guestroom",
            "gym",
            "hallway",
            "home",
            "kidsbedroom",
            "kitchen",
            "laundryroom",
            "livingroom",
            "lounge",
            "mancave",
            "music",
            "nursery",
            "office",
            "other",
            "pool",
            "porch",
            "reading",
            "recreation",
            "staircase",
            "storage",
            "studio",
            "tv",
            "terrace",
            "toilet",
            "topfloor",
            "upstairs",
        ]
    }
}

impl std::str::FromStr for GroupClass {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use huelib::group::Class;
        let value = match s.to_lowercase().as_ref() {
            "attic" => Class::Attic,
            "balcony" => Class::Balcony,
            "barbecue" => Class::Barbecue,
            "bathroom" => Class::Bathroom,
            "bedroom" => Class::Bedroom,
            "carport" => Class::Carport,
            "closet" => Class::Closet,
            "computer" => Class::Computer,
            "dining" => Class::Dining,
            "downstairs" => Class::Downstairs,
            "driveway" => Class::Driveway,
            "frontdoor" => Class::FrontDoor,
            "garage" => Class::Garage,
            "garden" => Class::Garden,
            "guestroom" => Class::GuestRoom,
            "gym" => Class::Gym,
            "hallway" => Class::Hallway,
            "home" => Class::Home,
            "kidsbedroom" => Class::KidsBedroom,
            "kitchen" => Class::Kitchen,
            "laundryroom" => Class::LaundryRoom,
            "livingroom" => Class::LivingRoom,
            "lounge" => Class::Lounge,
            "mancave" => Class::ManCave,
            "music" => Class::Music,
            "nursery" => Class::Nursery,
            "office" => Class::Office,
            "other" => Class::Other,
            "pool" => Class::Pool,
            "porch" => Class::Porch,
            "reading" => Class::Reading,
            "recreation" => Class::Recreation,
            "staircase" => Class::Staircase,
            "storage" => Class::Storage,
            "studio" => Class::Studio,
            "tv" => Class::TV,
            "terrace" => Class::Terrace,
            "toilet" => Class::Toilet,
            "topfloor" => Class::TopFloor,
            "upstairs" => Class::Upstairs,
            _ => return Err(arg::ParseError::new("Invalid value for class")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct SceneType {
    pub value: huelib::scene::Kind,
}

impl SceneType {
    pub fn variants() -> &'static [&'static str] {
        &["lightscene", "groupscene"]
    }
}

impl std::str::FromStr for SceneType {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use huelib::scene::Kind;
        let value = match s.to_lowercase().as_ref() {
            "lightscene" => Kind::LightScene,
            "groupscene" => Kind::GroupScene,
            _ => return Err(arg::ParseError::new("Invalid value for kind")),
        };
        Ok(Self { value })
    }
}
