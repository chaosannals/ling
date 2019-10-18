use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;

#[derive(Debug)]
enum Mode {
    Normal,
    Comment,
}

#[derive(Debug)]
pub enum Token {
    Comma,
    Dot,
    Plus,
    Minus,
    Identifier(String),
}

#[derive(Debug)]
pub struct Lexeme {
    pub row: u32,
    pub column: u32,
    pub token: Token,
}

#[derive(Debug)]
pub struct Lexer {
    row: u32,
    column: u32,
    offset: u32,
    mode: Mode,
    buffer: Vec<char>,
}

impl Lexer {
    /**
     * 初始化。
     * 
     */
    pub fn new() -> Lexer {
        return Lexer {
            row: 0,
            column: 0,
            offset: 0,
            mode: Mode::Normal,
            buffer: Vec::new(),
        };
    }

    /**
     * 分析指定文件的词素。
     * 
     */
    pub fn lex(&mut self, filename: String) -> Vec<Lexeme> {
        let mut lexemes: Vec<Lexeme> = Vec::new();
        for c in read_file_chars(filename) {
            self.locate(c);
            match self.mode {
                Mode::Comment => {
                    self.buffer.push(c);
                    if c == '\n' {
                        lexemes.push(self.as_comment());
                    }
                }
                _ => {
                    if c.is_whitespace() {
                        continue;
                    }
                    if c == '#' {
                        self.offset = self.column;
                        self.mode = Mode::Comment;
                    }
                }
            };
        }
        return lexemes;
    }

    /**
     * 生成一个注释词素。
     * 
     */
    fn as_comment(&mut self) -> Lexeme {
        let text: String = self.buffer.clone().into_iter().collect();
        let lexeme = Lexeme {
            row: self.row,
            column: self.offset,
            token: Token::Identifier(text),
        };
        self.buffer.clear();
        self.mode = Mode::Normal;
        return lexeme;
    }

    /**
     * 定位字符。
     * 
     */
    fn locate(&mut self, c: char) {
        if c == '\n' {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

/**
 * 读取所有的行。
 *
 */
fn read_file_lines(filename: String) -> Lines<BufReader<File>> {
    let path = Path::new(&filename);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("无法打开文件 {} : {}。", display, Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}

/**
 * 读取所有的字符。
 *
 */
fn read_file_chars(filename: String) -> Vec<char> {
    let mut buffer = Vec::new();
    for line in read_file_lines(filename) {
        for c in line.expect("字符有误").chars() {
            buffer.push(c);
        }
        buffer.push('\n');
    }
    return buffer;
}
