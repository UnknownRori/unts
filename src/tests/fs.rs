use crate::fs::FileService;

#[cfg(test)]
fn setup_file_service() -> FileService {
    FileService::default()
}

#[cfg(test)]
#[tokio::test]
async fn test_create_n_read_file() {
    let fs_service = setup_file_service();
    let filename = "case1.txt";

    let content = String::from("Hello, World");
    fs_service.write_data(filename, &content).unwrap();

    let read_content = fs_service.read_data(filename).unwrap();
    assert!(content == read_content);

    fs_service.delete_data("case1.txt").unwrap();

    let path  = fs_service.generate_data_path(filename);
    assert!((*path).is_file() == false);
}
