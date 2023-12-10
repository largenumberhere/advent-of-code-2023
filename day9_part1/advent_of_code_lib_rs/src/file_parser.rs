use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};
use std::num::ParseIntError;
use std::path::Path;
use std::str::{from_utf8, FromStr};

pub struct FileParser<R: Read> {
    file: R,
    queue: VecDeque<u8>,
}

impl FileParser<BufReader<File>> {
    pub fn new_from_path(
        path: impl AsRef<Path>,
    ) -> Result<FileParser<BufReader<File>>, std::io::Error> {
        let file = File::open(path)?;
        let bufreader = BufReader::new(file);
        let parser = Self::new_from_reader(bufreader);

        return Ok(parser);
    }
}

impl<R: Read> FileParser<R> {
    pub fn new_from_reader(reader: R) -> FileParser<R> {
        FileParser {
            file: reader,
            queue: VecDeque::new(),
        }
    }

    fn get_utf8_length_from_first_byte(first_byte: u8) -> Option<usize> {
        // relevant reading: https://www.quora.com/In-Unicode-UTF-8-how-do-we-know-the-start-and-the-end-of-a-character-since-characters-vary-in-length

        // Table in section 3 of rfc3629 shows expected starting bits of first utf8 byte
        //  https://datatracker.ietf.org/doc/html/rfc3629

        let first_byte = first_byte.to_le();

        // get important bits
        let bit1 = (first_byte >> 7) & 1;
        let bit2 = (first_byte >> 6) & 1;
        let bit3 = (first_byte >> 5) & 1;
        let bit4 = (first_byte >> 4) & 1;
        let bit5 = (first_byte >> 3) & 1;

        if bit1 == 0 {
            return Some(1);
        }

        if bit1 == 1 && bit2 == 1 && bit3 == 0 {
            return Some(2);
        }

        if bit1 == 1 && bit2 == 1 && bit3 == 1 && bit4 == 0 {
            return Some(3);
        }

        if bit1 == 1 && bit2 == 1 && bit3 == 1 && bit4 == 1 && bit5 == 0 {
            return Some(4);
        }

        // invalid utf8
        return None;
    }

    /// Read a utf8 char from the file
    pub fn read_char(&mut self) -> Result<char, ScannerError> {
        // make sure at least 4 bytes in queue
        let mut tries = 0;
        while self.queue.len() <= 4 {
            let mut bytes = [0; 4];
            let count = self.file.read(&mut bytes);
            let count = match count {
                Ok(v) => v,
                Err(e) => {
                    panic!("{:?}", e);
                }
            };

            for b in &bytes[0..count] {
                self.queue.push_back(*b);
            }

            tries += 1;
            // most likely end of file
            if tries == 4 {
                break;
            }
        }

        // check for end of file
        if self.queue.len() == 0 {
            return Err(ScannerError::EndOfFile);
        }

        // check for next codepoint length
        let length = Self::get_utf8_length_from_first_byte(*self.queue.iter().next().unwrap());
        let length = match length {
            Some(v) => v,
            None => {
                return Err(ScannerError::InvalidUtf8);
            }
        };

        // parse to &str
        let section = &self.queue.make_contiguous()[0..length];
        let string = from_utf8(&section);
        let string = match string {
            Ok(v) => v,
            Err(e) => {
                //panic!("{:?}", e);
                return Err(ScannerError::InvalidUtf8);
            }
        };

        // convert to char
        let chr = string.chars().next().unwrap();

        self.dequeue_char(length);

        return Ok(chr);
    }

    /// Read the next char without moving the cursor. Returns the size of the char in bytes.
    ///  Length is returned so user can dequeue the char only when desired
    fn peek_char(&mut self) -> Result<(char, usize), ScannerError> {
        // make sure at least 4 bytes in queue
        let mut tries = 0;
        while self.queue.len() <= 4 {
            let mut bytes = [0; 4];
            let count = self.file.read(&mut bytes);
            let count = match count {
                Ok(v) => v,
                Err(e) => {
                    panic!("{:?}", e);
                }
            };

            for b in &bytes[0..count] {
                self.queue.push_back(*b);
            }

            tries += 1;
            // most likely end of file
            if tries == 4 {
                break;
            }
        }

        // check for end of file
        if self.queue.len() == 0 {
            return Err(ScannerError::EndOfFile);
        }

        // check for next codepoint length
        let length = Self::get_utf8_length_from_first_byte(*self.queue.iter().next().unwrap());
        let length = match length {
            Some(v) => v,
            None => {
                return Err(ScannerError::InvalidUtf8);
            }
        };

        // parse to &str
        let section = &self.queue.make_contiguous()[0..length];
        let string = from_utf8(&section);
        let string = match string {
            Ok(v) => v,
            Err(e) => {
                //panic!("{:?}", e);
                return Err(ScannerError::InvalidUtf8);
            }
        };

        // convert to char
        let chr = string.chars().next().unwrap();

        return Ok((chr, length));
    }

    fn dequeue_char(&mut self, length: usize) {
        for i in 0..length {
            self.queue.pop_front();
        }
    }

    /// returns true if \n, \r or eof is next
    pub fn peek_newline(&mut self) -> Result<bool, ScannerError> {
        let (c, length) = match self.peek_char() {
            Ok(v) => v,
            Err(ScannerError::EndOfFile) => return Ok(true),
            Err(e) => return Err(e),
        };

        let sameness = c == '\n' || c == '\r';

        return Ok(sameness);
    }

    // Read until \n is encountered. Returns everything before the \n
    //  Ignores end of file
    pub fn read_line(&mut self) -> Result<String, ScannerError> {
        let mut line = String::new();
        loop {
            let ch1 = self.peek_char();

            let (ch1, length) = match ch1 {
                Err(ScannerError::EndOfFile) => {
                    return Err(ScannerError::EndOfFile);
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(v) => v,
            };

            self.dequeue_char(length);

            // break if \n
            if ch1 == '\n' {
                // if \r\n, remove the \r and don't add the \n
                if let Some(last_c) = line.chars().last() {
                    if last_c == '\r' {
                        line.pop();
                    }
                }

                break;
            }

            line.push(ch1);
        }

        return Ok(line);
    }

    pub fn discard_line(&mut self) -> Result<(), ScannerError> {
        let line = self.read_line()?;
        return Ok(());
    }

    /// Read a single number
    pub fn read_digit(&mut self) -> Result<u32, ScannerError> {
        let ch = self.read_char()?;

        //println!("{ch}");
        let num = ch.to_digit(10).ok_or(ScannerError::NotADigit)?;

        return Ok(num);
    }

    /// Read until a non-digit is found and convert the string to u32
    ///  Ignores end of file
    pub fn read_number(&mut self) -> Result<u32, ScannerError> {
        let mut digits = String::new();
        loop {
            let (ch, length) = match self.peek_char() {
                Ok(v) => v,
                Err(ScannerError::EndOfFile) => {
                    break;
                }
                Err(e) => return Err(e),
            };

            if !ch.is_digit(10) {
                break;
            }

            self.dequeue_char(length);

            digits.push(ch);
        }

        let num = u32::from_str(digits.as_str());
        let num = num.map_err(|e| ScannerError::ParseIntError(e))?;

        return Ok(num);
    }

    /// Assert that the string is present or return an error
    pub fn discard_str(&mut self, expected: &str) -> Result<(), ScannerError> {
        let mut string = String::new();
        for i in 0..expected.len() {
            let ch = self.read_char()?;
            string.push(ch);
        }

        if string.eq(expected) {
            return Ok(());
        }

        return Err(ScannerError::StringDidNotMatch);
    }

    /// Ignore the block of whitespace in front of the cursor, including newline
    pub fn discard_contiguous_whitespace(&mut self) -> Result<(), ScannerError> {
        loop {
            let (char, length) = self.peek_char()?;
            if !char.is_whitespace() {
                break;
            }

            self.dequeue_char(length);
        }

        return Ok(());
    }

    pub fn discard_contiguous_whitespace_ecx_newline(&mut self) -> Result<(), ScannerError> {
        loop {
            let (char, length) = self.peek_char()?;
            if !char.is_whitespace() || char == '\n' {
                break;
            }

            self.dequeue_char(length);
        }

        return Ok(());
    }

    /// read until this character is encountered. Similar to split, but only does one at a time
    pub fn read_until(&mut self, character: char) -> Result<String, ScannerError> {
        let mut string = String::new();

        let mut current;
        loop {
            current = self.read_char()?;

            if current != character {
                string.push(current);
            }
            else {
                break;
            }
        }

        return Ok(string);
    }

    pub fn parse_file_to_vec(&mut self, seperator: char) -> Result<Vec<String>, ScannerError> {
        let mut buffer = String::new();
        let mut buffers = Vec::new();
        let mut current;
        loop {
            current = match self.read_char() {
                Ok(v) => {v}
                Err(e) => {
                    match e {
                        ScannerError::EndOfFile => {
                            break;
                        }
                        er => return Err(er)
                    }
                }
            };

            if current == seperator {

                // end current buffer
                buffers.push(buffer.clone());
                buffer.clear();

                continue;
            }

            buffer.push(current);
        }

        // end last buffer
        buffers.push(buffer);
        return Ok(buffers);
    }

    /// Does what it says...
    ///  Does not include newline characters at end of line.
    ///  Will give end of file error only if the first part of the input is eof
    pub fn parse_line_to_vec(&mut self, seperator: char) -> Result<Vec<String>, ScannerError> {
        if let Err(e) = self.peek_char() {
            return Err(e);
        }

        let mut buffer = String::new();
        let mut buffers = Vec::new();
        let mut current;
        loop {
            current = match self.read_char() {
                Ok(v) => {v}
                Err(e) => {
                    match e {
                        ScannerError::EndOfFile => {
                            break;
                        }
                        er => return Err(er)
                    }
                }
            };

            if current == '\n' {
                break;
            }

            if current == '\r' {
                let peek = self.peek_char()?;
                if peek.0 == '\n' {
                    self.dequeue_char(peek.1);
                    break;
                }
            }

            if current == seperator {
                // end current buffer
                buffers.push(buffer.clone());
                buffer.clear();
                continue;
            }

            buffer.push(current);
        }

        // end last buffer
        if buffer.len() != 0 {
            buffers.push(buffer);
        }
        return Ok(buffers);
    }

    pub fn parse_line_to_vec_u32(&mut self, separator: char) -> Result<Vec<u32>, ScannerError> {
        let bits = self.parse_line_to_vec(separator)?;

        let mut numbers = Vec::with_capacity(bits.len());
        for bit in bits {

            let int = bit.parse::<u32>()
                .map_err(|e| ScannerError::ParseIntError(e))
                ?;

            numbers.push(int);
        }

        return Ok(numbers);
    }

    pub fn parse_line_to_vec_i64(&mut self, separator: char) -> Result<Vec<i64>, ScannerError> {
        let bits = self.parse_line_to_vec(separator)?;

        let mut numbers = Vec::with_capacity(bits.len());
        for bit in bits {
            let int = bit.parse::<i64>()
                .map_err(|e| ScannerError::ParseIntError(e))
                ?;

            numbers.push(int);
        }

        return Ok(numbers);
    }
}

#[derive(Debug)]
pub enum ScannerError {
    InvalidUtf8,
    EndOfFile,
    NotADigit,
    StringDidNotMatch,
    ParseIntError(ParseIntError),
}

/*
TODO: make this into tests

    println!("Hello, world!");
    let file = File::open("example_input.txt").unwrap();
    let mut scanner = string_scanner::FileParser::new_from_reader(file);
    // for i in 0 ..50 {
    //     println!("{:?}", scanner.read_char())
    // }

    // println!("{:?}", scanner.read_line());
    //
    // scanner.discard_str("Distance:").unwrap();
    // scanner.discard_contiguous_whitespace().unwrap();
    // //println!("{:?}", scanner.read_digit());
    // println!("{:?}", scanner.read_line());
    //
    // scanner

    scanner.discard_str("Time:").unwrap();
    println!("'Time:' discarded");
    scanner.discard_contiguous_whitespace().unwrap();
    println!("whitespace discarded");
    let number = scanner.read_number().unwrap();
    println!("first number: {}", number);

    scanner.discard_contiguous_whitespace().unwrap();
    let number2 = scanner.read_number().unwrap();
    println!("first number: {}", number2);


    scanner.discard_contiguous_whitespace().unwrap();
    let number3 = scanner.read_number().unwrap();
    println!("first number: {}", number3);

    scanner.discard_line().unwrap();

    scanner.discard_str("Distance:").unwrap();
    println!("'Time:' discarded");
    scanner.discard_contiguous_whitespace().unwrap();
    println!("whitespace discarded");
    let number = scanner.read_number().unwrap();
    println!("first number: {}", number);

    scanner.discard_contiguous_whitespace().unwrap();
    let number2 = scanner.read_number().unwrap();
    println!("first number: {}", number2);


    scanner.discard_contiguous_whitespace().unwrap();
    let number3 = scanner.read_number().unwrap();
    println!("first number: {}", number3);
*/
