use std::path::PathBuf;
use clipboard_win::{formats, Getter, SystemError};

pub fn get_filenames_from_selection() -> Result<Vec<String>, SystemError> {
    let mut filenames: Vec<PathBuf> = Vec::new();
    match formats::FileList.read_clipboard(&mut filenames) {
        Ok(_) => Ok(<Vec<PathBuf> as AsRef<[PathBuf]>>::as_ref(&filenames)
            .into_iter()
            .filter_map(|f| f.file_name())
            .filter_map(|o| o.to_str())
            .map(|o| o.to_owned())
            .collect()
        ),
        Err(_) => {
            let mut clipboard_content = String::new();
            formats::Unicode.read_clipboard(&mut clipboard_content)
                .map(|_| split_selection(&clipboard_content))
        }
    }
}

fn split_selection(selection: &str) -> Vec<String> {
    selection.split(|c| (c as u32) < 32).map(|o| o.to_owned()).collect()
}

#[test]
fn test_split() {
    const FILE_LIST: &str = "Develop
Documents and Settings
FPC
Go
IDriveLocal
Intel
msys64
OneDriveTemp
OSGeo4W64
PerfLogs";

    let filenames = split_selection(FILE_LIST);
    assert!(filenames.contains(&String::from("Develop")));
    assert!(filenames.contains(&String::from("Documents and Settings")));
    assert!(filenames.contains(&String::from("msys64")));
    assert_eq!(filenames.len(), 10);
}
