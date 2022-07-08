#[derive(Debug, PartialEq)]
struct File {
    pub id: u8,
    pub name: String,
}

impl File {
    fn new(id: u8, name: String) -> Self {
        File { id, name }
    }
}

fn main() {
    let mut files: Vec<File> = vec![
        File::new(8, "file.log.8".to_string()),
        File::new(3, "file.log.3".to_string()),
    ];
    assert_eq!(
        vec![
            File::new(8, "file.log.8".to_string()),
            File::new(3, "file.log.3".to_string())
        ],
        files
    );
    files.sort_by_key(|file| file.id);
    assert_eq!(
        vec![
            File::new(3, "file.log.3".to_string()),
            File::new(8, "file.log.8".to_string())
        ],
        files
    );
}
