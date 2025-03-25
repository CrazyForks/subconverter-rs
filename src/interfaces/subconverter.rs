use crate::generator::config::formats::{
    loon::proxy_to_loon, mellow::proxy_to_mellow, quan::proxy_to_quan, quanx::proxy_to_quanx,
    singbox::proxy_to_singbox, ss_sub::proxy_to_ss_sub, surge::proxy_to_surge,
};
use crate::generator::exports::clash::proxy_to_clash;
use crate::models::ruleset::RulesetConfigs;
use crate::models::{
    ExtraSettings, Proxy, ProxyGroupConfigs, RegexMatchConfig, RulesetContent, SubconverterTarget,
};
use crate::parser::parse_settings::ParseSettings;
use crate::parser::subparser::add_nodes;
use crate::utils::http::parse_proxy;
use crate::utils::{file_get, web_get};
use crate::Settings;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct RuleBases {
    // Rule bases
    pub clash_rule_base: String,
    pub surge_rule_base: String,
    pub surfboard_rule_base: String,
    pub mellow_rule_base: String,
    pub quan_rule_base: String,
    pub quanx_rule_base: String,
    pub loon_rule_base: String,
    pub sssub_rule_base: String,
    pub singbox_rule_base: String,
}

/// Configuration for subconverter
#[derive(Debug, Clone)]
pub struct SubconverterConfig {
    /// Target conversion format
    pub target: SubconverterTarget,
    /// URLs to parse
    pub urls: Vec<String>,
    /// URLs to insert
    pub insert_urls: Vec<String>,
    /// Whether to prepend inserted nodes
    pub prepend_insert: bool,
    /// Custom group name
    pub group_name: Option<String>,
    /// Base configuration content for the target format
    pub base_content: HashMap<SubconverterTarget, String>,
    // Ruleset configs
    pub ruleset_configs: RulesetConfigs,
    /// Custom proxy groups
    pub proxy_groups: ProxyGroupConfigs,
    /// Include nodes matching these remarks
    pub include_remarks: Vec<String>,
    /// Exclude nodes matching these remarks
    pub exclude_remarks: Vec<String>,
    /// Additional settings
    pub extra: ExtraSettings,
    /// Device ID for certain formats
    pub device_id: Option<String>,
    /// Filename for download
    pub filename: Option<String>,
    /// Update interval in seconds
    pub update_interval: u32,
    /// Filter script
    pub filter_script: Option<String>,
    /// Whether update is strict
    pub update_strict: bool,
    /// Managed config prefix
    pub managed_config_prefix: String,
    /// Upload path
    pub upload_path: Option<String>,
    /// Whether to upload the result
    pub upload: bool,
    /// Proxy for fetching subscriptions
    pub proxy: Option<String>,
    /// Authentication token
    pub token: Option<String>,
    /// Whether this request is authorized
    pub authorized: bool,
    /// Subscription information
    pub sub_info: Option<String>,
    /// Rule bases
    pub rule_bases: RuleBases,
}

/// Builder for SubconverterConfig
#[derive(Debug, Clone)]
pub struct SubconverterConfigBuilder {
    config: SubconverterConfig,
}

impl Default for SubconverterConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubconverterConfigBuilder {
    /// Create a new default builder
    pub fn new() -> Self {
        SubconverterConfigBuilder {
            config: SubconverterConfig {
                target: SubconverterTarget::Clash,
                urls: Vec::new(),
                insert_urls: Vec::new(),
                prepend_insert: false,
                group_name: None,
                base_content: HashMap::new(),
                ruleset_configs: Vec::new(),
                proxy_groups: Vec::new(),
                include_remarks: Vec::new(),
                exclude_remarks: Vec::new(),
                extra: ExtraSettings::default(),
                device_id: None,
                filename: None,
                update_interval: 86400, // 24 hours
                filter_script: None,
                update_strict: false,
                managed_config_prefix: String::new(),
                upload_path: None,
                upload: false,
                proxy: None,
                token: None,
                authorized: false,
                sub_info: None,
                rule_bases: RuleBases::default(),
            },
        }
    }

    /// Set the target format
    pub fn target(&mut self, target: SubconverterTarget) -> &mut Self {
        self.config.target = target;
        self
    }

    /// Set target from string
    pub fn target_from_str(&mut self, target: &str) -> &mut Self {
        if let Some(t) = SubconverterTarget::from_str(target) {
            self.config.target = t;
        }
        self
    }

    /// Set Surge version if target is Surge
    pub fn surge_version(&mut self, version: i32) -> &mut Self {
        if let SubconverterTarget::Surge(_) = self.config.target {
            self.config.target = SubconverterTarget::Surge(version);
        }
        self
    }

    /// Add a URL to parse
    pub fn add_url(&mut self, url: &str) -> &mut Self {
        self.config.urls.push(url.to_string());
        self
    }

    /// Set URLs to parse
    pub fn urls(&mut self, urls: Vec<String>) -> &mut Self {
        self.config.urls = urls;
        self
    }

    /// Set URLs from pipe-separated string
    pub fn urls_from_str(&mut self, urls: &str) -> &mut Self {
        self.config.urls = urls.split('|').map(|s| s.trim().to_string()).collect();
        self
    }

    /// Add an insert URL
    pub fn add_insert_url(&mut self, url: &str) -> &mut Self {
        self.config.insert_urls.push(url.to_string());
        self
    }

    /// Set insert URLs
    pub fn insert_urls(&mut self, urls: Vec<String>) -> &mut Self {
        self.config.insert_urls = urls;
        self
    }

    /// Set insert URLs from pipe-separated string
    pub fn insert_urls_from_str(&mut self, urls: &str) -> &mut Self {
        self.config.insert_urls = urls.split('|').map(|s| s.trim().to_string()).collect();
        self
    }

    /// Set whether to prepend inserted nodes
    pub fn prepend_insert(&mut self, prepend: bool) -> &mut Self {
        self.config.prepend_insert = prepend;
        self
    }

    /// Set custom group name
    pub fn group_name(&mut self, name: Option<String>) -> &mut Self {
        self.config.group_name = name;
        self
    }

    /// Set proxy groups
    pub fn proxy_groups(&mut self, groups: ProxyGroupConfigs) -> &mut Self {
        self.config.proxy_groups = groups;
        self
    }

    pub fn ruleset_configs(&mut self, configs: RulesetConfigs) -> &mut Self {
        self.config.ruleset_configs = configs;
        self
    }

    /// Add an include remark pattern
    pub fn add_include_remark(&mut self, pattern: &str) -> &mut Self {
        self.config.include_remarks.push(pattern.to_string());
        self
    }

    /// Set include remark patterns
    pub fn include_remarks(&mut self, patterns: Vec<String>) -> &mut Self {
        self.config.include_remarks = patterns;
        self
    }

    /// Add an exclude remark pattern
    pub fn add_exclude_remark(&mut self, pattern: &str) -> &mut Self {
        self.config.exclude_remarks.push(pattern.to_string());
        self
    }

    /// Set exclude remark patterns
    pub fn exclude_remarks(&mut self, patterns: Vec<String>) -> &mut Self {
        self.config.exclude_remarks = patterns;
        self
    }

    pub fn emoji_array(&mut self, patterns: Vec<RegexMatchConfig>) -> &mut Self {
        self.config.extra.emoji_array = patterns;
        self
    }

    pub fn rename_array(&mut self, patterns: Vec<RegexMatchConfig>) -> &mut Self {
        self.config.extra.rename_array = patterns;
        self
    }

    pub fn add_emoji(&mut self, add: bool) -> &mut Self {
        self.config.extra.add_emoji = add;
        self
    }

    pub fn remove_emoji(&mut self, remove: bool) -> &mut Self {
        self.config.extra.remove_emoji = remove;
        self
    }

    /// Set extra settings
    pub fn extra(&mut self, extra: ExtraSettings) -> &mut Self {
        self.config.extra = extra;
        self
    }

    /// Set whether to append proxy type to remarks
    pub fn append_proxy_type(&mut self, append: bool) -> &mut Self {
        self.config.extra.append_proxy_type = append;
        self
    }

    /// Set whether to enable TCP Fast Open
    pub fn tfo(&mut self, tfo: Option<bool>) -> &mut Self {
        self.config.extra.tfo = tfo;
        self
    }

    /// Set whether to enable UDP
    pub fn udp(&mut self, udp: Option<bool>) -> &mut Self {
        self.config.extra.udp = udp;
        self
    }

    /// Set whether to skip certificate verification
    pub fn skip_cert_verify(&mut self, skip: Option<bool>) -> &mut Self {
        self.config.extra.skip_cert_verify = skip;
        self
    }

    /// Set whether to enable TLS 1.3
    pub fn tls13(&mut self, tls13: Option<bool>) -> &mut Self {
        self.config.extra.tls13 = tls13;
        self
    }

    /// Set whether to sort nodes
    pub fn sort(&mut self, sort: bool) -> &mut Self {
        self.config.extra.sort_flag = sort;
        self
    }

    /// Set sort script
    pub fn sort_script(&mut self, script: String) -> &mut Self {
        self.config.extra.sort_script = script;
        self
    }

    /// Set whether to filter deprecated nodes
    pub fn filter_deprecated(&mut self, filter: bool) -> &mut Self {
        self.config.extra.filter_deprecated = filter;
        self
    }

    /// Set whether to use new field names in Clash
    pub fn clash_new_field_name(&mut self, new_field: bool) -> &mut Self {
        self.config.extra.clash_new_field_name = new_field;
        self
    }

    /// Set whether to enable Clash script
    pub fn clash_script(&mut self, enable: bool) -> &mut Self {
        self.config.extra.clash_script = enable;
        self
    }

    pub fn clash_classical_ruleset(&mut self, enable: bool) -> &mut Self {
        self.config.extra.clash_classical_ruleset = enable;
        self
    }

    /// Set whether to generate node list
    pub fn nodelist(&mut self, nodelist: bool) -> &mut Self {
        self.config.extra.nodelist = nodelist;
        self
    }

    /// Set whether to enable rule generator
    pub fn enable_rule_generator(&mut self, enable: bool) -> &mut Self {
        self.config.extra.enable_rule_generator = enable;
        self
    }

    /// Set whether to overwrite original rules
    pub fn overwrite_original_rules(&mut self, overwrite: bool) -> &mut Self {
        self.config.extra.overwrite_original_rules = overwrite;
        self
    }

    /// Set device ID
    pub fn device_id(&mut self, device_id: Option<String>) -> &mut Self {
        self.config.device_id = device_id;
        self
    }

    /// Set filename
    pub fn filename(&mut self, filename: Option<String>) -> &mut Self {
        self.config.filename = filename;
        self
    }

    /// Set update interval
    pub fn update_interval(&mut self, interval: u32) -> &mut Self {
        self.config.update_interval = interval;
        self
    }

    /// Set filter script
    pub fn filter_script(&mut self, script: Option<String>) -> &mut Self {
        self.config.filter_script = script;
        self
    }

    /// Set whether update is strict
    pub fn update_strict(&mut self, strict: bool) -> &mut Self {
        self.config.update_strict = strict;
        self
    }

    /// Set managed config prefix
    pub fn managed_config_prefix(&mut self, prefix: String) -> &mut Self {
        self.config.managed_config_prefix = prefix;
        self
    }

    /// Set upload path
    pub fn upload_path(&mut self, path: Option<String>) -> &mut Self {
        self.config.upload_path = path;
        self
    }

    /// Set whether to upload the result
    pub fn upload(&mut self, upload: bool) -> &mut Self {
        self.config.upload = upload;
        self
    }

    /// Set proxy for fetching subscriptions
    pub fn proxy(&mut self, proxy: Option<String>) -> &mut Self {
        self.config.proxy = proxy;
        self
    }

    /// Set authentication token
    pub fn token(&mut self, token: Option<String>) -> &mut Self {
        self.config.token = token;
        self
    }

    /// Set whether this request is authorized
    pub fn authorized(&mut self, authorized: bool) -> &mut Self {
        self.config.authorized = authorized;
        self
    }

    /// Set subscription information
    pub fn sub_info(&mut self, sub_info: Option<String>) -> &mut Self {
        self.config.sub_info = sub_info;
        self
    }
    /// rule bases updates
    pub fn rule_bases(&mut self, rule_bases: RuleBases) -> &mut Self {
        self.config.rule_bases = rule_bases;
        self
    }

    /// Set rule base for Clash
    pub fn clash_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.clash_rule_base = path.to_string();
        self
    }

    /// Set rule base for Surge
    pub fn surge_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.surge_rule_base = path.to_string();
        self
    }

    /// Set rule base for Surfboard
    pub fn surfboard_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.surfboard_rule_base = path.to_string();
        self
    }

    /// Set rule base for Mellow
    pub fn mellow_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.mellow_rule_base = path.to_string();
        self
    }

    /// Set rule base for Quantumult
    pub fn quan_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.quan_rule_base = path.to_string();
        self
    }

    /// Set rule base for QuantumultX
    pub fn quanx_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.quanx_rule_base = path.to_string();
        self
    }

    /// Set rule base for Loon
    pub fn loon_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.loon_rule_base = path.to_string();
        self
    }

    /// Set rule base for SS Subscription
    pub fn sssub_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.sssub_rule_base = path.to_string();
        self
    }

    /// Set rule base for SingBox
    pub fn singbox_rule_base(&mut self, path: &str) -> &mut Self {
        self.config.rule_bases.singbox_rule_base = path.to_string();
        self
    }

    /// Build the final configuration
    pub fn build(self) -> Result<SubconverterConfig, String> {
        let config = self.config;

        // Basic validation
        if config.urls.is_empty() && config.insert_urls.is_empty() {
            return Err("No URLs provided".to_string());
        }

        Ok(config)
    }
}

/// Result of subscription conversion
#[derive(Debug, Clone)]
pub struct SubconverterResult {
    /// Converted content
    pub content: String,
    /// Response headers
    pub headers: HashMap<String, String>,
}

/// Options for parsing subscriptions
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Remarks to include in parsing
    pub include_remarks: Vec<String>,

    /// Remarks to exclude from parsing
    pub exclude_remarks: Vec<String>,

    /// Whether the request is authorized
    pub authorized: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            include_remarks: Vec::new(),
            exclude_remarks: Vec::new(),
            authorized: false,
        }
    }
}

/// Parse a subscription URL and return a vector of proxies
///
/// # Arguments
/// * `url` - The subscription URL to parse
/// * `options` - Options for parsing
///
/// # Returns
/// * `Ok(Vec<Proxy>)` - The parsed proxies
/// * `Err(String)` - Error message if parsing fails
pub fn parse_subscription(url: &str, options: ParseOptions) -> Result<Vec<Proxy>, String> {
    // Create a new parse settings instance
    let mut parse_settings = ParseSettings::default();

    // Set options from the provided config
    if !options.include_remarks.is_empty() {
        parse_settings.include_remarks = Some(options.include_remarks.clone());
    }

    if !options.exclude_remarks.is_empty() {
        parse_settings.exclude_remarks = Some(options.exclude_remarks.clone());
    }

    parse_settings.authorized = options.authorized;

    // Create a vector to hold the nodes
    let mut nodes = Vec::new();

    // Call add_nodes to do the actual parsing
    // We use group_id = 0 since we don't care about it in this context
    add_nodes(url.to_string(), &mut nodes, 0, &mut parse_settings)?;

    Ok(nodes)
}

/// Process a subscription conversion request
pub fn subconverter(config: SubconverterConfig) -> Result<SubconverterResult, String> {
    let mut response_headers = HashMap::new();
    let mut nodes = Vec::new();
    let global = Settings::current();

    info!(
        "Processing subscription conversion request to {}",
        config.target.to_str()
    );
    // Load rule base content
    let base_content = config.rule_bases.load_content();
    let mut config = config;
    config.base_content = base_content;

    // Parse subscription URLs
    let opts = ParseOptions {
        include_remarks: config.include_remarks.clone(),
        exclude_remarks: config.exclude_remarks.clone(),
        authorized: config.authorized,
    };

    // Parse insert URLs first if needed
    let mut insert_nodes = Vec::new();
    if !config.insert_urls.is_empty() {
        info!("Fetching node data from insert URLs");
        for url in &config.insert_urls {
            debug!("Parsing insert URL: {}", url);
            match parse_subscription(url, opts.clone()) {
                Ok(mut parsed_nodes) => {
                    info!("Found {} nodes from insert URL", parsed_nodes.len());
                    insert_nodes.append(&mut parsed_nodes);
                }
                Err(e) => {
                    warn!("Failed to parse insert URL '{}': {}", url, e);
                    // Continue with other URLs even if this one failed
                }
            }
        }
    }

    // Parse main URLs
    info!("Fetching node data from main URLs");
    for url in &config.urls {
        debug!("Parsing URL: {}", url);
        match parse_subscription(url, opts.clone()) {
            Ok(mut parsed_nodes) => {
                info!("Found {} nodes from URL", parsed_nodes.len());
                nodes.append(&mut parsed_nodes);
            }
            Err(e) => {
                error!("Failed to parse URL '{}': {}", url, e);
                return Err(format!("Failed to parse URL '{}': {}", url, e));
            }
        }
    }

    // Exit if found nothing
    if nodes.is_empty() && insert_nodes.is_empty() {
        return Err("No nodes were found!".to_string());
    }

    // Merge insert nodes and main nodes
    if config.prepend_insert {
        // Prepend insert nodes
        info!(
            "Prepending {} insert nodes to {} main nodes",
            insert_nodes.len(),
            nodes.len()
        );
        let mut combined = insert_nodes;
        combined.append(&mut nodes);
        nodes = combined;
    } else {
        // Append insert nodes
        info!(
            "Appending {} insert nodes to {} main nodes",
            insert_nodes.len(),
            nodes.len()
        );
        nodes.append(&mut insert_nodes);
    }

    // Apply group name if specified
    if let Some(group_name) = &config.group_name {
        info!("Setting group name to '{}'", group_name);
        for node in &mut nodes {
            (*node).group = group_name.clone();
        }
    }

    // Apply filter script if available
    // if let Some(script) = &config.filter_script {
    //     info!("Applying filter script");
    //     // In the real implementation, this would involve running a JavaScript engine
    //     // to filter nodes based on the script. Left as placeholder.
    // }

    // Process nodes (rename, emoji, sort, etc.)
    preprocess_nodes(
        &mut nodes,
        &config.extra,
        &config.extra.rename_array,
        &config.extra.emoji_array,
    );

    // Pass subscription info if provided
    if let Some(sub_info) = &config.sub_info {
        response_headers.insert("Subscription-UserInfo".to_string(), sub_info.clone());
    }

    let mut ruleset_content;
    if config.ruleset_configs == global.custom_rulesets {
        ruleset_content = global.rulesets_content.clone();
    } else {
        ruleset_content = vec![];
    }

    // Generate output based on target
    let output_content = match &config.target {
        SubconverterTarget::Clash => {
            info!("Generate target: Clash");
            let base = config
                .base_content
                .get(&SubconverterTarget::Clash)
                .cloned()
                .unwrap_or_default();
            proxy_to_clash(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                false,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::ClashR => {
            info!("Generate target: ClashR");
            let base = config
                .base_content
                .get(&SubconverterTarget::ClashR)
                .cloned()
                .unwrap_or_default();
            proxy_to_clash(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                true,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::Surge(ver) => {
            info!("Generate target: Surge {}", ver);
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            let output = proxy_to_surge(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                *ver,
                &mut config.extra.clone(),
            );

            // Add managed configuration header if needed
            if !config.managed_config_prefix.is_empty() && config.extra.enable_rule_generator {
                let managed_url = format!(
                    "{}sub?target=surge&ver={}&url={}",
                    config.managed_config_prefix,
                    ver,
                    // URL would need to be properly encoded
                    config.urls.join("|")
                );

                format!(
                    "#!MANAGED-CONFIG {} interval={} strict={}\n\n{}",
                    managed_url,
                    config.update_interval,
                    if config.update_strict {
                        "true"
                    } else {
                        "false"
                    },
                    output
                )
            } else {
                output
            }
        }
        SubconverterTarget::Surfboard => {
            info!("Generate target: Surfboard");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            let output = proxy_to_surge(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                -3, // Special version for Surfboard
                &mut config.extra.clone(),
            );

            // Add managed configuration header if needed
            if !config.managed_config_prefix.is_empty() && config.extra.enable_rule_generator {
                let managed_url = format!(
                    "{}sub?target=surfboard&url={}",
                    config.managed_config_prefix,
                    // URL would need to be properly encoded
                    config.urls.join("|")
                );

                format!(
                    "#!MANAGED-CONFIG {} interval={} strict={}\n\n{}",
                    managed_url,
                    config.update_interval,
                    if config.update_strict {
                        "true"
                    } else {
                        "false"
                    },
                    output
                )
            } else {
                output
            }
        }
        SubconverterTarget::Mellow => {
            info!("Generate target: Mellow");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_mellow(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::SSSub => {
            info!("Generate target: SS Subscription");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_ss_sub(&base, &mut nodes, &mut config.extra.clone())
        }
        SubconverterTarget::SS => {
            info!("Generate target: SS");
            // To be implemented: convert nodes to SS format
            String::new() // placeholder
        }
        SubconverterTarget::SSR => {
            info!("Generate target: SSR");
            // To be implemented: convert nodes to SSR format
            String::new() // placeholder
        }
        SubconverterTarget::V2Ray => {
            info!("Generate target: V2Ray");
            // To be implemented: convert nodes to V2Ray format
            String::new() // placeholder
        }
        SubconverterTarget::Trojan => {
            info!("Generate target: Trojan");
            // To be implemented: convert nodes to Trojan format
            String::new() // placeholder
        }
        SubconverterTarget::Mixed => {
            info!("Generate target: Mixed");
            // To be implemented: convert nodes to mixed format
            String::new() // placeholder
        }
        SubconverterTarget::Quantumult => {
            info!("Generate target: Quantumult");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_quan(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::QuantumultX => {
            info!("Generate target: Quantumult X");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_quanx(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::Loon => {
            info!("Generate target: Loon");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_loon(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::SSD => {
            info!("Generate target: SSD");
            // To be implemented: convert nodes to SSD format
            String::new() // placeholder
        }
        SubconverterTarget::SingBox => {
            info!("Generate target: SingBox");
            let base = config
                .base_content
                .get(&config.target)
                .cloned()
                .unwrap_or_default();
            proxy_to_singbox(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                &mut config.extra.clone(),
            )
        }
        SubconverterTarget::Auto => {
            // When target is Auto, we should have decided on a specific target earlier based on user agent
            // If we still have Auto at this point, default to Clash
            info!("Generate target: Auto (defaulting to Clash)");
            let base = config
                .base_content
                .get(&SubconverterTarget::Clash)
                .cloned()
                .unwrap_or_default();
            proxy_to_clash(
                &mut nodes,
                &base,
                &mut ruleset_content,
                &config.proxy_groups,
                false,
                &mut config.extra.clone(),
            )
        }
    };

    // Set filename header if provided
    if let Some(filename) = &config.filename {
        response_headers.insert(
            "Content-Disposition".to_string(),
            format!("attachment; filename=\"{}\"; filename*=utf-8''", filename),
        );
    }

    // Upload result if needed
    if config.upload {
        if let Some(upload_path) = &config.upload_path {
            info!("Uploading result to path: {}", upload_path);
            // Implement upload functionality here
            // This is typically a separate function like `upload_gist`
        }
    }

    info!("Conversion completed");
    Ok(SubconverterResult {
        content: output_content,
        headers: response_headers,
    })
}

/// Preprocess nodes before conversion
pub fn preprocess_nodes(
    nodes: &mut Vec<Proxy>,
    extra: &ExtraSettings,
    rename_patterns: &Vec<RegexMatchConfig>,
    emoji_patterns: &Vec<RegexMatchConfig>,
) {
    // Apply renames
    if !rename_patterns.is_empty() {
        info!(
            "Applying {} rename patterns to {} nodes",
            rename_patterns.len(),
            nodes.len()
        );
        for node in nodes.iter_mut() {
            for pattern in rename_patterns {
                pattern.process(&mut node.remark);
            }
        }
    }

    // Apply emojis
    if extra.add_emoji && !emoji_patterns.is_empty() {
        info!("Applying emoji patterns to {} nodes", nodes.len());
        for node in nodes.iter_mut() {
            // Remove existing emoji if needed
            if extra.remove_emoji {
                // Simplified emoji removal; actual implementation would use regex
                // to remove emoji patterns
            }

            // Add emoji based on patterns
            for pattern in emoji_patterns {
                pattern.process(&mut node.remark);
            }
        }
    }

    // Sort nodes if needed
    if extra.sort_flag {
        info!("Sorting {} nodes", nodes.len());
        if !extra.sort_script.is_empty() {
            // Apply sort using script
            // This would involve running a JavaScript engine
        } else {
            // Simple default sort by remark
            nodes.sort_by(|a, b| a.remark.cmp(&b.remark));
        }
    }
}

impl RuleBases {
    /// Load rule base content from files or URLs
    pub fn load_content(&self) -> HashMap<SubconverterTarget, String> {
        let mut base_content = HashMap::new();

        let global = Settings::current();
        let proxy_config = parse_proxy(&global.proxy_config);

        // Helper function to load content from file or URL
        let load_content = |path: &str| -> Option<String> {
            if path.is_empty() {
                return None;
            }

            // Check if path is a URL
            if path.starts_with("http://") || path.starts_with("https://") {
                match web_get(path, &proxy_config, None) {
                    Ok((content, _)) => {
                        debug!("Loaded rule base from URL: {}", path);
                        Some(content)
                    }
                    Err(e) => {
                        warn!("Failed to load rule base from URL {}: {}", path, e);
                        None
                    }
                }
            } else {
                // Treat as file path
                match file_get(path, None) {
                    Ok(content) => {
                        debug!("Loaded rule base from file: {}", path);
                        Some(content)
                    }
                    Err(e) => {
                        warn!("Failed to load rule base from file {}: {}", path, e);
                        None
                    }
                }
            }
        };

        // Load rule bases for each target format
        if let Some(content) = load_content(&self.clash_rule_base) {
            base_content.insert(SubconverterTarget::Clash, content.clone());
            base_content.insert(SubconverterTarget::ClashR, content);
        }

        if let Some(content) = load_content(&self.surge_rule_base) {
            base_content.insert(SubconverterTarget::Surge(3), content.clone());
            base_content.insert(SubconverterTarget::Surge(4), content);
        }

        if let Some(content) = load_content(&self.surfboard_rule_base) {
            base_content.insert(SubconverterTarget::Surfboard, content);
        }

        if let Some(content) = load_content(&self.mellow_rule_base) {
            base_content.insert(SubconverterTarget::Mellow, content);
        }

        if let Some(content) = load_content(&self.quan_rule_base) {
            base_content.insert(SubconverterTarget::Quantumult, content);
        }

        if let Some(content) = load_content(&self.quanx_rule_base) {
            base_content.insert(SubconverterTarget::QuantumultX, content);
        }

        if let Some(content) = load_content(&self.loon_rule_base) {
            base_content.insert(SubconverterTarget::Loon, content);
        }

        if let Some(content) = load_content(&self.sssub_rule_base) {
            base_content.insert(SubconverterTarget::SSSub, content);
        }

        if let Some(content) = load_content(&self.singbox_rule_base) {
            base_content.insert(SubconverterTarget::SingBox, content);
        }

        base_content
    }
}
