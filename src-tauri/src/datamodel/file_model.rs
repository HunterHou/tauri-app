// 数据模型 文件
#[derive(Debug)]
pub struct FileModel {
    pub Id: String,
    pub Name: String,
    pub Code: String,
    pub MovieType: String,
    pub FileType: String,
    pub Png: String,
    pub Jpg: String,
    pub Actress: String,
    pub Path: String,
    pub DirPath: String,
    pub Title: String,
    pub SizeStr: String,
    pub Size: i64,
    pub MTime: i64,
}

impl FileModel {
    pub fn is_empty(&self) -> bool {
        self.Id == ""
    }

    pub fn new() -> FileModel {
        return FileModel {
            Id: "".to_string(),
            Name: "".to_string(),
            Code: "".to_string(),
            MovieType: "".to_string(),
            FileType: "".to_string(),
            Png: "".to_string(),
            Jpg: "".to_string(),
            Actress: "".to_string(),
            Path: "".to_string(),
            DirPath: "".to_string(),
            Title: "".to_string(),
            SizeStr: "".to_string(),
            Size: 0,
            MTime: 0,
        };
    }


    pub fn from_path(dirpath: String, path: String, name: String, ext: String) -> FileModel {
        let path_bac = &String::from(path);
        return FileModel {
            Id: String::from(path_bac),
            Name: name,
            Code: "".to_string(),
            MovieType: "".to_string(),
            FileType: ext,
            Png: "".to_string(),
            Jpg: "".to_string(),
            Actress: "".to_string(),
            Path: String::from(path_bac),
            DirPath: dirpath,
            Title: "".to_string(),
            SizeStr: "".to_string(),
            Size: 0,
            MTime: 0,
        };
    }
}

