use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct AskpassHelper;

impl AskpassHelper {
    /// SUDO_ASKPASS için sistemde çalışan grafiksel parola doğrulama betiğini hazırlar ve yolunu döndürür.
    pub fn ensure_askpass_script() -> Option<String> {
        let home = std::env::var("HOME").ok()?;
        let dir = PathBuf::from(home).join(".config").join("aurora");
        let _ = fs::create_dir_all(&dir);
        let script_path = dir.join("aurora-askpass.sh");

        let script_content = r#"#!/bin/sh
PROMPT="${1:-Aurora paket kurulumu için yönetici parolası gerekli:}"
if [ -n "$KDE_FULL_SESSION" ] && command -v kdialog >/dev/null 2>&1; then
    exec kdialog --password "$PROMPT"
elif command -v zenity >/dev/null 2>&1; then
    exec zenity --password --title="Aurora Yetkilendirme"
elif command -v ksshaskpass >/dev/null 2>&1; then
    exec ksshaskpass "$PROMPT"
elif command -v x11-ssh-askpass >/dev/null 2>&1; then
    exec x11-ssh-askpass "$PROMPT"
fi
"#;

        if fs::write(&script_path, script_content).is_ok() {
            let _ = fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755));
            Some(script_path.to_string_lossy().to_string())
        } else {
            None
        }
    }
}
