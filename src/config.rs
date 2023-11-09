// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
// | This file is part of texrs.                                                                                                       |
// |                                                                                                                                   |
// | texrs is free software: you can redistribute it and/or modify it under the terms                                                  |
// | of the GNU General Public License as published by the Free Software Foundation,                                                   |
// | either version 3 of the License, or (at your option) any later version.                                                           |
// |                                                                                                                                   |
// | texrs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;                                                |
// | without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.                                         |
// | See the GNU General Public License for more details.                                                                              |
// |                                                                                                                                   |
// | You should have received a copy of the GNU General Public License along with texrs. If not, see <https://www.gnu.org/licenses/>.  |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
// | Copyright (c) 2023 Ethan Barry <ethanbarry@howdytx.net>                                                                           |
// | Feel free to contact the author if you do come across this source code for some reason...                                         |
// | <https://github.com/ethanbarry> is the author's profile.                                                                          |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +

use crate::DocumentType;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::Write;
use toml;

/// The ProjectConfig struct stores five pieces of
/// information about the project: what name, driver,
/// whether citations or graphics are used, & what
/// type of document it is.
///
/// ## Structure
/// name: String,
/// driver: String,
/// citations: bool,
/// graphics: bool,
/// doctype: DocumentType
#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    driver: String,
    citations: bool,
    graphics: bool,
    doctype: DocumentType,
} // TODO: I don't think these need to be `pub` now...

impl ProjectConfig {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_driver(&self) -> String {
        self.driver.clone()
    }

    pub fn get_citations(&self) -> bool {
        self.citations
    }

    pub fn get_graphics(&self) -> bool {
        self.graphics
    }

    pub fn get_doctype(&self) -> DocumentType {
        self.doctype.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn set_driver(&mut self, driver: &str) {
        self.driver = driver.to_owned();
    }

    pub fn set_citations(&mut self, citations: bool) {
        self.citations = citations;
    }

    pub fn set_graphics(&mut self, graphics: bool) {
        self.graphics = graphics;
    }

    pub fn set_doctype(&mut self, doctype: DocumentType) {
        self.doctype = doctype;
    }

    pub fn new() -> ProjectConfig {
        ProjectConfig {
            name: "document1".to_owned(),
            driver: "pdflatex".to_owned(),
            citations: true,
            graphics: true,
            doctype: DocumentType::Letter,
        }
    }
}

pub fn write_project_config(config: &ProjectConfig) -> Result<(), toml::ser::Error> {
    let root_dir = config.get_name() + "/";
    let toml_str = toml::to_string(&config)?;
    let mut file = File::create(root_dir + "config.toml").expect("Location must be writable.");
    file.write_all(toml_str.as_bytes())
        .expect("Location must be writable.");
    Ok(())
}
