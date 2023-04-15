use serde::Deserialize;
use clap::Parser;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct UdcNode {
    pub name: String,
    pub dns: Vec<String>,
    pub domains: Vec<String>,
}

impl Display for UdcNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        assert!(self.dns.len() > 0);
        assert!(self.domains.len() > 0);

        let dns = self.dns.iter()
            .map(|dns| format!("    forward-addr: {dns}"))
            .chain(vec![
                "    forward-tls-upstream: yes".to_owned(),
            ].into_iter())
            .collect::<Vec<String>>()
            .join("\n");

        let result = self.domains.iter()
            .map(|domain| format!("forward-zone:\n    name: '{}'\n{}", domain, &dns))
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{result}")
    }
}

#[derive(Debug, Deserialize)]
pub struct Udc (
    pub HashMap<String, Vec<UdcNode>>,
);


#[cfg(test)]
mod udc_node_tests {
    use super::UdcNode;

    #[test]
    fn test_one_line() {
        let node = UdcNode {
            name: "test".to_owned(),
            dns: vec![
                "1.1.1.1".to_owned(),
            ],
            domains: vec![
                ".".to_owned(),
            ],
        };
        assert_eq!(node.to_string(), "\
forward-zone:
    name: '.'
    forward-addr: 1.1.1.1
    forward-tls-upstream: yes");
    }

    #[test]
    fn test_two_lines() {
        let node = UdcNode {
            name: "test".to_owned(),
            dns: vec![
                "1.1.1.1".to_owned(),
            ],
            domains: vec![
                ".".to_owned(),
                ".com".to_owned(),
            ],
        };
        assert_eq!(node.to_string(), "\
forward-zone:
    name: '.'
    forward-addr: 1.1.1.1
    forward-tls-upstream: yes

forward-zone:
    name: '.com'
    forward-addr: 1.1.1.1
    forward-tls-upstream: yes");
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// TOML style unbound configuration file
    pub conf: PathBuf,

    /// Generate the unbound configuration file into
    #[arg(short, long, value_name = "DIRECTORY")]
    pub out: PathBuf,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
