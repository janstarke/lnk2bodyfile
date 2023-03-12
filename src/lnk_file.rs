use std::{fs::File, path::PathBuf};

use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use lnk_parser::{
    LNKParser,
};

use crate::lnk_target_path::LnkTargetPath;

pub struct LnkFile {
    lnk_file: LNKParser,
    file_name: String,
    target_name: LnkTargetPath,
}


impl LnkFile {
    pub fn print_bodyfile(&self) {
        self.print_bodyfile_for_me();
    }

    fn print_bodyfile_for_me(&self) {
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
}

impl TryFrom<&str> for LnkFile {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let file_path = PathBuf::from(value);
        let lnk_file = LNKParser::from_reader(&mut file)?;
        let target_name = LnkTargetPath::try_from(&lnk_file)?;

        Ok(Self {
            lnk_file,
            file_name: file_path.file_name().unwrap().to_string_lossy().into(),
            target_name,
        })
    }
}
