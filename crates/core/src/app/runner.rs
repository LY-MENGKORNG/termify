use crate::service::Terminal;

pub struct Runner {
    terminal: Terminal,
}

impl Runner {
    pub fn new(terminal: Terminal) -> Self {
        Self { terminal }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        self.draw()?;
        Ok(())
    }

    fn draw(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
