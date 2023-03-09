pub enum CoolingStatus {
    CoolingBottom(String),
    CoolingTop(String),
    CoolingAll(String),
    Disabled(String),
    CooldownMode(String),
    Overheat(String)
}

pub struct Fursuit {
    pub intern_temp: i64,
    pub extern_temp: i64,
    pub cooling_status: CoolingStatus,
    pub suit_fw_version: Option<String>,
    pub cooling_enabled: bool,
    pub luma_enabled: bool,
    pub gradiant_system_enabled: bool,
    pub sound_system_enabled: bool
}