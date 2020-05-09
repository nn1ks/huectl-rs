use huelib::resource;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Config {
    name: String,
    software_update: ConfigSoftwareUpdate,
    software_version: String,
    api_version: String,
    link_button: bool,
    ip_address: String,
    mac_address: String,
    netmask: String,
    gateway: String,
    dhcp: bool,
    portal_services: bool,
    portal_connection: String,
    // TODO: portal_state
    // TODO: internet_services
    current_time: String,
    local_time: Option<String>,
    timezone: Option<String>,
    zigbee_channel: u8,
    model_id: String,
    bridge_id: String,
    factory_new: bool,
    replaces_bridge_id: Option<String>,
    datastore_version: String,
    starterkit_id: String,
    backup: ConfigBackup,
    whitelist: Vec<ConfigUser>,
}

impl std::convert::From<resource::Config> for Config {
    fn from(v: resource::Config) -> Self {
        use resource::config::ServiceStatus;
        Self {
            name: v.name,
            software_update: ConfigSoftwareUpdate::from(v.software_update),
            software_version: v.software_version,
            api_version: v.api_version,
            link_button: v.link_button,
            ip_address: v.ip_address.to_string(),
            mac_address: v.mac_address,
            netmask: v.netmask,
            gateway: v.gateway.to_string(),
            dhcp: v.dhcp,
            portal_services: v.portal_services,
            portal_connection: match v.portal_connection {
                ServiceStatus::Connected => "Connected",
                ServiceStatus::Disconnected => "Disconnected",
            }
            .to_owned(),
            current_time: v.current_time.to_string(),
            local_time: v.local_time.map(|v| v.to_string()),
            timezone: v.timezone,
            zigbee_channel: v.zigbee_channel,
            model_id: v.model_id,
            bridge_id: v.bridge_id,
            factory_new: v.factory_new,
            replaces_bridge_id: v.replaces_bridge_id,
            datastore_version: v.datastore_version,
            starterkit_id: v.starterkit_id,
            backup: ConfigBackup::from(v.backup),
            whitelist: v.whitelist.into_iter().map(ConfigUser::from).collect(),
        }
    }
}

#[derive(Serialize)]
struct ConfigSoftwareUpdate {
    state: String,
    check: bool,
    last_change: Option<String>,
    last_install: Option<String>,
    auto_install_on: bool,
    auto_install_time: Option<String>,
}

impl std::convert::From<resource::config::SoftwareUpdate> for ConfigSoftwareUpdate {
    fn from(v: resource::config::SoftwareUpdate) -> Self {
        Self {
            state: format!("{:?}", v.state),
            check: v.check,
            last_change: v.last_change.map(|v| v.to_string()),
            last_install: v.last_install.map(|v| v.to_string()),
            auto_install_on: v.auto_install.on,
            auto_install_time: v.auto_install.update_time.map(|v| v.to_string()),
        }
    }
}

#[derive(Serialize)]
struct ConfigBackup {
    status: String,
    error: Option<String>,
}

impl std::convert::From<resource::config::Backup> for ConfigBackup {
    fn from(v: resource::config::Backup) -> Self {
        use resource::config::{BackupError, BackupStatus};
        Self {
            status: match v.status {
                BackupStatus::Idle => "Idle",
                BackupStatus::StartMigration => "StartMigration",
                BackupStatus::FilereadyDisabled => "FilereadyDisabled",
                BackupStatus::PrepareRestore => "PrepareRestore",
                BackupStatus::Restoring => "Restoring",
            }
            .to_owned(),
            error: match v.error {
                BackupError::ExportFailed => Some("ExportFailed".to_owned()),
                BackupError::ImportFailed => Some("ImportFailed".to_owned()),
                BackupError::None => None,
            },
        }
    }
}

#[derive(Serialize)]
struct ConfigUser {
    id: String,
    name: String,
    last_used: String,
    created: String,
}

impl std::convert::From<resource::config::User> for ConfigUser {
    fn from(v: resource::config::User) -> Self {
        Self {
            id: v.id,
            name: v.name,
            last_used: v.last_use_date.to_string(),
            created: v.create_date.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Group {
    id: String,
    name: String,
    lights: Vec<String>,
    sensors: Vec<String>,
    kind: String,
    class: Option<String>,
    state: Option<GroupState>,
    model_id: Option<String>,
    unique_id: Option<String>,
    recycle: Option<bool>,
}

impl std::convert::From<resource::Group> for Group {
    fn from(v: resource::Group) -> Self {
        use resource::group::Kind;
        Self {
            id: v.id,
            name: v.name,
            lights: v.lights,
            sensors: v.sensors,
            kind: match v.kind {
                Kind::Creatable(v) => format!("{:?}", v),
                Kind::Immutable(v) => format!("{:?}", v),
            },
            class: v.class.map(|v| format!("{:?}", v)),
            state: v.state.map(GroupState::from),
            model_id: v.model_id,
            unique_id: v.unique_id,
            recycle: v.recycle,
        }
    }
}

#[derive(Serialize)]
struct GroupState {
    any_on: bool,
    all_on: bool,
}

impl std::convert::From<resource::group::State> for GroupState {
    fn from(v: resource::group::State) -> Self {
        Self {
            any_on: v.any_on,
            all_on: v.all_on,
        }
    }
}

#[derive(Serialize)]
pub struct Light {
    id: String,
    name: String,
    kind: String,
    state: LightState,
    model_id: String,
    unique_id: String,
    product_id: Option<String>,
    product_name: Option<String>,
    manufacturer_name: Option<String>,
    software_version: String,
    software_update: LightSoftwareUpdate,
    // TODO: config
    // TODO: capabilities
}

impl std::convert::From<resource::Light> for Light {
    fn from(v: resource::Light) -> Self {
        Self {
            id: v.id,
            name: v.name,
            kind: v.kind,
            state: LightState::from(v.state),
            model_id: v.model_id,
            unique_id: v.unique_id,
            product_id: v.product_id,
            product_name: v.product_name,
            manufacturer_name: v.manufacturer_name,
            software_version: v.software_version,
            software_update: LightSoftwareUpdate::from(v.software_update),
        }
    }
}

#[derive(Serialize)]
struct LightState {
    on: Option<bool>,
    brightness: Option<u8>,
    hue: Option<u16>,
    saturation: Option<u8>,
    color_space_coordinates: Option<(f32, f32)>,
    color_temperature: Option<u16>,
    alert: Option<Alert>,
    effect: Option<Effect>,
    color_mode: Option<ColorMode>,
    reachable: bool,
}

impl std::convert::From<resource::light::State> for LightState {
    fn from(v: resource::light::State) -> Self {
        Self {
            on: v.on,
            brightness: v.brightness,
            hue: v.hue,
            saturation: v.saturation,
            color_space_coordinates: v.color_space_coordinates,
            color_temperature: v.color_temperature,
            alert: v.alert.map(Alert::from),
            effect: v.effect.map(Effect::from),
            color_mode: v.color_mode.map(ColorMode::from),
            reachable: v.reachable,
        }
    }
}

#[derive(Serialize)]
struct LightSoftwareUpdate {
    state: String,
    last_install: Option<String>,
}

impl std::convert::From<resource::light::SoftwareUpdate> for LightSoftwareUpdate {
    fn from(v: resource::light::SoftwareUpdate) -> Self {
        Self {
            state: format!("{:?}", v.state),
            last_install: v.last_install.map(|v| v.to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct Resourcelink {
    id: String,
    name: String,
    description: String,
    owner: String,
    kind: String,
    class_id: u16,
    recycle: bool,
    links: Vec<ResourcelinkLink>,
}

impl std::convert::From<resource::Resourcelink> for Resourcelink {
    fn from(v: resource::Resourcelink) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            owner: v.owner,
            kind: format!("{:?}", v.kind),
            class_id: v.class_id,
            recycle: v.recycle,
            links: v.links.into_iter().map(ResourcelinkLink::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct ResourcelinkLink {
    kind: String,
    id: String,
}

impl std::convert::From<resource::resourcelink::Link> for ResourcelinkLink {
    fn from(v: resource::resourcelink::Link) -> Self {
        Self {
            kind: format!("{:?}", v.kind),
            id: v.id,
        }
    }
}

#[derive(Serialize)]
pub struct Rule {
    id: String,
    name: String,
    owner: Option<String>,
    last_triggered: Option<String>,
    times_triggered: usize,
    created: String,
    status: String,
    conditions: Vec<RuleCondition>,
    actions: Vec<Action>,
}

impl std::convert::From<resource::Rule> for Rule {
    fn from(v: resource::Rule) -> Self {
        Self {
            id: v.id,
            name: v.name,
            owner: v.owner,
            last_triggered: v.last_triggered.map(|v| v.to_string()),
            times_triggered: v.times_triggered,
            created: v.created.to_string(),
            status: format!("{:?}", v.status),
            conditions: v.conditions.into_iter().map(RuleCondition::from).collect(),
            actions: v.actions.into_iter().map(Action::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct RuleCondition {
    address: String,
    operator: String,
    value: Option<String>,
}

impl std::convert::From<resource::rule::Condition> for RuleCondition {
    fn from(v: resource::rule::Condition) -> Self {
        Self {
            address: v.address,
            operator: format!("{:?}", v.operator),
            value: v.value,
        }
    }
}

#[derive(Serialize)]
pub struct Scene {
    id: String,
    name: String,
    kind: String,
    group: Option<String>,
    lights: Option<Vec<String>>,
    owner: Option<String>,
    recycle: bool,
    locked: bool,
    app_data: SceneAppData,
    picture: Option<String>,
    last_update: Option<String>,
    version: String,
}

impl std::convert::From<resource::Scene> for Scene {
    fn from(v: resource::Scene) -> Self {
        Self {
            id: v.id,
            name: v.name,
            kind: format!("{:?}", v.kind),
            group: v.group,
            lights: v.lights,
            owner: v.owner,
            recycle: v.recycle,
            locked: v.locked,
            app_data: SceneAppData::from(v.app_data),
            picture: v.picture,
            last_update: v.last_update.map(|v| v.to_string()),
            version: format!("{:?}", v.version),
        }
    }
}

#[derive(Serialize)]
struct SceneAppData {
    version: Option<i8>,
    data: Option<String>,
}

impl std::convert::From<resource::scene::AppData> for SceneAppData {
    fn from(v: resource::scene::AppData) -> Self {
        Self {
            version: v.version,
            data: v.data,
        }
    }
}

#[derive(Serialize)]
pub struct Schedule {
    id: String,
    name: String,
    description: String,
    action: Action,
    local_time: String,
    start_time: Option<String>,
    status: String,
    auto_delete: Option<bool>,
}

impl std::convert::From<resource::Schedule> for Schedule {
    fn from(v: resource::Schedule) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            action: Action::from(v.action),
            local_time: v.local_time,
            start_time: v.start_time.map(|v| v.to_string()),
            status: format!("{:?}", v.status),
            auto_delete: v.auto_delete,
        }
    }
}

#[derive(Serialize)]
pub struct Sensor {
    id: String,
    name: String,
    type_name: String,
    model_id: String,
    unique_id: Option<String>,
    manufacturer_name: Option<String>,
    software_verion: String,
    state: SensorState,
    config: SensorConfig,
    recycle: Option<bool>,
}

impl std::convert::From<resource::Sensor> for Sensor {
    fn from(v: resource::Sensor) -> Self {
        Self {
            id: v.id,
            name: v.name,
            type_name: v.type_name,
            model_id: v.model_id,
            unique_id: v.unique_id,
            manufacturer_name: v.manufacturer_name,
            software_verion: v.software_verion,
            state: SensorState::from(v.state),
            config: SensorConfig::from(v.config),
            recycle: v.recycle,
        }
    }
}

#[derive(Serialize)]
pub struct SensorState {
    presence: Option<bool>,
    flag: Option<bool>,
    last_updated: Option<String>,
}

impl std::convert::From<resource::sensor::State> for SensorState {
    fn from(v: resource::sensor::State) -> Self {
        Self {
            presence: v.presence,
            flag: v.flag,
            last_updated: v.last_updated.map(|v| v.to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct SensorConfig {
    on: bool,
    reachable: Option<bool>,
    battery: Option<u8>,
}

impl std::convert::From<resource::sensor::Config> for SensorConfig {
    fn from(v: resource::sensor::Config) -> Self {
        Self {
            on: v.on,
            reachable: v.reachable,
            battery: v.battery,
        }
    }
}

#[derive(Serialize)]
pub struct Scan {
    last_scan: Option<String>,
    resources: Vec<ScanResource>,
}

impl std::convert::From<resource::Scan> for Scan {
    fn from(v: resource::Scan) -> Self {
        use resource::LastScan;
        Self {
            last_scan: match v.last_scan {
                LastScan::DateTime(v) => Some(v.to_string()),
                LastScan::Active => Some("Active".to_owned()),
                LastScan::None => None,
            },
            resources: v.resources.into_iter().map(ScanResource::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct ScanResource {
    id: String,
    name: String,
}

impl std::convert::From<resource::ScanResource> for ScanResource {
    fn from(v: resource::ScanResource) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

#[derive(Serialize)]
enum Alert {
    Select,
    LSelect,
    None,
}

impl std::convert::From<resource::Alert> for Alert {
    fn from(v: resource::Alert) -> Self {
        match v {
            resource::Alert::Select => Self::Select,
            resource::Alert::LSelect => Self::LSelect,
            resource::Alert::None => Self::None,
        }
    }
}

#[derive(Serialize)]
enum Effect {
    Colorloop,
    None,
}

impl std::convert::From<resource::Effect> for Effect {
    fn from(v: resource::Effect) -> Self {
        match v {
            resource::Effect::Colorloop => Self::Colorloop,
            resource::Effect::None => Self::None,
        }
    }
}

#[derive(Serialize)]
enum ColorMode {
    ColorSpaceCoordinates,
    ColorTemperature,
    HueAndSaturation,
}

impl std::convert::From<resource::ColorMode> for ColorMode {
    fn from(v: resource::ColorMode) -> Self {
        match v {
            resource::ColorMode::ColorSpaceCoordinates => Self::ColorSpaceCoordinates,
            resource::ColorMode::ColorTemperature => Self::ColorTemperature,
            resource::ColorMode::HueAndSaturation => Self::HueAndSaturation,
        }
    }
}

#[derive(Serialize)]
struct Action {
    address: String,
    request_type: String,
    body: HashMap<String, JsonValue>,
}

impl std::convert::From<resource::Action> for Action {
    fn from(v: resource::Action) -> Self {
        Self {
            address: v.address,
            request_type: format!("{:?}", v.request_type),
            body: v.body,
        }
    }
}
