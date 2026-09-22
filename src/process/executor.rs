use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Debug, Clone)]
pub enum ProcessMessage {
    Stdout(String),
    Stderr(String),
    Finished(bool, Option<i32>),
}

pub struct CommandExecutor;

impl CommandExecutor {
    /// Asenkron komut koşturucu
    /// UI thread'ini asla bloklamaz, her satırı glib main context üzerinden gönderilen callback ile UI'a iletir.
    pub async fn run_streaming<F>(
        program: &str,
        args: &[String],
        callback: F,
    ) -> Result<bool, std::io::Error>
    where
        F: Fn(ProcessMessage) + Send + Sync + 'static,
    {
        let (_tx, rx) = tokio::sync::watch::channel(false);
        Self::run_streaming_cancellable(program, args, rx, callback).await
    }

    /// Asenkron komut koşturucu (İptal edilebilir)
    /// UI thread'ini asla bloklamaz, cancel_rx üzerinden gelen iptal sinyaliyle child process'i derhal sonlandırır.
    pub async fn run_streaming_cancellable<F>(
        program: &str,
        args: &[String],
        mut cancel_rx: tokio::sync::watch::Receiver<bool>,
        callback: F,
    ) -> Result<bool, std::io::Error>
    where
        F: Fn(ProcessMessage) + Send + Sync + 'static,
    {
        tracing::info!("Komut başlatılıyor: {} {:?}", program, args);

        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if let Some(askpass_path) = crate::utils::AskpassHelper::ensure_askpass_script() {
            cmd.env("SUDO_ASKPASS", askpass_path);
        }

        let mut child = cmd.spawn()?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let cb_out = std::sync::Arc::new(callback);
        let cb_err = cb_out.clone();

        let mut handles = Vec::new();

        if let Some(stdout) = stdout {
            let cb = cb_out.clone();
            handles.push(tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    cb(ProcessMessage::Stdout(line));
                }
            }));
        }

        if let Some(stderr) = stderr {
            let cb = cb_err.clone();
            handles.push(tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    cb(ProcessMessage::Stderr(line));
                }
            }));
        }

        let mut cancelled = *cancel_rx.borrow();

        let (success, exit_code) = if cancelled {
            tracing::warn!("İşlem başlatılmadan önce iptal edildi: {}", program);
            let _ = child.kill().await;
            cb_out(ProcessMessage::Stderr("İşlem kullanıcı tarafından iptal edildi.".to_string()));
            (false, None)
        } else {
            tokio::select! {
                res = child.wait() => {
                    let status = res?;
                    (status.success(), status.code())
                }
                res = cancel_rx.changed() => {
                    if res.is_ok() && *cancel_rx.borrow() {
                        cancelled = true;
                        tracing::warn!("İşlem yürütülürken iptal edildi, child process sonlandırılıyor: {}", program);
                        let _ = child.kill().await;
                        cb_out(ProcessMessage::Stderr("İşlem kullanıcı tarafından iptal edildi.".to_string()));
                        (false, None)
                    } else {
                        let status = child.wait().await?;
                        (status.success(), status.code())
                    }
                }
            }
        };

        if cancelled {
            for handle in handles {
                handle.abort();
            }
        } else {
            for handle in handles {
                let _ = handle.await;
            }
        }

        cb_out(ProcessMessage::Finished(success, exit_code));
        Ok(success)
    }

    /// Çıktıyı tam ExitStatus ile birlikte alma (çıkış kodu duyarlı komutlar için örn. checkupdates)
    pub async fn run_captured_with_status(
        program: &str,
        args: &[&str],
    ) -> Result<(std::process::ExitStatus, String, String), std::io::Error> {
        let output = Command::new(program)
            .args(args)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok((output.status, stdout, stderr))
    }

    /// Çıktıyı tek seferde string olarak alma (hızlı sorgular için)
    pub async fn run_captured(program: &str, args: &[&str]) -> Result<(bool, String, String), std::io::Error> {
        let (status, stdout, stderr) = Self::run_captured_with_status(program, args).await?;
        Ok((status.success(), stdout, stderr))
    }
}
