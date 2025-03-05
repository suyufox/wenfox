use colored::Colorize;
use std::env;

pub struct WenhuCliMessage {
    pub version: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub build_time: &'static str,
    pub license: &'static str,
    pub author: &'static str,
}

pub static WENHU_CLI_MESSAGE: WenhuCliMessage = WenhuCliMessage {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    build_time: env!("BUILD_TIMESTAMP"),
    license: env!("CARGO_PKG_LICENSE"),
    author: env!("CARGO_PKG_AUTHORS"),
};

lazy_static::lazy_static! {
    static ref SEPARATOR: String = "──────────────────────────────────────────────────────────────────────────"
        .bright_cyan()
        .to_string();

    pub static ref HELP_TEMPLATE: String = format!(
            r#"{{before-help}} {{about-section}}{}
{{usage-heading}} {{usage}}
{{all-args}}
            
{}{{after-help}}"#,
            *SEPARATOR,  // 上方分隔线
            *SEPARATOR   // 下方分隔线
    );

    pub static ref BEFORE_HELP: String = format!(
        r#"> {} ──── @{} "#,
            WENHU_CLI_MESSAGE.name.red().bold(),
            WENHU_CLI_MESSAGE.version.bright_cyan()
    );

    pub static ref AFTER_HELP: String = format!(
        "{}:
  # {}: \x1B[1;31m\x1B]8;;https://github.com/{}/{}\x1B\\{}\x1B]8;;\x1B\\\x1B[0m - {}
  # {}: \x1B]8;;https://github.com/{}\x1B\\@{}\x1B]8;;\x1B\\\x1B[0m
  # {}: \x1B]8;;https://opensource.org/licenses/{}\x1B\\{}\x1B]8;;\x1B\\\x1B[0m
  # {}: {}
{}",
            "Packet information".bright_magenta().bold(),
            "Name".bright_cyan(),
            WENHU_CLI_MESSAGE.author,
            WENHU_CLI_MESSAGE.name,
            WENHU_CLI_MESSAGE.name.red().bold(),
            WENHU_CLI_MESSAGE.version.bright_cyan(),
            "Author".bright_cyan(),
            WENHU_CLI_MESSAGE.author,
            WENHU_CLI_MESSAGE.author.yellow(),
            "License".bright_cyan(),
            WENHU_CLI_MESSAGE.license,
            WENHU_CLI_MESSAGE.license.bright_blue(),
            "Build-Time".bright_cyan(),
            WENHU_CLI_MESSAGE.build_time.bright_white(),
            *SEPARATOR
    );
}
