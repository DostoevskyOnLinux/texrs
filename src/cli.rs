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
// | Copyright (c) 2024 Ethan Barry <ethanbarry@howdytx.net>                                                                           |
// | Feel free to contact the author if you do come across this source code for some reason...                                         |
// | <https://github.com/ethanbarry> is the author's profile.                                                                          |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +

use crate::{config::*, DocumentType};

pub fn config_menu(name: &str) -> ProjectConfig {
    use cumaea::{Choice::*, ChoiceColor::*}; // Import enums from the cumaea crate.

    let mut config = ProjectConfig::new();
    config.set_name(name);

    // Prompt for driver:
    match cumaea::prompt_selection(
        "Select driver",
        "(P)dflatex, (l)ualatex, (x)elatex",
        Some(Normal(Green)),
        "p",
    )
    .to_ascii_lowercase()
    .as_str()
    {
        "p" => config.set_driver("pdflatex"),
        "x" => config.set_driver("xelatex"),
        "l" => config.set_driver("lualatex"),
        _ => config.set_driver("pdflatex"),
    }

    match cumaea::prompt_selection(
        "Select document category",
        "(M)athematical, (f)ormal, (p)ersonal",
        Some(Normal(Green)),
        "m",
    )
    .to_ascii_lowercase()
    .as_str()
    {
        "m" => {
            match cumaea::prompt_selection(
                "Select document type",
                "(N)otes, (b)ook, (m)athematical article, (p)resentation",
                Some(Normal(Green)),
                "n",
            )
            .to_ascii_lowercase()
            .as_str()
            {
                "b" => config.set_doctype(DocumentType::Book),
                "m" => config.set_doctype(DocumentType::MathArticle),
                "n" => config.set_doctype(DocumentType::Notes),
                "p" => config.set_doctype(DocumentType::Presentation),
                _ => todo!(),
            }
        }
        "f" => {
            match cumaea::prompt_selection(
                "Select document type",
                "(A)rticle, (t)hesis",
                Some(Normal(Green)),
                "a",
            )
            .to_ascii_lowercase()
            .as_str()
            {
                "a" => config.set_doctype(DocumentType::Article),
                "t" => config.set_doctype(DocumentType::Thesis),
                _ => todo!(),
            }
        }
        "p" => {
            match cumaea::prompt_selection(
                "Select document type",
                "(L)etter, (r)ecipe",
                Some(Normal(Green)),
                "l",
            )
            .to_ascii_lowercase()
            .as_str()
            {
                "l" => config.set_doctype(DocumentType::Letter),
                "r" => config.set_doctype(DocumentType::Recipe),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }

    // Prompt for citations:
    if cumaea::prompt_tf_default("Include citations? (Y/n): ", None, true) {
        config.set_citations(true);
    } else {
        config.set_citations(false);
    }

    // Prompt for graphics:
    if cumaea::prompt_tf_default("Include graphics? (Y/n): ", None, true) {
        config.set_graphics(true);
    } else {
        config.set_graphics(false);
    }

    config
}
