use std::io::{BufWriter, BufRead, Write, Bytes};
use std::io;
use std::collections::VecDeque;

pub struct ByteStreamSplitter<T> {
    input: Bytes<T>,
    end_of_stream_reached: bool
}

pub type SplitResult<T> = Result<T,io::Error>;

#[derive(Debug)]
pub enum SplitType {
    FullMatch,
    Suffix
}

impl<T> ByteStreamSplitter<T> where T: BufRead + Sized{
    pub fn new(input: T) -> ByteStreamSplitter<T>{
        ByteStreamSplitter{
            input: input.bytes(),
            end_of_stream_reached: false
        }
    }

    fn next_to_buf(&mut self, output: &mut Write) -> SplitResult<SplitType>{
        let mut bytes = VecDeque::new();
        if let Some(b) = self.input.next(){
            bytes.push_back(try!(b));
        }
        while bytes.iter().next() != Some(&0){
            let front_byte = bytes.pop_front().unwrap();
            try!(output.write(&[front_byte]));
            let next_byte = self.input.by_ref().next();
            if let Some(r) = next_byte {
                bytes.push_back(try!(r));
            }else {
                self.end_of_stream_reached = true;
                break;
            }
        }

        if self.end_of_stream_reached {
            try!(output.write_all(&bytes.into_iter().collect::<Vec<_>>()[..]));
            Ok(SplitType::Suffix)
        }else{
            Ok(SplitType::FullMatch)
        }

    }
}

impl <T> Iterator for ByteStreamSplitter<T> where T: BufRead + Sized{
    type Item = SplitResult<Vec<u8>>;

    fn next(&mut self) -> Option<SplitResult<Vec<u8>>> {
        if self.end_of_stream_reached {
            None
        } else {
            let mut part = BufWriter::new(Vec::new());
            let result = self.next_to_buf(&mut part)
            .and(part.flush())
            .and(Ok(part.into_inner().unwrap()));
            match result {
                Ok(inner) => Some(Ok(inner)),
                _   => None
            }
        }
   }
}
