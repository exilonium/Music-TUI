use anyhow::Result;
use std::thread;
use std::time::{Duration, Instant};
use std::{
    io::Write,
    os::unix::net::UnixStream,
    process::{Child, Command, Stdio},
};

const SOCKET: &str = "/tmp/music-tui-mpv";

pub struct Player {
    pub process: Option<Child>,
    pub socket: Option<UnixStream>,
    pub playing: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            process: None,
            socket: None,
            playing: false,
        }
    }

    pub fn play(&mut self, url: &str) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = std::fs::remove_file(SOCKET);
        }
        let child = Command::new("mpv")
            .arg(url)
            .arg("--no-video")
            .arg("--quiet")
            .arg(format!("--input-ipc-server={}", SOCKET))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        let socket = Self::connect_socket()?;
        self.process = Some(child);
        self.socket = Some(socket);
        self.playing = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
        self.socket = None;
        self.playing = false;
    }

    pub fn toggle_pause(&mut self) -> Result<()> {
        let socket = self
            .socket
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("mpv is not connected"))?;
        let cmd = r#"{"command": ["cycle", "pause"]}"#;
        socket.write_all(cmd.as_bytes())?;
        socket.write_all(b"\n")?;
        self.playing = !self.playing;
        Ok(())
    }

    pub fn seek(&mut self, forward: bool) -> Result<()> {
        let socket = self
            .socket
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("mpv is not connected"))?;
        let cmd = if forward {
            r#"{"command": ["seek", 5, "relative"]}"#
        } else {
            r#"{"command": ["seek", -5, "relative"]}"#
        };
        socket.write_all(cmd.as_bytes())?;
        socket.write_all(b"\n")?;
        Ok(())
    }

    fn connect_socket() -> Result<UnixStream> {
        let start = Instant::now();
        let timeout = Duration::from_secs(2);
        while start.elapsed() < timeout {
            match UnixStream::connect(SOCKET) {
                Ok(sock) => return Ok(sock),
                Err(_) => thread::sleep(Duration::from_millis(50)),
            }
        }
        Err(anyhow::anyhow!("Failed to connect to mpv IPC socket"))
    }
}
