error_chain! {
    foreign_links {
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        TomlSer(::toml::ser::Error);
        SetLogger(::log::SetLoggerError);
    }

    errors {
        ConfigError(msg: String) {
            description("configuration error")
            display("configuration error: {}", msg)
        }
    }
}
