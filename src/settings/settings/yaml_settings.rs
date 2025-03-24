use super::ini_bindings::{FromIni, FromIniWithDelimiter};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    models::{
        cron::{CronTaskConfig, CronTaskConfigs},
        ruleset::RulesetConfigs,
        ProxyGroupConfig, ProxyGroupConfigs, RegexMatchConfigs,
    },
    settings::import_items,
};

// 为serde_yml::Value添加默认值函数
fn default_yaml_value() -> serde_yml::Value {
    serde_yml::Value::String(String::new())
}

// 为常用默认值添加函数
fn default_true() -> bool {
    true
}

fn default_empty_string() -> String {
    String::new()
}

fn default_system() -> String {
    "SYSTEM".to_string()
}

fn default_none() -> String {
    "NONE".to_string()
}

fn default_listen_address() -> String {
    "127.0.0.1".to_string()
}

fn default_listen_port() -> i32 {
    25500
}

fn default_max_pending_conns() -> i32 {
    10240
}

fn default_max_concurrent_threads() -> i32 {
    4
}

fn default_info_log_level() -> String {
    "info".to_string()
}

fn default_cache_subscription() -> i32 {
    60
}

fn default_cache_config() -> i32 {
    300
}

fn default_cache_ruleset() -> i32 {
    21600
}

fn default_max_rulesets() -> usize {
    64
}

fn default_max_rules() -> usize {
    32768
}

fn default_max_download_size() -> i64 {
    32 * 1024 * 1024 // 32MB
}

/// Stream rule configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RegexMatchRuleInYaml {
    #[serde(rename = "match")]
    pub match_str: Option<String>,
    pub replace: Option<String>,
    pub script: Option<String>,
    pub import: Option<String>,
}

/// Trait for converting to INI format with a specified delimiter
pub trait ToIniWithDelimiter {
    fn to_ini_with_delimiter(&self, delimiter: &str) -> String;
}

impl ToIniWithDelimiter for RegexMatchRuleInYaml {
    fn to_ini_with_delimiter(&self, delimiter: &str) -> String {
        // Check for script first
        if let Some(script) = &self.script {
            if !script.is_empty() {
                return format!("!!script:{}", script);
            }
        }

        // Then check for import
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Finally check for match and replace
        if let (Some(match_str), Some(replace)) = (&self.match_str, &self.replace) {
            if !match_str.is_empty() && !replace.is_empty() {
                return format!("{}{}{}", match_str, delimiter, replace);
            }
        }

        // Default to empty string if nothing matches
        String::new()
    }
}

pub trait ToIni {
    fn to_ini(&self) -> String;
}

impl ToIni for RulesetConfigInYaml {
    fn to_ini(&self) -> String {
        // Check for import first
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Then check for ruleset URL
        if let Some(ruleset) = &self.ruleset {
            if !ruleset.is_empty() {
                let mut result = format!("{},{}", self.group, ruleset);
                // Add interval if provided
                if let Some(interval) = self.interval {
                    result = format!("{},{}", result, interval);
                }
                return result;
            }
        }

        // Finally check for rule
        if let Some(rule) = &self.rule {
            if !rule.is_empty() {
                return format!("{},[]{}", self.group, rule);
            }
        }

        // Default to empty string if nothing matches
        String::new()
    }
}

impl ToIni for TaskConfigInYaml {
    fn to_ini(&self) -> String {
        // Check for import first
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Otherwise join fields with backticks
        format!(
            "{}`{}`{}`{}",
            self.name, self.cronexp, self.path, self.timeout
        )
    }
}

/// User info settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct UserInfoSettings {
    pub stream_rule: Vec<RegexMatchRuleInYaml>,
    pub time_rule: Vec<RegexMatchRuleInYaml>,
}

/// Common settings section
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct CommonSettings {
    pub api_mode: bool,
    pub api_access_token: String,
    pub default_url: Vec<String>,
    #[serde(default = "default_true")]
    pub enable_insert: bool,
    pub insert_url: Vec<String>,
    #[serde(default = "default_true")]
    pub prepend_insert_url: bool,
    pub exclude_remarks: Vec<String>,
    pub include_remarks: Vec<String>,
    pub enable_filter: bool,
    pub filter_script: String,
    pub default_external_config: String,
    #[serde(default = "default_empty_string")]
    pub base_path: String,
    pub clash_rule_base: String,
    pub surge_rule_base: String,
    pub surfboard_rule_base: String,
    pub mellow_rule_base: String,
    pub quan_rule_base: String,
    pub quanx_rule_base: String,
    pub loon_rule_base: String,
    pub sssub_rule_base: String,
    pub singbox_rule_base: String,
    #[serde(default = "default_system")]
    pub proxy_config: String,
    #[serde(default = "default_system")]
    pub proxy_ruleset: String,
    #[serde(default = "default_none")]
    pub proxy_subscription: String,
    pub append_proxy_type: bool,
    pub reload_conf_on_request: bool,
}

/// Node preferences
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct NodePreferences {
    pub udp_flag: Option<bool>,
    pub tcp_fast_open_flag: Option<bool>,
    pub skip_cert_verify_flag: Option<bool>,
    pub tls13_flag: Option<bool>,
    pub sort_flag: bool,
    pub sort_script: String,
    pub filter_deprecated_nodes: bool,
    #[serde(default = "default_true")]
    pub append_sub_userinfo: bool,
    #[serde(default = "default_true")]
    pub clash_use_new_field_name: bool,
    pub clash_proxies_style: String,
    pub clash_proxy_groups_style: String,
    pub singbox_add_clash_modes: bool,
    pub rename_node: Vec<RegexMatchRuleInYaml>,
}

/// Managed config settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ManagedConfigSettings {
    #[serde(default = "default_true")]
    pub write_managed_config: bool,
    #[serde(default = "default_listen_address")]
    pub managed_config_prefix: String,
    #[serde(default = "default_update_interval")]
    pub config_update_interval: i32,
    pub config_update_strict: bool,
    pub quanx_device_id: String,
}

fn default_update_interval() -> i32 {
    86400 // 24 hours
}

/// Surge external proxy settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SurgeExternalProxySettings {
    pub surge_ssr_path: String,
    #[serde(default = "default_true")]
    pub resolve_hostname: bool,
}

/// Emoji settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EmojiSettings {
    pub add_emoji: bool,
    #[serde(default = "default_true")]
    pub remove_old_emoji: bool,
    pub rules: Vec<RegexMatchRuleInYaml>,
}

/// Ruleset configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RulesetConfigInYaml {
    pub rule: Option<String>,
    pub ruleset: Option<String>,
    pub group: String,
    pub interval: Option<i32>,
    pub import: Option<String>,
}

/// Ruleset settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RulesetSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub overwrite_original_rules: bool,
    pub update_ruleset_on_request: bool,
    #[serde(alias = "surge_ruleset")]
    pub rulesets: Vec<RulesetConfigInYaml>,
}

/// Proxy group configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct ProxyGroupConfigInYaml {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub rule: Vec<String>,
    #[serde(default = "default_test_url")]
    pub url: Option<String>,
    #[serde(default = "default_interval")]
    pub interval: Option<i32>,
    pub tolerance: Option<i32>,
    pub timeout: Option<i32>,
    pub import: Option<String>,
}

fn default_test_url() -> Option<String> {
    Some("http://www.gstatic.com/generate_204".to_string())
}

fn default_interval() -> Option<i32> {
    Some(300)
}

/// Proxy groups settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGroupsSettings {
    pub custom_proxy_group: Vec<ProxyGroupConfigInYaml>,
}

/// Template variable
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateVariable {
    pub key: String,
    #[serde(default = "default_yaml_value")]
    pub value: serde_yml::Value,
}

impl Default for TemplateVariable {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: default_yaml_value(),
        }
    }
}

/// Template settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct TemplateSettings {
    pub template_path: String,
    pub globals: Vec<TemplateVariable>,
}

/// Alias configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AliasConfig {
    pub uri: String,
    pub target: String,
}

/// Task configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct TaskConfigInYaml {
    pub name: String,
    pub cronexp: String,
    pub path: String,
    pub timeout: i32,
    pub import: Option<String>,
}

/// Server settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ServerSettings {
    #[serde(default = "default_listen_address")]
    pub listen: String,
    #[serde(default = "default_listen_port")]
    pub port: i32,
    pub serve_file_root: String,
}

/// Advanced settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AdvancedSettings {
    #[serde(default = "default_info_log_level")]
    pub log_level: String,
    pub print_debug_info: bool,
    #[serde(default = "default_max_pending_conns")]
    pub max_pending_connections: i32,
    #[serde(default = "default_max_concurrent_threads")]
    pub max_concurrent_threads: i32,
    #[serde(default = "default_max_rulesets")]
    pub max_allowed_rulesets: usize,
    #[serde(default = "default_max_rules")]
    pub max_allowed_rules: usize,
    #[serde(default = "default_max_download_size")]
    pub max_allowed_download_size: i64,
    pub enable_cache: bool,
    #[serde(default = "default_cache_subscription")]
    pub cache_subscription: i32,
    #[serde(default = "default_cache_config")]
    pub cache_config: i32,
    #[serde(default = "default_cache_ruleset")]
    pub cache_ruleset: i32,
    #[serde(default = "default_true")]
    pub script_clean_context: bool,
    pub async_fetch_ruleset: bool,
    pub skip_failed_links: bool,
}

/// Main YAML settings structure
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct YamlSettings {
    pub common: CommonSettings,
    pub userinfo: UserInfoSettings,
    pub node_pref: NodePreferences,
    pub managed_config: ManagedConfigSettings,
    pub surge_external_proxy: SurgeExternalProxySettings,
    pub emojis: EmojiSettings,

    #[serde(alias = "ruleset")]
    pub rulesets: RulesetSettings,

    #[serde(alias = "proxy_group")]
    pub proxy_groups: ProxyGroupsSettings,

    pub template: TemplateSettings,
    pub aliases: Vec<AliasConfig>,
    pub tasks: Vec<TaskConfigInYaml>,
    pub server: ServerSettings,
    pub advanced: AdvancedSettings,

    // Extra fields not in the YAML but needed for settings
    #[serde(skip)]
    pub parsed_rename: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_stream_rule: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_time_rule: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_emoji_rules: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_proxy_group: ProxyGroupConfigs,
    #[serde(skip)]
    pub parsed_ruleset: RulesetConfigs,
    #[serde(skip)]
    pub parsed_tasks: CronTaskConfigs,
}

impl YamlSettings {
    pub fn process_imports_and_inis(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        // read renames
        let mut rename_nodes = self
            .node_pref
            .rename_node
            .iter()
            .map(|rule| rule.to_ini_with_delimiter("@"))
            .collect::<Vec<String>>();

        import_items(
            &mut rename_nodes,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_rename = RegexMatchConfigs::from_ini_with_delimiter(&rename_nodes, "@");

        // read streamrule
        let mut stream_rules = self
            .userinfo
            .stream_rule
            .iter()
            .map(|rule| rule.to_ini_with_delimiter("|"))
            .collect::<Vec<String>>();

        import_items(
            &mut stream_rules,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_stream_rule = RegexMatchConfigs::from_ini_with_delimiter(&stream_rules, "|");

        // read time rule
        let mut time_rules = self
            .userinfo
            .time_rule
            .iter()
            .map(|rule| rule.to_ini_with_delimiter("|"))
            .collect::<Vec<String>>();
        import_items(
            &mut time_rules,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_time_rule = RegexMatchConfigs::from_ini_with_delimiter(&time_rules, "|");

        // read emojis
        let mut emoji_rules = self
            .emojis
            .rules
            .iter()
            .map(|rule| rule.to_ini_with_delimiter(","))
            .collect::<Vec<String>>();
        import_items(
            &mut emoji_rules,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_emoji_rules = RegexMatchConfigs::from_ini_with_delimiter(&emoji_rules, ",");

        // read rulesets
        let mut rulesets = self
            .rulesets
            .rulesets
            .iter()
            .map(|rule| rule.to_ini())
            .collect::<Vec<String>>();
        import_items(
            &mut rulesets,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_ruleset = RulesetConfigs::from_ini(&rulesets);

        // read proxy groups
        let mut proxy_groups = Vec::new();
        import_items(
            &mut proxy_groups,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;

        self.parsed_proxy_group = ProxyGroupConfigs::from_ini(&proxy_groups);

        let mut tasks = self
            .tasks
            .iter()
            .map(|task| task.to_ini())
            .collect::<Vec<String>>();
        import_items(
            &mut tasks,
            false,
            &self.common.proxy_config,
            &self.common.base_path,
        )?;
        self.parsed_tasks = CronTaskConfigs::from_ini(&tasks);
        Ok(())
    }
}
