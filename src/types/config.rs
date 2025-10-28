use std::fs;
use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LogFileConfig {
    pub level: Option<String>, // e.g., "info", "warn"
    pub color: Option<bool>,   // true/false; None by omission
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GeneralFileConfig {
    pub db: Option<String>,
    pub ignore_targets: Option<Vec<String>>, // glob patterns
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MutationsFileConfig {
    pub slugs: Option<Vec<String>>, // global whitelist of mutation slugs
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TestFileConfig {
    pub cmd: Option<String>,
    pub timeout: Option<u32>,
    pub per_target: Option<Vec<PerTargetTestFileConfig>>, // ordered, first match wins
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FileConfig {
    pub log: Option<LogFileConfig>,
    pub general: Option<GeneralFileConfig>,
    pub mutations: Option<MutationsFileConfig>,
    pub test: Option<TestFileConfig>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LogConfig {
    pub level: String,       // resolved level; default "info"
    pub color: Option<bool>, // Some(true)=force on, Some(false)=force off, None=auto
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GeneralConfig {
    pub db: String,                  // resolved db path; default "muton.sqlite"
    pub ignore_targets: Vec<String>, // merged globs
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MutationsConfig {
    pub slugs: Option<Vec<String>>, // highest-priority non-empty overrides
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TestConfig {
    pub cmd: String,                        // resolved; default "npx blueprint test"
    pub timeout: Option<u32>,               // seconds
    pub per_target: Vec<PerTargetTestRule>, // ordered, first match wins
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GlobalConfig {
    pub general: GeneralConfig,
    pub mutations: MutationsConfig,
    pub test: TestConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PerTargetTestFileConfig {
    pub glob: String,
    pub cmd: Option<String>,
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PerTargetTestRule {
    pub glob: String,
    pub cmd: String,
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    pub db: Option<String>,
    pub log_level: Option<String>,
    pub log_color: Option<String>,       // "on" | "off"
    pub ignore_targets: Option<String>,  // csv
    pub mutations_slugs: Option<String>, // csv
    pub test_cmd: Option<String>,
    pub test_timeout: Option<u32>,
}

static CONFIG: OnceCell<GlobalConfig> = OnceCell::new();

pub fn config() -> &'static GlobalConfig {
    CONFIG.get_or_init(|| {
        let mut cfg = default_global_config();
        // Apply nearest config file found by walking up from cwd, then env
        if let Some(path) = find_nearest_config_file() {
            if let Some(file_cfg) = read_config_file(&path) {
                apply_file_config(&mut cfg, &file_cfg);
            }
        }
        apply_env_overrides(&mut cfg);
        cfg
    })
}

pub fn init_with_overrides(overrides: &CliOverrides) {
    let mut cfg = default_global_config();

    // 1) Config file: walk up from cwd and use the first muton.toml found
    if let Some(path) = find_nearest_config_file() {
        if let Some(file_cfg) = read_config_file(&path) {
            apply_file_config(&mut cfg, &file_cfg);
        }
    }

    // 2) Environment variables
    apply_env_overrides(&mut cfg);

    // 3) CLI arguments (highest priority). Only override if user specified.
    apply_cli_overrides(&mut cfg, overrides);

    let _ = CONFIG.set(cfg);
}

pub fn default_global_config() -> GlobalConfig {
    GlobalConfig {
        general: GeneralConfig {
            db: "muton.sqlite".to_string(),
            ignore_targets: Vec::new(),
        },
        mutations: MutationsConfig { slugs: None },
        test: TestConfig {
            cmd: "npx blueprint test".to_string(),
            timeout: None,
            per_target: Vec::new(),
        },
        log: LogConfig {
            level: "info".to_string(),
            color: None,
        },
    }
}

fn read_config_file(path: &Path) -> Option<FileConfig> {
    match fs::read_to_string(path) {
        Ok(contents) => toml::from_str::<FileConfig>(&contents).ok(),
        Err(_) => None,
    }
}

fn apply_file_config(cfg: &mut GlobalConfig, file: &FileConfig) {
    if let Some(log) = &file.log {
        if let Some(level) = &log.level {
            cfg.log.level = level.clone();
        }
        if let Some(color) = log.color {
            cfg.log.color = Some(color);
        }
    }
    if let Some(r#gen) = &file.general {
        if let Some(db) = &r#gen.db {
            cfg.general.db = db.clone();
        }
        if let Some(globs) = &r#gen.ignore_targets {
            cfg.general.ignore_targets.extend(globs.clone());
        }
    }
    if let Some(muts) = &file.mutations
        && let Some(slugs) = &muts.slugs
        && !slugs.is_empty()
    {
        cfg.mutations.slugs = Some(slugs.clone()); // override semantics
    }
    if let Some(test) = &file.test {
        if let Some(cmd) = &test.cmd {
            cfg.test.cmd = cmd.clone();
        }
        if let Some(timeout) = test.timeout {
            cfg.test.timeout = Some(timeout);
        }
        if let Some(per) = &test.per_target {
            for rule in per {
                if let Some(cmd) = &rule.cmd
                    && !cmd.trim().is_empty()
                {
                    cfg.test.per_target.push(PerTargetTestRule {
                        glob: rule.glob.clone(),
                        cmd: cmd.clone(),
                        timeout: rule.timeout,
                    });
                }
            }
        }
    }
}

fn apply_env_overrides(cfg: &mut GlobalConfig) {
    // Logging
    if let Ok(level) = std::env::var("MUTON_LOG_LEVEL")
        && !level.trim().is_empty()
    {
        cfg.log.level = level.trim().to_string();
    }
    if let Ok(color) = std::env::var("MUTON_LOG_COLOR") {
        match color.to_lowercase().as_str() {
            "on" => cfg.log.color = Some(true),
            "off" => cfg.log.color = Some(false),
            _ => {}
        }
    }

    // General
    if let Ok(db) = std::env::var("MUTON_DB")
        && !db.trim().is_empty()
    {
        cfg.general.db = db;
    }
    if let Ok(ignore) = std::env::var("MUTON_IGNORE_TARGETS") {
        let patterns = parse_csv(&ignore);
        cfg.general.ignore_targets.extend(patterns);
    }

    // Mutations whitelist (override)
    if let Ok(slugs) = std::env::var("MUTON_SLUGS") {
        let list = parse_csv(&slugs);
        if !list.is_empty() {
            cfg.mutations.slugs = Some(list);
        }
    }

    // Test
    if let Ok(cmd) = std::env::var("MUTON_TEST_CMD")
        && !cmd.trim().is_empty()
    {
        cfg.test.cmd = cmd;
    }
    if let Ok(timeout) = std::env::var("MUTON_TEST_TIMEOUT")
        && let Ok(parsed) = timeout.trim().parse::<u32>()
    {
        cfg.test.timeout = Some(parsed);
    }
}

fn apply_cli_overrides(cfg: &mut GlobalConfig, overrides: &CliOverrides) {
    // Global overrides
    if let Some(db) = overrides.db.as_ref() {
        cfg.general.db = db.clone();
    }
    if let Some(level) = overrides.log_level.as_ref()
        && !level.trim().is_empty()
    {
        cfg.log.level = level.trim().to_string();
    }
    if let Some(color) = overrides.log_color.as_ref() {
        match color.to_lowercase().as_str() {
            "on" => cfg.log.color = Some(true),
            "off" => cfg.log.color = Some(false),
            _ => {}
        }
    }
    if let Some(ignore_csv) = overrides.ignore_targets.as_ref() {
        cfg.general.ignore_targets.extend(parse_csv(ignore_csv));
    }

    // Mutations slugs override (highest non-empty wins)
    if let Some(muts_csv) = overrides.mutations_slugs.as_ref() {
        let list = parse_csv(muts_csv);
        if !list.is_empty() {
            cfg.mutations.slugs = Some(list);
        }
    }

    // Test overrides
    if let Some(cmd) = overrides.test_cmd.as_ref()
        && !cmd.trim().is_empty()
    {
        cfg.test.cmd = cmd.clone();
    }
    if let Some(timeout) = overrides.test_timeout {
        cfg.test.timeout = Some(timeout);
    }
}

fn parse_csv(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn find_nearest_config_file() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    for dir in cwd.ancestors() {
        let candidate = dir.join("muton.toml");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

pub fn colors_enabled() -> bool {
    match config().log.color {
        Some(force) => force,
        None => console::colors_enabled(),
    }
}

pub fn is_slug_enabled(slug: &str) -> bool {
    if let Some(list) = &config().mutations.slugs {
        return list.iter().any(|s| s == slug);
    }
    true
}

pub fn is_path_excluded(path: &Path) -> bool {
    if config().general.ignore_targets.is_empty() {
        return false;
    }
    let mut builder = globset::GlobSetBuilder::new();
    for pat in &config().general.ignore_targets {
        if let Ok(glob) = globset::Glob::new(pat) {
            builder.add(glob);
        }
    }
    let Ok(set) = builder.build() else {
        return false;
    };
    set.is_match(PathBuf::from(path))
}

pub fn resolve_test_for_path_with_cli(
    path: &Path,
    cli_test_cmd: &Option<String>,
    cli_timeout: Option<u32>,
) -> (String, Option<u32>) {
    // CLI has highest precedence
    if let Some(cmd) = cli_test_cmd.as_ref()
        && !cmd.trim().is_empty()
    {
        let timeout = cli_timeout.or(config().test.timeout);
        return (cmd.clone(), timeout);
    }

    // Per-target rules: first match wins
    let path_buf = PathBuf::from(path);
    for rule in &config().test.per_target {
        if glob_matches(&rule.glob, &path_buf) {
            let timeout = cli_timeout.or(rule.timeout).or(config().test.timeout);
            return (rule.cmd.clone(), timeout);
        }
    }

    // Fallback to global
    (
        config().test.cmd.clone(),
        cli_timeout.or(config().test.timeout),
    )
}

fn glob_matches(pattern: &str, path: &Path) -> bool {
    if let Ok(glob) = globset::Glob::new(pattern) {
        let matcher = glob.compile_matcher();
        return matcher.is_match(path);
    }
    false
}
