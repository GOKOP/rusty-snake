// Note that this is *not* a suckless-style config file
// It's just a module dealing with settings logic

pub struct Setting {
    pub name: String,
    pub help: String,
    pub short: char,
    pub validator: fn(String) -> Result<(), String>,
}
