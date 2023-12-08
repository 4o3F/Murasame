use std::fs::File;
use std::io::{Read, Seek};

use crate::reader::{MurasameFileInfo, MurasameFileReader, MurasameMultiFileReader};

mod reader;

impl MurasameFileReader for File {}

fn main() {
    let mut readers = Vec::<MurasameFileInfo>::new();
    let mut begin = 0usize;
    let mut end = 0usize;
    for i in 1..=3 {
        let file = File::open(format!("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z.00{}", i)).unwrap();
        // let mut file = File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z").unwrap();
        let mut offset_begin = 0;
        if i == 1 {
            offset_begin = 16;
        } else if i==2 {
            offset_begin = 8;
        }

        let file_length = file.metadata().unwrap().len();
        end = begin + (file_length - offset_begin) as usize;

        readers.push(MurasameFileInfo {
            multifile_begin: begin as u64,
            multifile_end: end as u64,
            offset_begin,
            stream: Box::new(file),
        });
        begin = end;
    }
    let mut multifile = MurasameMultiFileReader::new(readers);

    // readers.push(into_file(std::fs::File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z.001").unwrap(), "file1".to_string(), 0x8));
    // readers.push(into_file(std::fs::File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z.002").unwrap(), "file2".to_string(), 0x0));
    // readers.push(into_file(std::fs::File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z.003").unwrap(), "file3".to_string(), 0));
    // readers.push(into_file(std::fs::File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z").unwrap(), "file1".to_string(), 16));
    // readers.push(into_file(std::fs::File::open("C:/Users/403F/Downloads/Compressed/test/mediamtx1.7z").unwrap(), "file1".to_string()));

    // let mut multifile = MultiFile::new(readers);

    // println!("{:?}", multifile.stream_position());

    // let mut vec = Vec::new();
    // let _ = multifile.read_to_end(&mut vec).unwrap();
    // let mut context = md5::Context::new();
    // context.consume(vec);
    // let md5 = context.compute();
    // println!("{:?}", md5);
    // sevenz_rust::decompress_file("C:/Users/403F/Downloads/Compressed/test/mediamtx.7z", "C:/Users/403F/Downloads/Compressed/test/").expect("Decompress");
    sevenz_rust::decompress(multifile, "C:/Users/403F/Downloads/Compressed/test/").expect("Decompress");
    println!("Test program");
    return;
    //sevenz_rust::decompress_with_extract_fn_and_password()
}
