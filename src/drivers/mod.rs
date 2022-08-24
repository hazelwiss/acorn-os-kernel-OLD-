pub mod interfaces;
pub mod x64;

pub struct Desc {
    pub entry: fn(),
    pub iserialin: Option<interfaces::serial::ISerialIn>,
    pub iserialout: Option<interfaces::serial::ISerialOut>,
    pub console: Option<interfaces::serial::IConsole>,
}
