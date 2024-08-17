#![allow(non_snake_case)]

use std::io::Write;
use std::path::PathBuf;

use colored::{Color, Colorize};
use regex::Regex;
use rust_embed::RustEmbed;
use rust_i18n::t;
use zip::write::SimpleFileOptions;

use crate::FileName::*;

rust_i18n::i18n!("i18n");

mod FileName;
mod log;

#[derive(RustEmbed)]
#[folder = "Assets/"]
struct Asset;

pub enum EDA {
    Kicad,
    Protel,

    /// 自动识别
    Auto,

    /// 自定义
    Custom(String),
}

pub trait JlcTrait {
    fn new(path: String, output_path: String, eda: EDA) -> Self;

    /// 添加 “PCB下单必读.txt” 文件
    fn add_pcb_must_read(&mut self) -> Result<(), std::io::Error>;

    /// 遍历文件夹，如果找到了匹配的文件，就将它复制到指定的路径，并且重命名为JLC_STYLE
    fn copy_file(&mut self) -> Result<(), std::io::Error>;

    /// 将处理之后的文件打包为zip文件
    fn zip_file(&mut self, name: &str) -> Result<(), std::io::Error>;
}

pub struct JLC {
    /// The path to the Gerber file
    pub path: String,

    /// 输出路径
    pub output_path: String,

    /// eda software name
    pub eda: EDA,

    /// 处理之后的文件路径
    pub process_path: Vec<PathBuf>,
}

impl JlcTrait for JLC {
    fn new(path: String, output_path: String, eda: EDA) -> Self {
        Self {
            path,
            output_path,
            eda,
            process_path: vec![],
        }
    }

    fn add_pcb_must_read(&mut self) -> Result<(), std::io::Error> {
        const NAME: &str = "PCB下单必读.txt";
        let content = Asset::get(NAME).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ))?;
        // 把这个文件写到指定的路径
        std::fs::write(
            std::path::Path::new(&self.output_path).join(NAME),
            content.data.as_ref(),
        )?;
        self.process_path
            .push(std::path::Path::new(&self.output_path).join(NAME));
        Ok(())
    }

    fn copy_file(&mut self) -> Result<(), std::io::Error> {
        let files = std::fs::read_dir(&self.path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let style = match &self.eda {
            EDA::Auto => {
                // 自动识别
                ALL_STYLE
                    .iter()
                    .find(|rule| {
                        // 我们假定所有的合法Gerber文件里面都一定包含了一个边框层，所以我们使用这个边框层来尝试判断是什么风格的EDA
                        let re = Regex::new(rule.Board_Outline).unwrap();
                        files.iter().any(|file| re.is_match(file.to_str().unwrap()))
                    })
                    .copied()
            }

            EDA::Custom(name) => {
                // 自定义
                ALL_STYLE.iter().find(|rule| rule.EDA_Name == name).copied()
            }

            EDA::Kicad => {
                // 使用KiCAD风格
                Some(&KICAD_STYLE).map(|v| &**v)
            }

            // EDA::AltiumDesigner => {
            //     // 使用Altium Designer风格
            //     Some(ALTUIM_DESIGNER_STYLE)
            // },
            _ => {
                // 直接使用指定的风格
                None
            }
        };

        if style.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No matching EDA style found",
            ));
        }

        for file in files {
            if let Some(style) = style {
                if file.is_file() {
                    // 匹配文件名
                    let file_name = file.file_name().unwrap().to_str().unwrap();
                    // 遍历style的所有字段
                    for (key, value) in style.clone() {
                        if key == "null" {
                            continue;
                        }

                        let mut file_paths: Vec<PathBuf> = vec![];
                        for value in value {
                            if Regex::new(value).unwrap().is_match(file_name) {
                                let file_path = match key {
                                    "InnerLayer" => {
                                        let mut num = 0;
                                        let re = Regex::new(r"\d+").unwrap();
                                        if let Some(caps) = re.captures(file_name) {
                                            // 获取第一个捕获组（即第一个数字）
                                            if let Some(matched) = caps.get(0) {
                                                num = matched.as_str().parse::<i32>().unwrap();
                                            }
                                        } else {
                                            return Err(std::io::Error::new(
                                                std::io::ErrorKind::NotFound,
                                                "No number found",
                                            ));
                                        }

                                        let new_file_name = JLC_STYLE
                                            .InnerLayer_Templete
                                            .replace("{0}", num.to_string().as_str())
                                            .replace("{1}", num.to_string().as_str());

                                        let file_path = std::path::Path::new(&self.output_path)
                                            .join(new_file_name);
                                        file_path
                                    }

                                    _ => {
                                        let file_path = std::path::Path::new(&self.output_path)
                                            .join(JLC_STYLE.get(key).unwrap());
                                        file_path
                                    }
                                };
                                file_paths.push(file_path);
                            }

                            for file_path in &file_paths {
                                std::fs::copy(file.clone(), file_path.clone())?;
                            }

                            // 钻孔层只复制不修改
                            const SKIP_KEYS: [&str; 3] =
                                ["NPTH_Through", "PTH_Through", "PTH_Through_Via"];
                            if SKIP_KEYS.contains(&key) {
                                continue;
                            }

                            // 获取运行时间
                            let now = chrono::Local::now();

                            for file_path in &file_paths {
                                // 在复制之后的文件的头部插入一些信息
                                let temp = std::fs::read_to_string(&file_path)?;
                                let temp = format!(
                                    "G04 EasyEDA Pro v2.1.61, {}*\nG04 Gerber Generator version 0.3*{}",
                                    now.format("%Y-%m-%d %H:%M:%S"),
                                    temp
                                );
                                std::fs::write(&file_path, temp)?;
                                
                                // 将处理之后的文件路径保存到process_path
                                self.process_path.push(file_path.clone());
                            }
                        }
                    }
                }
            }
        }

        // 将PCB下单必读文件复制到输出路径
        self.add_pcb_must_read()?;

        Ok(())
    }

    fn zip_file(&mut self, name: &str) -> Result<(), std::io::Error> {
        let zip_file = std::path::Path::new(&self.output_path).join(name.to_owned() + ".zip");
        let mut zip = zip::ZipWriter::new(std::fs::File::create(zip_file)?);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        for file in &self.process_path {
            let file_name = file.file_name().unwrap().to_str().unwrap();
            zip.start_file(file_name, options)?;
            let content = std::fs::read(file)?;
            zip.write(&content)?;
        }

        zip.finish()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
