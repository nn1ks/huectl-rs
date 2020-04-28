use crate::arg;
use huelib::resource::{self, ModifierType};
use huelib::Color;

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
pub struct ColorHex {
    pub value: Color,
}

impl std::str::FromStr for ColorHex {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        Ok(Self {
            value: Color::from_hex(s).map_err(|_| {
                arg::ParseError::new("The value must begin with `#` followed by 3 or 6 hex values.")
            })?,
        })
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
    pub value: resource::Alert,
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
            "select" => resource::Alert::Select,
            "lselect" => resource::Alert::LSelect,
            "none" => resource::Alert::None,
            _ => return Err(arg::ParseError::new("Invalid value for alert")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct Effect {
    pub value: resource::Effect,
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
            "colorloop" => resource::Effect::Colorloop,
            "none" => resource::Effect::None,
            _ => return Err(arg::ParseError::new("Invalid value for effect")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct GroupTypeCreator {
    pub value: resource::group::CreatableKind,
}

impl GroupTypeCreator {
    pub fn variants() -> &'static [&'static str] {
        &["lightgroup", "room", "entertainment", "zone"]
    }
}

impl std::str::FromStr for GroupTypeCreator {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use resource::group::CreatableKind;
        let value = match s.to_lowercase().as_ref() {
            "lightgroup" => CreatableKind::LightGroup,
            "room" => CreatableKind::Room,
            "entertainment" => CreatableKind::Entertainment,
            "zone" => CreatableKind::Zone,
            _ => return Err(arg::ParseError::new("Invalid value for kind")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct GroupClass {
    pub value: resource::group::Class,
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
        use resource::group::Class;
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
    pub value: resource::scene::Kind,
}

impl SceneType {
    pub fn variants() -> &'static [&'static str] {
        &["lightscene", "groupscene"]
    }
}

impl std::str::FromStr for SceneType {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use resource::scene::Kind;
        let value = match s.to_lowercase().as_ref() {
            "lightscene" => Kind::LightScene,
            "groupscene" => Kind::GroupScene,
            _ => return Err(arg::ParseError::new("Invalid value for kind")),
        };
        Ok(Self { value })
    }
}

#[derive(Debug)]
pub struct ScheduleRequestType {
    pub value: resource::ActionRequestType,
}

impl ScheduleRequestType {
    pub fn variants() -> &'static [&'static str] {
        &["put", "post", "delete"]
    }
}

impl std::str::FromStr for ScheduleRequestType {
    type Err = arg::ParseError;
    fn from_str(s: &str) -> Result<Self, arg::ParseError> {
        use resource::ActionRequestType;
        let value = match s.to_lowercase().as_ref() {
            "put" => ActionRequestType::Put,
            "post" => ActionRequestType::Post,
            "delete" => ActionRequestType::Delete,
            _ => return Err(arg::ParseError::new("Invalid value for request type")),
        };
        Ok(Self { value })
    }
}
