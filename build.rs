const COMMANDS: &[&str] = &[
    "start_recording",
    "stop_recording",
    "pause_recording",
    "resume_recording",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
