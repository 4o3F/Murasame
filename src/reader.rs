use std::io::{Read, Seek, SeekFrom};

pub trait MurasameFileReader: Read + Seek {}

pub struct MurasameFileInfo {
    pub multifile_begin: u64,
    pub multifile_end: u64,
    pub offset_begin: u64,
    pub stream: Box<dyn MurasameFileReader>,
}

pub struct MurasameMultiFileReader {
    pub files: Vec<MurasameFileInfo>,
    pub current_file_index: usize,
    pub current_position: u64,
    pub total_length: u64,
}

impl Read for MurasameMultiFileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let expected = buf.len();
        if expected == 7615 {
            println!("AAA")
        }
        let mut taken = 0usize;
        while taken != expected && self.current_file_index < self.files.len() {
            let fileinfo = &mut self.files[self.current_file_index];
            let read = fileinfo.stream.read(&mut buf[taken..])?;
            taken += read;
            self.current_position += read as u64;
            if read == 0 {
                self.current_file_index += 1;
                if self.current_file_index > self.files.len() {
                    break;
                }
            }
        }
        while taken != expected && self.current_file_index == self.files.len() {}
        Ok(taken)
    }
}

impl Seek for MurasameMultiFileReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let multifile_position = match pos {
            SeekFrom::Start(index) => { index }
            SeekFrom::End(index) => { self.total_length - index as u64 }
            SeekFrom::Current(index) => { self.current_position + index as u64 }
        };
        if multifile_position == self.total_length {
            self.current_position = multifile_position;
            return Ok(multifile_position);
        }
        let mut result: std::io::Result<u64> = Err(std::io::Error::new(std::io::ErrorKind::Other, "Seek error"));
        for (index, fileinfo) in self.files.iter_mut().enumerate() {
            if multifile_position >= fileinfo.multifile_begin && multifile_position < fileinfo.multifile_end {
                let infile_position = multifile_position - fileinfo.multifile_begin;
                self.current_file_index = index;
                self.current_position = multifile_position;
                result = fileinfo.stream.seek(SeekFrom::Start(fileinfo.offset_begin + infile_position));
            } else {
                fileinfo.stream.seek(SeekFrom::Start(fileinfo.offset_begin)).unwrap();
            }
        }
        result
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.current_position)
    }
}

impl MurasameMultiFileReader {
    pub fn new(mut files: Vec<MurasameFileInfo>) -> Self {
        let mut length = files.iter().map(|x| x.multifile_end - x.multifile_begin).sum();
        // length = length + 1;
        files.iter_mut().for_each(|x| {
            x.stream.seek(SeekFrom::Start(x.offset_begin)).unwrap();
        });
        Self {
            files,
            current_file_index: 0,
            current_position: 0,
            total_length: length,
        }
    }
}