use super::ieditor::*;
use crate::{error::Error, Result};
use crossterm::{cursor, event, execute, style, style::Colorize, terminal};
use defer::defer;
use std::io::{BufRead, Write};

pub trait Searchable {
    fn is_match(&self, pat: &String) -> bool;
}

pub enum Direction {
    #[allow(dead_code)]
    Vertical,
    Horizontal,
}

pub struct Core<R, W> {
    reader: R,
    writer: W,
    prompt: &'static str,
    visiable_nums: usize,
}

impl<R, W> Core<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Core {
            reader,
            writer,
            prompt: ">>>",
            visiable_nums: 5,
        }
    }

    #[allow(dead_code)]
    pub fn prompt(&mut self, prompt: &'static str) -> &mut Self {
        self.prompt = prompt;
        self
    }

    #[allow(dead_code)]
    pub fn visiable_nums(&mut self, visiable_nums: usize) -> &mut Self {
        self.visiable_nums = visiable_nums;
        self
    }

    pub fn question(&mut self, question: &'static str, tips: &'static str) -> Result<()> {
        writeln!(
            self.writer,
            "{} {} {}",
            self.prompt.green(),
            question.white(),
            tips.blue()
        )?;
        write!(self.writer, "❯ ")?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn read_input_from<T>(&mut self, list: Vec<T>, direction: Direction) -> Result<T>
    where
        T: std::fmt::Display + Clone + Searchable,
    {
        assert!(!list.is_empty());
        let mut cur: i32 = 0;
        let mut start: i32 = 0;
        let mut visiable_nums = std::cmp::min(self.visiable_nums, list.len()) as i32;
        let visiable_lines = match direction {
            Direction::Vertical => visiable_nums,
            Direction::Horizontal => 1,
        };
        let mut list_str = vec![];
        let mut list_idx: Vec<_> = (0..list.len()).collect();
        for item in &list {
            list_str.push(format!("❯ {:8}\t", item));
        }
        let move_cursor = |cursor: &mut i32, step: i32, max: usize| {
            if max == 0 {
                *cursor = 0;
                return;
            }
            *cursor += step;
            while *cursor < 0i32 {
                *cursor += max as i32;
            }
            *cursor = *cursor % (max as i32);
        };
        for _ in 0..(visiable_lines + 1) {
            writeln!(self.writer, "")?;
        }
        self.writer.flush()?;
        let (begin_x, end_y) = cursor::position()?;
        let begin_y = end_y - 1 - visiable_lines as u16;
        let mut search_buf = String::with_capacity(16);
        let mut search = false;
        let mut dirty = false;
        loop {
            if dirty {
                list_idx = (0..list.len())
                    .filter(|i| list[*i].is_match(&search_buf))
                    .collect();
                visiable_nums = std::cmp::min(self.visiable_nums, list_idx.len()) as i32;
                dirty = false;
            }
            execute!(
                self.writer,
                cursor::MoveTo(begin_x, begin_y),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )?;
            let at_start = cur == start;
            let mut scroll = false;
            let mut cur_y = begin_y;
            let mut cur_x = begin_x;
            for i in 0..visiable_nums {
                let display_idx = (i + start) as usize % list_idx.len();
                let idx = list_idx[display_idx];
                let item = &list_str[idx as usize];
                if cur == display_idx as i32 {
                    if i == (visiable_nums - 1) / 2 {
                        scroll = true;
                    }
                    let (new_x, new_y) = cursor::position()?;
                    cur_x = new_x;
                    cur_y = new_y;
                    execute!(self.writer, style::Print(item[..].black().on_yellow()),)?;
                } else {
                    execute!(self.writer, style::Print(item),)?;
                }
                match direction {
                    Direction::Vertical => execute!(self.writer, cursor::MoveToNextLine(1))?,
                    Direction::Horizontal if i == visiable_nums - 1 => {
                        execute!(self.writer, cursor::MoveToNextLine(1))?
                    }
                    _ => {}
                }
            }
            if search {
                execute!(
                    self.writer,
                    style::Print(format!("Search: {}", search_buf).dark_blue()),
                )?;
            } else {
                execute!(
                    self.writer,
                    style::Print("(*Move [ArrowKey/Tab], *Confirm [Enter])".dark_grey()),
                    cursor::MoveTo(cur_x, cur_y)
                )?;
            }

            terminal::enable_raw_mode()?;
            let reset = defer(|| {
                terminal::disable_raw_mode().unwrap();
            });

            loop {
                match event::read()? {
                    event::Event::Key(event) => match event.code {
                        event::KeyCode::Up | event::KeyCode::BackTab | event::KeyCode::Left => {
                            search = false;
                            move_cursor(&mut cur, -1, list_idx.len());
                            if scroll || at_start {
                                move_cursor(&mut start, -1, list_idx.len());
                            }
                            break;
                        }
                        event::KeyCode::Down | event::KeyCode::Tab | event::KeyCode::Right => {
                            search = false;
                            move_cursor(&mut cur, 1, list_idx.len());
                            if scroll {
                                move_cursor(&mut start, 1, list_idx.len());
                            }
                            break;
                        }
                        event::KeyCode::Enter => {
                            assert!(!dirty);
                            execute!(
                                self.writer,
                                cursor::MoveTo(begin_x, begin_y),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                                style::Print(format!("{}", &list[list_idx[cur as usize]])),
                                cursor::MoveToNextLine(1),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                            )?;
                            return Ok(list[list_idx[cur as usize]].clone());
                        }
                        event::KeyCode::Char('c')
                            if event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            drop(reset);
                            return Err(Error::StringError("Terminate by Ctrl-C".to_owned()));
                        }
                        event::KeyCode::Esc => {
                            if search {
                                search = false;
                                search_buf.clear();
                                dirty = true;
                            }
                            break;
                        }
                        event::KeyCode::Backspace => {
                            if search {
                                start = 0;
                                cur = 0;
                                search_buf.pop();
                                dirty = true;
                                if search_buf.is_empty() {
                                    search = false;
                                }
                            }
                            break;
                        }
                        event::KeyCode::Char(c) => {
                            search = true;
                            start = 0;
                            cur = 0;
                            search_buf.push(c);
                            dirty = true;
                            break;
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            }
        }
    }

    pub fn read_input_with(&mut self, hint: Option<String>) -> Result<Option<String>> {
        let (begin_x, mut begin_y) = cursor::position()?;
        let (w, _) = terminal::size()?;
        let ydist = |n: u16, w: u16, begin: u16| {
            if n > (w - begin) {
                1u16 + (n - w + begin) / w
            } else {
                0u16
            }
        };
        match hint {
            Some(ref hint) => {
                if let Some(n) = hint.find('\n') {
                    begin_y = begin_y - ydist((n + 3) as u16, w, begin_x);
                    write!(self.writer, "{}...", hint[..n].dark_grey())?
                } else {
                    begin_y = begin_y - ydist(hint.len() as u16, w, begin_x);
                    write!(self.writer, "{}", hint[..].dark_grey())?
                }
            }
            None => {}
        }
        execute!(self.writer, cursor::MoveTo(begin_x, begin_y))?;

        match self.read_input()? {
            None => Ok(hint),
            input @ _ => Ok(input),
        }
    }

    pub fn read_input(&mut self) -> Result<Option<String>> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        let postbuf = buf.trim();
        Ok(if postbuf == "" || postbuf == "\n" {
            None
        } else if postbuf.starts_with("!") {
            read_from_editor(postbuf[1..].trim())?
        } else {
            Some(postbuf.to_string())
        })
    }
}

impl Searchable for char {
    fn is_match(&self, _: &String) -> bool {
        false
    }
}

impl Searchable for String {
    fn is_match(&self, pat: &String) -> bool {
        self.contains(pat)
    }
}

impl Searchable for &str {
    fn is_match(&self, pat: &String) -> bool {
        self.contains(pat)
    }
}
