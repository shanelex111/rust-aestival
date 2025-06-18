use config::Config;

pub fn load() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config").format(config::FileFormat::Yaml))
        .build()
        .unwrap()
}
