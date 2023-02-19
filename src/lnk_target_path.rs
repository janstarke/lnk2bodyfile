use std::fmt::Display;

use anyhow::bail;
use lnk_parser::LNKParser;
use winparsingtools::structs::shell_items::Name;

/// refer to <https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-SHLLINK/%5bMS-SHLLINK%5d.pdf>
pub struct LnkTargetPath(String);

impl LnkTargetPath {
    fn path_from_target_list(parser: &LNKParser) -> Option<Self> {
        if let Some(id_list) = parser.get_link_target_id_list() {
            let path = id_list
                .items()
                .filter(|si| si.class_type & 0x70 != 0x10)
                .map(|shell_item| shell_item.name())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("\\")
                .replace("\\\\", "\\");
            Some(Self(path))
        } else {
            None
        }
    }
}

impl TryFrom<&LNKParser> for LnkTargetPath {
    type Error = anyhow::Error;

    fn try_from(parser: &LNKParser) -> Result<Self, Self::Error> {
        // try to generate path from link_info structure
        if let Some(link_info) = parser.get_link_info() {
            assert_ne!(
                link_info.flags.get_CommonNetworkRelativeLinkAndPathSuffix(),
                link_info.flags.get_VolumeIDAndLocalBasePath()
            );

            if *link_info.flags.get_VolumeIDAndLocalBasePath() {
                let local_basepath = match link_info.get_local_base_path() {
                    None => bail!("missing local base path"),
                    Some(path) => path,
                };
                match link_info.get_common_path_suffix() {
                    None => Ok(Self(local_basepath.to_string())),
                    Some(s) => Ok(Self(format!("{local_basepath}\\{s}"))),
                }
            } else {
                let relative_link = match link_info.get_common_network_relative_link() {
                    None => bail!("missing common network relative link"),
                    Some(link) => link,
                };

                match link_info.get_common_path_suffix() {
                    None => {
                        if let Some(path) = Self::path_from_target_list(parser) {
                            Ok(path)
                        } else {
                            Ok(Self(
                                relative_link.get_net_name().as_ref().unwrap().to_string(),
                            ))
                        }
                    }
                    Some(s) => Ok(Self(format!(
                        "{}\\{s}",
                        relative_link.get_net_name().as_ref().unwrap()
                    ))),
                }
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
