use std::{fs::File, path::PathBuf};

use anyhow::bail;
use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use lnk_parser::{
    link_info::{self, LinkInfo, LinkInfoFlags},
    LNKParser,
};
use winparsingtools::traits::Path;

pub struct LnkFile {
    lnk_file: LNKParser,
    file_name: String,
    target_name: String,
}

impl LnkFile {
    pub fn print_bodyfile(&self, include_target_link_information: bool) {
        self.print_bodyfile_for_me();
        if include_target_link_information {
            self.print_bodyfile_for_link_info();
        }
    }

    fn print_bodyfile_for_me(&self) {
        log::warn!("target_full_path: {:?}", self.lnk_file.get_target_full_path());
        log::warn!("common_network_relative_link: {:?}", self.lnk_file.get_link_info().as_ref().unwrap().get_common_network_relative_link());
        let name = match self.lnk_file.get_target_full_path() {
            Some(s) => s.to_owned(),
            None => match self.lnk_file.get_relative_path() {
                Some(s) => format!("relative path: {}", s.string.clone()),
                None => match self.lnk_file.get_name_string() {
                    Some(s) => format!("name sring: {}", s.string.clone()),
                    None => "None".into(),
                },
            },
        };

        let link_header = self.lnk_file.get_shell_link_header();
        //println!("{:?}", self.lnk_file);
        let bfline = Bodyfile3Line::new()
            .with_name(&format!("{} (referred to by {})",self.target_name, self.file_name))
            .with_atime(DateTime::<Utc>::from(link_header.get_atime()).timestamp())
            .with_mtime(DateTime::<Utc>::from(link_header.get_ctime()).timestamp())
            .with_crtime(DateTime::<Utc>::from(link_header.get_mtime()).timestamp())
            .with_size((*link_header.get_file_size()).into());

        println!("{bfline}");
    }

    fn print_bodyfile_for_link_info(&self) {}
}

impl TryFrom<&str> for LnkFile {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let file_path = PathBuf::from(value);
        let lnk_file = LNKParser::from_reader(&mut file)?;
        let target_name = match lnk_file.get_link_info() {
            Some(li) => match li.path() {
                Some(path) => path,
                None => bail!("link info contains no path"),
            },
            None => bail!("missing link info"),
        };
        println!("target_name is {target_name}");

        Ok(Self {
            lnk_file,
            file_name: file_path.file_name().unwrap().to_string_lossy().into(),
            target_name,
        })
    }
}
