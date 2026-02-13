use anyhow::Result;
use std::{
    io::Write,
    os::unix::net::UnixStream,
    process::{Child, Command, Stdio},
};

const SOCKET: &str = "/tmp/music-tui-mpv";

pub struct Player {
    pub process: Option<Child>,
}

impl Player {
    pub fn new() -> Self {
        Self { process: None }
    }
    pub fn play(&mut self, url: &str) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
        let child = Command::new("mpv")
            .arg(url)
            .arg("--no-video")
            .arg("--quiet")
            .arg(format!("--input-ipc-server={}", SOCKET))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.process = Some(child);
        Ok(())
    }
    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
    }
    pub fn toggle_pause(&self) -> Result<()> {
        let mut socket = UnixStream::connect(SOCKET)?;

        let cmd = r#"{ "command": ["cycle", "pause"] }"#; // this is mpv json IPC Command

        socket.write_all(cmd.as_bytes())?;
        socket.write_all(b"\n")?;
        Ok(())
    }
}
