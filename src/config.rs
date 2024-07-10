use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Config {
    pub jar_path: String,
    pub main_args: String,
    pub java_home: String,
    pub jvm_args: String,
}

impl Config {
    pub fn load(path: &str) -> Option<Config> {
        let path = Path::new(&path);
        if !path.exists() {
            eprintln!("path {} not exist", path.to_str().unwrap());
            return None;
        }
        let data = fs::read_to_string(path).unwrap();
        let properties = Self::parse(data.as_str());
        let jar_path = properties.get(&("JAR", "jar_path")).unwrap_or(&"").to_string();
        let main_args = properties.get(&("JAR", "main_args")).unwrap_or(&"").to_string();

        let java_home = properties.get(&("JVM", "java_home")).unwrap_or(&"").to_string();
        let jvm_args = properties.get(&("JVM", "jvm_args")).unwrap_or(&"").to_string();

        let config = Config {
            jar_path,
            main_args,
            java_home,
            jvm_args,
        };

        Some(config)
    }

    fn parse(s: &str) -> HashMap<(&str, &str), &str> {
        use ini_core::*;
        let mut map = HashMap::new();
        let mut sect = "";
        for line in Parser::new(s) {
            match line {
                Item::Section(section) => {
                    sect = section;
                }
                Item::Property(key, value) => {
                    match value {
                        Some(value) => {
                            let _ = map.insert((sect, key), value);
                        }
                        None => {
                            eprintln!("[{}.{}]的值不存在", &sect, &key);
                        }
                    };
                }
                _ => (),
            }
        }
        map
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{\n jar_path: {},\n main_args: {},\n jvm_args: {},\n }}",
            self.jar_path,
            self.main_args,
            self.jvm_args,
        )
    }
}
