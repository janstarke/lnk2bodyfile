use std::fmt::Display;

use anyhow::bail;
use lnk_parser::{LNKParser, link_info};


/// refer to <https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-SHLLINK/%5bMS-SHLLINK%5d.pdf>
pub struct LnkTargetPath(String);

impl TryFrom<&LNKParser> for LnkTargetPath {
    type Error = anyhow::Error;

    fn try_from(parser: &LNKParser) -> Result<Self, Self::Error> {
        if let Some(link_info) = parser.get_link_info() {
            let common_path_suffix = match link_info.get_common_path_suffix() {
                Some(s) => s,
                None => bail!("missing common path suffix"),
            };

            if *link_info.flags.get_VolumeIDAndLocalBasePath() {
                let local_basepath = match link_info.get_local_base_path() {
                    None => bail!("missing local base path"),
                    Some(path) => path,
                };
                Ok(Self(
                    format!("{local_basepath}\\{common_path_suffix}")
                ))
            } else {
                let relative_link = match link_info.get_common_network_relative_link() {
                    None => bail!("missing common network relative link"),
                    Some(link) => link
                };
                Ok(Self(
                    format!("{}\\{common_path_suffix}", relative_link.get_net_name().as_ref().unwrap())
                ))
            }
        } else {
            bail!("missing link info")
        }
    }
}

impl From<LnkTargetPath> for String {
    fn from(value: LnkTargetPath) -> Self {
        value.0
    }
}

impl AsRef<str> for LnkTargetPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for LnkTargetPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}