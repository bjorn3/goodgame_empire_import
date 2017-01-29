use std::io::{self, BufWriter, BufRead, Write};
use std::fmt;
use std::collections::VecDeque;

pub struct ByteStreamSplitter<'a,T: 'a> {
    separator: &'a [u8],
    input: T,
    started_splitting: bool,
    end_of_stream_reached: bool,
    buffer: Vec<u8>,
    deque: VecDeque<u8>,
    pub next_prepends_seperator: bool
}


pub type SplitResult<T> = Result<T,SplitError>;

#[derive(Debug)]
pub enum SplitType {
    Prefix,
    FullMatch,
    Suffix
}

#[derive(Debug)]
pub enum SplitError {
    Io(io::Error),
    Internal(String)
}

impl fmt::Display for SplitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SplitError::Io(ref e) => e.fmt(f),
            SplitError::Internal(ref s) => write!(f,"{}", s)
        }
    }
}

impl ::std::error::Error for SplitError {
    fn description(&self) -> &str{
        match *self {
            SplitError::Io(ref e) => e.description(),
            SplitError::Internal(ref s) => s
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            SplitError::Io(ref e) => Some(e),
            SplitError::Internal(_) => None
        }
    }
}

impl From<io::Error> for SplitError {
    fn from(e: io::Error) -> Self {
        SplitError::Io(e)
    }
}



impl<'a, T> ByteStreamSplitter<'a,T> where T: BufRead + Sized{
    pub fn new(input: T, separator: &'a [u8]) -> ByteStreamSplitter<'a, T>{
        ByteStreamSplitter{
            input: input,
            separator: separator,
            started_splitting: false,
            end_of_stream_reached: false,
            buffer: Vec::new(),
            deque: VecDeque::new(),
            next_prepends_seperator: false
        }
    }


    fn read_until_first_separator_byte_or_eof(&mut self, output: &mut Write)->SplitResult<Option<u8>>{
        let buffer = &mut self.buffer;
        buffer.clear();

        let num_bytes = try!(self.input.read_until(self.separator[0],buffer));
        if num_bytes == 0{
            Ok(None)
        }else {
            try!(output.write_all(&buffer[0..num_bytes-1]));
            let last_byte = buffer[num_bytes-1];
            Ok(Some(last_byte))
        }
    }

    pub fn next_to_buf(&mut self, output: &mut Write) -> SplitResult<SplitType>{

        if self.end_of_stream_reached {
            return Err(SplitError::Io(io::Error::new(io::ErrorKind::InvalidInput, "Stream has no more data.")));
        }

        loop {
            let last_byte = if let Some(last_byte) = try!(self.read_until_first_separator_byte_or_eof(output)){
                last_byte
            } else {
                self.end_of_stream_reached = true;
                return Ok(SplitType::Suffix);
            };
            if last_byte != self.separator[0] {
                self.end_of_stream_reached = true;
                try!(output.write(&[last_byte]));
                return Ok(SplitType::Suffix);
            }

            let buffer = &mut self.buffer;
            let bytes = &mut self.deque;
            bytes.clear();
            buffer.clear();

            bytes.push_back(last_byte);

            loop {
                buffer.resize(self.separator.len()-bytes.len(),0);

                let num_bytes = try!(self.input.read(buffer));

                for i in 0..num_bytes {
                    bytes.push_back(buffer[i]);
                }
                if self.separator.iter().ne(bytes.into_iter()) {
                    if let Some(b) = bytes.pop_front() {
                        try!(output.write(&[b]));
                    }
                    while let Some(b) = bytes.pop_front() {
                        if b == self.separator[0] {
                            bytes.push_front(b);
                            break;
                        }else{
                            try!(output.write(&[b]));
                        }
                    }
                } else {
                    return Ok(
                        if self.started_splitting{
                            SplitType::FullMatch
                        }else {
                            self.started_splitting = true;
                            SplitType::Prefix
                        }
                    )
                }
                if bytes.len() == 0 {
                    break;
                }
            }
        }



    }
}

impl <'a,T> Iterator for ByteStreamSplitter<'a,T> where T: BufRead + Sized{
    type Item = SplitResult<Vec<u8>>;

    fn next(&mut self) -> Option<SplitResult<Vec<u8>>> {
        if self.end_of_stream_reached {
            None
        } else {
            let mut part = BufWriter::new(
                if self.started_splitting && self.next_prepends_seperator{
                    Vec::from(self.separator)
                }else {
                    Vec::new()
                }
            );
            let result = self.next_to_buf(&mut part)
            .and(part.flush().map_err(SplitError::Io))
            .and(part.into_inner().map_err(|e|SplitError::Internal(e.to_string())));

            match result {
                Ok(inner) => Some(Ok(inner)),
                _   => None
            }
        }
   }
}


#[test]
fn test_with_prefix() {
    let separator = [0x00, 0x00];
    let mut data = io::Cursor::new(vec![
        0xAA, 0xAB,                     // Prefix
        0x00, 0x00, 0x01, 0x02, 0x03,   // FullMatch
        0x00, 0x00, 0x04, 0x05, 0x06,   // FullMatch
        0x00, 0x00, 0x07, 0x08          // Suffix
        ]);

    let mut splitter = ByteStreamSplitter::new(&mut data, &separator);
    let prefix = splitter.next().unwrap().unwrap();
    let match1 = splitter.next().unwrap().unwrap();
    let match2 = splitter.next().unwrap().unwrap();
    let suffix = splitter.next().unwrap().unwrap();

    assert_eq!(prefix, vec![0xAA, 0xAB]);
    assert_eq!(match1, vec![0x01, 0x02, 0x03]);
    assert_eq!(match2, vec![0x04, 0x05, 0x06]);
    assert_eq!(suffix, vec![0x07, 0x08]);
}

#[test]
fn test_without_prefix() {
    let separator = [0x00, 0x00];
    let mut data = io::Cursor::new(vec![
        0x00, 0x00, 0x01, 0x02, 0x03,   // FullMatch
        0x00, 0x00, 0x04, 0x05, 0x06,   // FullMatch
        0x00, 0x00, 0x07, 0x08          // Suffix
        ]);

    let mut splitter = ByteStreamSplitter::new(&mut data, &separator);
    let prefix = splitter.next().unwrap().unwrap();
    let match1 = splitter.next().unwrap().unwrap();
    let match2 = splitter.next().unwrap().unwrap();
    let suffix = splitter.next().unwrap().unwrap();

    assert_eq!(prefix, vec![] as Vec<u8>);
    assert_eq!(match1, vec![0x01, 0x02, 0x03]);
    assert_eq!(match2, vec![0x04, 0x05, 0x06]);
    assert_eq!(suffix, vec![0x07, 0x08]);
}

#[test]
fn test_skip_bug() {
    let separator = [0x00, 0x00];
    let mut data = io::Cursor::new(vec![
        0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x00, 0x00, 0x00, 0x04, 0x05, 0x06,
        0x00, 0x00, 0x07, 0x08
    ]);

    let mut splitter = ByteStreamSplitter::new(&mut data, &separator);
    let prefix = splitter.next().unwrap().unwrap();
    println!("p{:?}", prefix);
    let match1 = splitter.next().unwrap().unwrap();
    println!("1m{:?}", match1);
    let match2 = splitter.next().unwrap().unwrap();
    println!("1m{:?}", match2);
    let suffix = splitter.next().unwrap().unwrap();
    println!("s{:?}", suffix);


    assert_eq!(prefix, vec![] as Vec<u8>);
    assert_eq!(match1, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    assert_eq!(match2, vec![0x00, 0x04, 0x05, 0x06]);
    assert_eq!(suffix, vec![0x07, 0x08]);
}


#[test]
fn test_skip_bug2() {
    let separator = [0x01, 0x02, 0x03];
    let mut data = io::Cursor::new(vec![
        0x01, 0x02, 0x03, 0x02, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        0x01, 0x02, 0x03, 0x08
    ]);

    let mut splitter = ByteStreamSplitter::new(&mut data, &separator);
    let prefix = splitter.next().unwrap().unwrap();
    println!("p{:?}", prefix);
    let match1 = splitter.next().unwrap().unwrap();
    println!("1m{:?}", match1);
    let match2 = splitter.next().unwrap().unwrap();
    println!("1m{:?}", match2);
    let suffix = splitter.next().unwrap().unwrap();
    println!("s{:?}", suffix);


    assert_eq!(prefix, vec![] as Vec<u8>);
    assert_eq!(match1, vec![0x02, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    assert_eq!(match2, vec![0x04, 0x05, 0x06]);
    assert_eq!(suffix, vec![0x08]);

}