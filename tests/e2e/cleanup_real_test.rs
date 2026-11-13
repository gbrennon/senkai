use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tempfile::TempDir;

struct RepoGuard {
    _dir: TempDir,
    root: PathBuf,
}

impl RepoGuard {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("repo");
        std::fs::create_dir(&root).unwrap();
        run_command("git", &["init", "-q", "-b", "main"], Some(&root));
        run_command(
            "git",
            &["config", "user.email", "e2e@example.invalid"],
            Some(&root),
        );
        run_command("git", &["config", "user.name", "e2e"], Some(&root));
        std::fs::write(root.join("README"), "e2e\n").unwrap();
        run_command("git", &["add", "README"], Some(&root));
        run_command("git", &["commit", "-qm", "initial"], Some(&root));
        run_command(
            "git",
            &["remote", "add", "origin", root.to_str().unwrap()],
            Some(&root),
        );
        std::fs::create_dir(root.join(".worktrees")).unwrap();
        for (branch, directory) in [
            ("feat/e2e-a", ".worktrees/feat-e2e-a"),
            ("feat/e2e-b", ".worktrees/feat-e2e-b"),
        ] {
            run_command(
                "git",
                &["worktree", "add", "-q", "-b", branch, directory],
                Some(&root),
            );
        }
        Self { _dir: dir, root }
    }
}

struct TmuxGuard {
    socket: String,
    socket_path: String,
}

impl TmuxGuard {
    fn new(root: &Path, binary: &Path) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let socket = format!("senkai-e2e-{}-{nonce}", std::process::id());
        let _ = tmux_command(&socket, &["kill-server"]);
        let root = root.to_str().unwrap();
        let binary = binary.to_str().unwrap();
        let output = tmux_command(&socket, &["new-session", "-d", "-s", "cleanup", "-c", root]);
        assert_success(&output, "start cleanup tmux session");
        let socket_path = socket_path(&socket, "cleanup:0");
        let guard = Self {
            socket,
            socket_path,
        };
        guard.run_tui_cleanup(root, binary, "cleanup:0");
        guard
    }

    fn start_worktree(root: &Path, binary: &Path, branch: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let socket = format!("senkai-e2e-{}-{nonce}", std::process::id());
        let _ = tmux_command(&socket, &["kill-server"]);
        let root = root.to_str().unwrap();
        let output = tmux_command(
            &socket,
            &["new-session", "-d", "-s", "worktree", "-c", root],
        );
        assert_success(&output, "start worktree tmux session");
        let socket_path = socket_path(&socket, "worktree:0");
        let guard = Self {
            socket,
            socket_path,
        };
        guard.run_choose(root, binary, branch, "worktree:0");
        guard
    }

    fn open_worktree(&self, root: &Path, binary: &Path, branch: &str) {
        let root = root.to_str().unwrap();
        let output = tmux_command(
            &self.socket,
            &[
                "new-window",
                "-d",
                "-t",
                "worktree:",
                "-n",
                "invoke",
                "-c",
                root,
            ],
        );
        assert_success(&output, "start resume worktree command");
        self.run_choose(root, binary, branch, "worktree:invoke");
    }

    fn run_tui_cleanup(&self, root: &str, binary: &str, target: &str) {
        let command = format!(
            "env -u TMUX_PANE TMUX={},0,0 {} tui cleanup --root {root}",
            self.socket_path, binary
        );
        self.send_command(target, &command);
    }

    fn run_choose(&self, root: &str, binary: &Path, branch: &str, target: &str) {
        let command = format!(
            "env -u TMUX_PANE TMUX={},0,0 {} choose {branch} --root {root}",
            self.socket_path,
            binary.to_str().unwrap()
        );
        self.send_command(target, &command);
    }

    fn send_command(&self, target: &str, command: &str) {
        let output = tmux_command(&self.socket, &["send-keys", "-t", target, "-l", command]);
        assert_success(&output, "type command in tmux");
        let output = tmux_command(&self.socket, &["send-keys", "-t", target, "Enter"]);
        assert_success(&output, "submit command in tmux");
    }

    fn windows(&self) -> String {
        let output = tmux_command(
            &self.socket,
            &["list-windows", "-F", "#{window_name}|#{pane_current_path}"],
        );
        assert_success(&output, "list worktree windows");
        String::from_utf8(output.stdout).unwrap()
    }

    fn capture_target(&self, target: &str) -> String {
        let output = tmux_command(&self.socket, &["capture-pane", "-p", "-t", target]);
        assert_success(&output, "capture worktree pane");
        String::from_utf8(output.stdout).unwrap()
    }

    fn active_window(&self) -> String {
        let output = tmux_command(
            &self.socket,
            &[
                "display-message",
                "-p",
                "-t",
                "worktree:",
                "#{window_name}|#{pane_current_path}",
            ],
        );
        assert_success(&output, "resolve active worktree window");
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    fn wait_for_path(&self, path: &Path) -> String {
        let expected = path.to_string_lossy();
        for _ in 0..100 {
            let windows = self.windows();
            if windows
                .lines()
                .any(|line| line.ends_with(expected.as_ref()))
            {
                return windows;
            }
            sleep(Duration::from_millis(50));
        }
        let root = path.parent().and_then(Path::parent).unwrap();
        let worktrees = run_command("git", &["worktree", "list", "--porcelain"], Some(root));
        let branches = run_command("git", &["branch", "--format=%(refname:short)"], Some(root));
        panic!(
            "worktree path never appeared {expected:?}:\n{}\n{}\n{}\n{}",
            self.windows(),
            self.capture_target("worktree:0"),
            String::from_utf8_lossy(&worktrees.stdout),
            String::from_utf8_lossy(&branches.stdout)
        );
    }

    fn send_key(&self, key: &str) {
        let output = tmux_command(&self.socket, &["send-keys", "-t", "cleanup:0", key]);
        assert_success(&output, "send tmux key");
    }

    fn capture(&self) -> String {
        let output = tmux_command(&self.socket, &["capture-pane", "-p", "-t", "cleanup:0"]);
        assert_success(&output, "capture cleanup pane");
        String::from_utf8(output.stdout).unwrap()
    }

    fn wait_for(&self, expected: &str) -> String {
        for _ in 0..100 {
            let pane = self.capture();
            if pane.contains(expected) {
                return pane;
            }
            sleep(Duration::from_millis(50));
        }
        panic!(
            "cleanup pane never contained {expected:?}:\n{}",
            self.capture()
        );
    }
}

impl Drop for TmuxGuard {
    fn drop(&mut self) {
        let _ = tmux_command(&self.socket, &["kill-server"]);
    }
}

fn binary_path() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_sk")
        .map(PathBuf::from)
        .expect("Cargo did not provide the sk binary path")
}

fn socket_path(socket: &str, target: &str) -> String {
    let output = tmux_command(
        socket,
        &["display-message", "-p", "-t", target, "#{socket_path}"],
    );
    assert_success(&output, "resolve tmux socket path");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn tmux_command(socket: &str, args: &[&str]) -> Output {
    Command::new("tmux")
        .args(["-L", socket, "-f", "/dev/null"])
        .args(args)
        .output()
        .unwrap()
}

fn run_command(program: &str, args: &[&str], cwd: Option<&Path>) -> Output {
    let mut command = Command::new(program);
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output().unwrap();
    assert_success(&output, program);
    output
}

fn tmux_available() -> bool {
    Command::new("tmux")
        .arg("-V")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn assert_success(output: &Output, action: &str) {
    assert!(
        output.status.success(),
        "{action} failed: status={} stdout={} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn binary_entrypoint_creates_and_resumes_a_worktree_without_tui() {
    if !tmux_available() {
        eprintln!("skipping real tmux test: tmux is unavailable");
        return;
    }
    let repo = RepoGuard::new();
    let binary = binary_path();
    let root = repo.root.to_str().unwrap();
    let create = Command::new(&binary)
        .args(["choose", "feat/e2e-c", "--root", root])
        .output()
        .unwrap();
    assert_success(&create, "create worktree without tui");

    let resume = Command::new(&binary)
        .args(["choose", "feat/e2e-c", "--root", root])
        .output()
        .unwrap();
    assert_success(&resume, "resume worktree without tui");

    let worktrees = run_command(
        "git",
        &["worktree", "list", "--porcelain"],
        Some(&repo.root),
    );
    let output = String::from_utf8(worktrees.stdout).unwrap();
    assert_eq!(output.matches("feat-e2e-c").count(), 1);
}

#[test]
fn binary_entrypoint_changes_to_created_and_resumed_worktree_path() {
    if !tmux_available() {
        eprintln!("skipping real tmux test: tmux is unavailable");
        return;
    }
    let repo = RepoGuard::new();
    let binary = binary_path();
    let target = repo.root.join(".worktrees/feat-e2e-c");
    let tmux = TmuxGuard::start_worktree(&repo.root, &binary, "feat/e2e-c");

    let created = tmux.wait_for_path(&target);
    assert_eq!(
        created
            .lines()
            .filter(|line| line.ends_with(target.to_string_lossy().as_ref()))
            .count(),
        1
    );
    let active = tmux.active_window();
    assert!(
        active.ends_with(target.to_string_lossy().as_ref()),
        "active window: {active}"
    );

    tmux.open_worktree(&repo.root, &binary, "feat/e2e-c");
    let resumed = tmux.wait_for_path(&target);
    assert_eq!(
        resumed
            .lines()
            .filter(|line| line.ends_with(target.to_string_lossy().as_ref()))
            .count(),
        1
    );
    let active = tmux.active_window();
    assert!(
        active.ends_with(target.to_string_lossy().as_ref()),
        "active window: {active}"
    );
}

#[test]
fn binary_entrypoint_removes_two_worktrees_after_confirmation() {
    if !tmux_available() {
        eprintln!("skipping real tmux test: tmux is unavailable");
        return;
    }
    let repo = RepoGuard::new();
    let binary = binary_path();
    let tmux = TmuxGuard::new(&repo.root, &binary);

    let initial = tmux.wait_for("Space to toggle, Enter to continue, Esc to cancel");
    assert!(initial.contains("[ ] ✗ active  | feat/e2e-a"));
    assert!(initial.contains("[ ] ✗ active  | feat/e2e-b"));

    tmux.send_key("Space");
    tmux.send_key("Down");
    tmux.send_key("Space");
    tmux.send_key("Enter");
    let confirmation = tmux.wait_for("Confirm removal>");
    assert!(confirmation.contains("Remove 2 worktree(s): feat/e2e-a, feat/e2e-b"));

    tmux.send_key("Enter");

    for _ in 0..100 {
        let worktrees = run_command(
            "git",
            &["worktree", "list", "--porcelain"],
            Some(&repo.root),
        );
        let branches = run_command(
            "git",
            &["branch", "--format=%(refname:short)"],
            Some(&repo.root),
        );
        let worktrees = String::from_utf8(worktrees.stdout).unwrap();
        let branches = String::from_utf8(branches.stdout).unwrap();
        if !worktrees.contains("feat-e2e-a")
            && !worktrees.contains("feat-e2e-b")
            && !branches.contains("feat/e2e-a")
            && !branches.contains("feat/e2e-b")
        {
            return;
        }
        sleep(Duration::from_millis(50));
    }

    panic!(
        "selected worktrees or branches remained:\n{}\n{}",
        String::from_utf8_lossy(
            &run_command(
                "git",
                &["worktree", "list", "--porcelain"],
                Some(&repo.root)
            )
            .stdout
        ),
        String::from_utf8_lossy(
            &run_command(
                "git",
                &["branch", "--format=%(refname:short)"],
                Some(&repo.root),
            )
            .stdout
        )
    );
}
