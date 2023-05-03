pub struct Config {
    pub appdata_path: String,
}

impl Config {
    pub fn new() -> Self {
        let name = match std::env::var("USERNAME") {
            Ok(username) => username,
            Err(_) => match std::env::var("USER") {
                Ok(username) => username,
                Err(_) => String::from("unknown"),
            },
        };

        let env = match std::env::var("ENV") {
            Ok(env) => env,
            Err(_) => String::from("prod"),
        };
        let mut config: Config = Config {
            appdata_path: String::from(""),
        };

        if env == "dev" {
            config.appdata_path = String::from("./data");
        } else {
            config.appdata_path = String::from(format!("C:\\Users\\{}\\AppData\\Roaming\\sleepy\\", &name));
        }

        config
    }
}