pub const TEMPLATE_DIR: &str = "template/base";
pub const EXTRAS_DIR: &str = "template/extras";
pub const INDEX_MODULE_CSS_DIR:&str = "template/extras/src/index.module.css";
pub const OVERWRITE_OPTIONS: [&str; 2] = ["abort", "clear"];
pub const CONFIRM_OPTIONS: [&str; 2] = ["Yes", "No"];
pub const TAILWIND_CONFIGS: &[(&str, &str)] = &[
    ("config/tailwind.config.ts", "tailwind.config.ts"),
    ("config/postcss.config.js", "postcss.config.js"),
    ("config/_prettier.config.js", "prettier.config.js"),
    ("src/styles/globals.css", "src/styles/globals.css"),
];
