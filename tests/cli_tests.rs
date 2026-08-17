use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn get_clb_bin() -> String {
    env!("CARGO_BIN_EXE_clb").to_string()
}

fn create_temp_env() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

#[test]
fn test_copy_and_paste_text() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    // Copy text
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["copy", "Hello", "World"])
        .output()
        .expect("failed to execute copy");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Copied text \"Hello World\""));

    // Paste text
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["paste"])
        .output()
        .expect("failed to execute paste");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello World");
}

#[test]
fn test_pipe_in_and_pipe_out() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    // Pipe in to clb
    let mut child = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to spawn clb");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin.write_all(b"piped text content\n").unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    // Pipe out from clb
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["paste"])
        .stdout(Stdio::piped())
        .output()
        .expect("failed to run clb pipe out");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "piped text content\n"
    );
}

#[test]
fn test_copy_and_paste_files() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let work_dir = tmp.path().join("work");
    fs::create_dir_all(&work_dir).unwrap();

    let src_file1 = work_dir.join("test1.txt");
    let src_file2 = work_dir.join("test2.txt");
    fs::write(&src_file1, "file 1 content").unwrap();
    fs::write(&src_file2, "file 2 content").unwrap();

    // Copy files
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&work_dir)
        .args(["copy", "test1.txt", "test2.txt"])
        .output()
        .expect("failed to execute copy");
    assert!(output.status.success());

    let dest_dir = tmp.path().join("dest");
    fs::create_dir_all(&dest_dir).unwrap();

    // Paste files into dest_dir
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&dest_dir)
        .args(["paste"])
        .output()
        .expect("failed to execute paste");
    assert!(output.status.success());

    assert!(dest_dir.join("test1.txt").exists());
    assert!(dest_dir.join("test2.txt").exists());
    assert_eq!(
        fs::read_to_string(dest_dir.join("test1.txt")).unwrap(),
        "file 1 content"
    );
    // Source files still exist
    assert!(src_file1.exists());
    assert!(src_file2.exists());
}

#[test]
fn test_cut_and_paste_files() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let work_dir = tmp.path().join("work_cut");
    fs::create_dir_all(&work_dir).unwrap();

    let src_file = work_dir.join("cut_me.txt");
    fs::write(&src_file, "cut content").unwrap();

    // Cut file
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&work_dir)
        .args(["cut", "cut_me.txt"])
        .output()
        .expect("failed to execute cut");
    assert!(output.status.success());

    let dest_dir = tmp.path().join("dest_cut");
    fs::create_dir_all(&dest_dir).unwrap();

    // Paste into dest_dir
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&dest_dir)
        .args(["paste"])
        .output()
        .expect("failed to execute paste");
    assert!(output.status.success());

    assert!(dest_dir.join("cut_me.txt").exists());
    assert_eq!(
        fs::read_to_string(dest_dir.join("cut_me.txt")).unwrap(),
        "cut content"
    );
    // Original file should now be removed
    assert!(!src_file.exists());
}

#[test]
fn test_move_command() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let work_dir = tmp.path().join("work_move");
    fs::create_dir_all(&work_dir).unwrap();

    let src_file = work_dir.join("move_me.txt");
    fs::write(&src_file, "move content").unwrap();

    // First COPY the file
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&work_dir)
        .args(["copy", "move_me.txt"])
        .output()
        .expect("failed to copy");
    assert!(output.status.success());
    assert!(src_file.exists());

    let dest_dir = tmp.path().join("dest_move");
    fs::create_dir_all(&dest_dir).unwrap();

    // Now MOVE to dest_dir (treat last copy as cut, then paste)
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&dest_dir)
        .args(["move"])
        .output()
        .expect("failed to move");
    assert!(output.status.success());

    assert!(dest_dir.join("move_me.txt").exists());
    assert_eq!(
        fs::read_to_string(dest_dir.join("move_me.txt")).unwrap(),
        "move content"
    );
    // Source should be removed!
    assert!(!src_file.exists());
}

#[test]
fn test_clear_command() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    // Copy text
    Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["copy", "to be cleared"])
        .output()
        .unwrap();

    // Clear
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["clear"])
        .output()
        .expect("failed to clear");
    assert!(output.status.success());

    // Info should show empty
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["info", "--json"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"contentType\": \"empty\""));
}

#[test]
fn test_info_and_status() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["copy", "sample test data"])
        .output()
        .unwrap();

    // ASCII info
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["info"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("+--- Clipboard 1"));
    assert!(stdout.contains("sample test data"));

    // JSON info
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["info", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["name"], "1");
    assert_eq!(v["contentType"], "text/plain");

    // Status overview
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["status"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All your clipboards with content"));
    assert!(stdout.contains("1 | sample test data"));

    // Status JSON
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["status", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["1"], "sample test data");
}

#[test]
fn test_show_command() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["copy", "show content test"])
        .output()
        .unwrap();

    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["show"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "show content test"
    );
}

#[test]
fn test_config_command() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["config"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("+--- Configuration"));
    assert!(stdout.contains("7-bit ascii"));

    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["config", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["temporaryDirectory"], tmp_path);
}

#[test]
fn test_completion_command() {
    let output = Command::new(get_clb_bin())
        .args(["completion", "bash"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("_clb"));

    let output = Command::new(get_clb_bin())
        .args(["completion", "zsh"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#compdef clb"));
}

#[test]
fn test_placeholders() {
    let output = Command::new(get_clb_bin())
        .args(["import"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("is deferred and not implemented yet"));

    let output = Command::new(get_clb_bin()).args(["undo"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("is not implemented yet"));
}

#[test]
fn test_edit_command() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    // Create a mock editor script that appends "-edited" to the file
    let script_path = tmp.path().join("mock_editor.sh");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::write(&script_path, "#!/bin/sh\necho \" - edited\" >> \"$1\"\n").unwrap();
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }

    // Copy initial text
    Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["copy", "original"])
        .output()
        .unwrap();

    // Run edit with mock editor
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .env("CLIPBOARD_EDITOR", script_path.to_str().unwrap())
        .args(["edit"])
        .output()
        .expect("failed to run edit");
    assert!(output.status.success());

    // Verify edited text
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["paste"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("original"));
    assert!(stdout.contains("- edited"));

    // Test canceled edit (editor exits with 1)
    let cancel_script = tmp.path().join("cancel_editor.sh");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::write(
            &cancel_script,
            "#!/bin/sh\necho \"should not be saved\" >> \"$1\"\nexit 1\n",
        )
        .unwrap();
        let mut perms = fs::metadata(&cancel_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&cancel_script, perms).unwrap();
    }

    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .env("CLIPBOARD_EDITOR", cancel_script.to_str().unwrap())
        .args(["edit"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // Verify clipboard was untouched after cancel
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["paste"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("should not be saved"));
}

#[test]
fn test_subcommand_aliases() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    // 'cp' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["cp", "alias test text"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 'p' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["p"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "alias test text"
    );

    // 'c' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["c", "alias c text"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 'sh' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["sh"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "alias c text"
    );

    // 'in' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["in"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 'st' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["st"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 'cfg' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["cfg"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // 'clr' alias
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["clr"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_directory_copy_and_paste() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let work_dir = tmp.path().join("work_dir");
    let nested_dir = work_dir.join("subdir");
    fs::create_dir_all(&nested_dir).unwrap();
    fs::write(nested_dir.join("subfile.txt"), "nested file content").unwrap();

    // Copy directory
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&work_dir)
        .args(["copy", "subdir"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let dest_dir = tmp.path().join("dest_dir");
    fs::create_dir_all(&dest_dir).unwrap();

    // Paste directory
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&dest_dir)
        .args(["paste"])
        .output()
        .unwrap();
    assert!(output.status.success());

    assert!(dest_dir.join("subdir/subfile.txt").exists());
    assert_eq!(
        fs::read_to_string(dest_dir.join("subdir/subfile.txt")).unwrap(),
        "nested file content"
    );
}

#[test]
fn test_regex_filter_paste() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();

    let work_dir = tmp.path().join("work_regex");
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(work_dir.join("alpha.txt"), "alpha").unwrap();
    fs::write(work_dir.join("beta.log"), "beta").unwrap();
    fs::write(work_dir.join("gamma.txt"), "gamma").unwrap();

    // Copy files
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&work_dir)
        .args(["copy", "alpha.txt", "beta.log", "gamma.txt"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let dest_dir = tmp.path().join("dest_regex");
    fs::create_dir_all(&dest_dir).unwrap();

    // Paste with regex pattern matching only *.txt
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_NOGUI", "1")
        .current_dir(&dest_dir)
        .args(["paste", ".*\\.txt$"])
        .output()
        .unwrap();
    assert!(output.status.success());

    assert!(dest_dir.join("alpha.txt").exists());
    assert!(dest_dir.join("gamma.txt").exists());
    assert!(!dest_dir.join("beta.log").exists());
}

#[test]
fn test_custom_clipboard_and_persistence() {
    let tmp = create_temp_env();
    let tmp_path = tmp.path().to_str().unwrap();
    let persist_path = tmp.path().join("persist");
    let persist_str = persist_path.to_str().unwrap();

    // Custom named clipboard
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_PERSISTDIR", persist_str)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["-c", "myclip", "copy", "hello custom"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // Verify retrieval from custom clipboard
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_PERSISTDIR", persist_str)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["-c", "myclip", "paste"])
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "hello custom"
    );

    // Persistent clipboard (containing underscore)
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_PERSISTDIR", persist_str)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["-c", "persist_one", "copy", "hello persist"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // Check info to verify isPersistent is true
    let output = Command::new(get_clb_bin())
        .env("CLIPBOARD_TMPDIR", tmp_path)
        .env("CLIPBOARD_PERSISTDIR", persist_str)
        .env("CLIPBOARD_NOGUI", "1")
        .args(["-c", "persist_one", "info", "--json"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["isPersistent"], true);
}
