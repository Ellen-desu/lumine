use lumine::{Disposition, FileStream, Stream};
use std::{fs::File, io::Write, path::PathBuf};

fn create_temp_file(name: &str, content: &[u8]) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(name);
    let mut file = File::create(&path).unwrap();
    file.write_all(content).unwrap();
    path
}

#[tokio::test]
async fn filestream_open() {
    let path = create_temp_file("test_open.bin", b"hello world");

    let stream = FileStream::open(&path).await.unwrap();

    assert_eq!(stream.size_hint(), Some(11));

    let content_type = stream
        .headers_hint()
        .unwrap()
        .get(http::header::CONTENT_TYPE)
        .unwrap();
    assert!(!content_type.to_str().unwrap().is_empty());

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn filestream_open_with_disposition() {
    let path = create_temp_file("test_disp.txt", b"dummy text content");

    let stream = FileStream::open_with_disposition(&path, Disposition::Attachment)
        .await
        .unwrap();

    assert_eq!(stream.size_hint(), Some(18));

    let headers = stream.headers_hint().unwrap();
    let content_type = headers.get(http::header::CONTENT_TYPE).unwrap();
    assert_eq!(content_type.to_str().unwrap(), "application/octet-stream");

    let disposition = headers.get(http::header::CONTENT_DISPOSITION).unwrap();

    let disposition_str = disposition.to_str().unwrap();
    assert_eq!(disposition_str, "attachment; filename=\"test_disp.txt\"");

    std::fs::remove_file(path).unwrap();
}
