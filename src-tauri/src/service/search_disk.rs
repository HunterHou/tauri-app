use std::io::Result;
use walkdir::WalkDir;
use std::path::Path;
use std::time::SystemTime;
use crate::datamodel::file_model::FileModel as FileModel;


fn visit_dirs(dir: &str) -> Result<Vec<FileModel>> {
    let walker = WalkDir::new(dir).into_iter();
    let mut filelist: Vec<FileModel> = Vec::new();
    for entry in walker {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let mut size = 0;
            let mut created = SystemTime::now();
            match entry.metadata() {
                Ok(meta) => {
                    size = meta.len();
                    match meta.created() {
                        Ok(value) => created = value,
                        _ => {}
                    };
                }
                _ => {}
            }
            let filepath = Path::new(entry.path());
            // println!("{:?}", filepath.parent());
            // println!("{:?}", filepath.file_stem());
            // println!("{:?}", filepath.extension());
            // println!("{:?}", filepath.file_name());
            let mut dirpath = "".to_string();
            match filepath.parent() {
                Some(value) => dirpath = format!("{}", value.display()),
                _ => {}
            }


            let mut path = "".to_string();
            match filepath.parent() {
                Some(value) => path = format!("{}", value.display()),
                _ => {}
            }
            let mut filename = "".to_string();
            match filepath.parent() {
                Some(value) => filename = format!("{}", value.display()),
                _ => {}
            }

            let mut extname = "".to_string();
            match filepath.parent() {
                Some(value) => extname = format!("{}", value.display()),
                _ => {}
            }

            let mut file = FileModel::from_path(
                dirpath,
                path,
                filename,
                extname,
                size,
                created);
            if file.is_empty() {
                continue;
            }
            // println!("{:?}", file);
            filelist.push(file);
        }
    }
    Ok(filelist)
}


pub fn search_disk(dir_paths: Vec<&str>) -> Result<Vec<FileModel>> {
    let mut filelist: Vec<FileModel> = Vec::new();
    for dir_path in dir_paths {
        match visit_dirs(dir_path) {
            Ok(value) => {
                for val in value {
                    filelist.push(val)
                }
            }
            Err(err) => println!("{}", err)
        }
    }
    Ok(filelist)
}