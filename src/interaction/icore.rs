use crossterm::{cursor, event, execute, style, style::Colorize, terminal, Result};
use defer::defer;
use std::io::{BufRead, Write};

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
        self.writer.flush()?;
        Ok(())
    }

    pub fn read_input_from<T>(&mut self, list: Vec<T>, direction: Direction) -> Result<T>
    where
        T: std::fmt::Display + Clone,
    {
        assert!(!list.is_empty());
        let mut cur: i32 = 0;
        let mut start: i32 = 0;
        let visiable_nums = std::cmp::min(self.visiable_nums, list.len()) as i32;
        let visiable_lines = match direction {
            Direction::Vertical => visiable_nums,
            Direction::Horizontal => 1,
        };
        let move_cursor = |cursor: &mut i32, step: i32| {
            *cursor += step;
            while *cursor < 0i32 {
                *cursor += list.len() as i32;
            }
            *cursor = *cursor % (list.len() as i32);
        };
        execute!(
            self.writer,
            cursor::MoveToNextLine(visiable_lines as u16 + 1)
        )?;
        let (begin_x, end_y) = cursor::position()?;
        let begin_y = end_y - 1 - visiable_lines as u16;
        loop {
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
                let idx = (i + start) % list.len() as i32;
                let item = format!("❯ {:8}\t", list[idx as usize]);
                if idx == cur {
                    if i == (visiable_nums - 1) / 2 {
                        scroll = true;
                    }
                    let (new_x, new_y) = cursor::position()?;
                    cur_x = new_x;
                    cur_y = new_y;
                    execute!(
                        self.writer,
                        style::SetForegroundColor(style::Color::Black),
                        style::SetBackgroundColor(style::Color::Yellow),
                        style::Print(item),
                    )?;
                } else {
                    execute!(
                        self.writer,
                        style::SetForegroundColor(style::Color::Reset),
                        style::SetBackgroundColor(style::Color::Reset),
                        style::Print(item),
                    )?;
                }
                match direction {
                    Direction::Vertical => execute!(self.writer, cursor::MoveToNextLine(1))?,
                    Direction::Horizontal if i == visiable_nums - 1 => {
                        execute!(self.writer, cursor::MoveToNextLine(1))?
                    }
                    _ => {}
                }
            }
            execute!(
                self.writer,
                style::Print("(*Move [ArrowKey/Tab], *Confirm [Enter])".dark_grey()),
                cursor::MoveTo(cur_x, cur_y)
            )?;

            terminal::enable_raw_mode()?;
            let reset = defer(|| {
                terminal::disable_raw_mode().unwrap();
            });

            loop {
                match event::read()? {
                    event::Event::Key(event) => match event.code {
                        event::KeyCode::Up | event::KeyCode::BackTab | event::KeyCode::Left => {
                            move_cursor(&mut cur, -1);
                            if scroll || at_start {
                                move_cursor(&mut start, -1);
                            }
                            break;
                        }
                        event::KeyCode::Down | event::KeyCode::Tab | event::KeyCode::Right => {
                            move_cursor(&mut cur, 1);
                            if scroll {
                                move_cursor(&mut start, 1);
                            }
                            break;
                        }
                        event::KeyCode::Enter => {
                            execute!(
                                self.writer,
                                cursor::MoveTo(begin_x, begin_y),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                                style::Print(format!("{}", &list[cur as usize])),
                                cursor::MoveToNextLine(1),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                            )?;
                            return Ok(list[cur as usize].clone());
                        }
                        event::KeyCode::Char('c')
                            if event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            drop(reset);
                            panic!("Terminate by Ctrl-C");
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            }
        }
    }

    pub fn read_input(&mut self) -> Result<Option<String>> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        let postbuf = buf.trim();
        Ok(if postbuf == "" {
            None
        } else {
            Some(postbuf.to_string())
        })
    }
}
