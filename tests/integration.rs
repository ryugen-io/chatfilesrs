use std::fs;

use chatfiles::core::{Chatfile, names};
use chatfiles::log;

#[test]
fn test_chatfile_create_and_read() {
    let _ = fs::remove_file("test_create.Chatfile");

    let cf = Chatfile::create(Some("test_create")).unwrap();
    assert!(cf.path.exists());

    cf.send("test-agent", "Hello world").unwrap();

    let lines = cf.read_last(10).unwrap();
    assert!(lines.len() >= 2);
    assert!(lines.last().unwrap().contains("Hello world"));

    fs::remove_file("test_create.Chatfile").unwrap();
}

#[test]
fn test_name_generation() {
    let _ = fs::remove_file("test_namegen.Chatfile");

    let cf = Chatfile::create(Some("test_namegen")).unwrap();
    let name = names::generate(&cf).unwrap();

    assert!(name.contains('-'));
    let parts: Vec<&str> = name.split('-').collect();
    assert_eq!(parts.len(), 3);

    fs::remove_file("test_namegen.Chatfile").unwrap();
}

#[test]
fn test_list_rooms() {
    let _ = fs::remove_file("test_list.Chatfile");

    let _ = Chatfile::create(Some("test_list")).unwrap();
    let rooms = Chatfile::list_rooms().unwrap();

    assert!(
        rooms
            .iter()
            .any(|p| p.to_string_lossy().contains("test_list"))
    );

    fs::remove_file("test_list.Chatfile").unwrap();
}

#[test]
fn test_message_format() {
    let _ = fs::remove_file("test_msg.Chatfile");

    let cf = Chatfile::create(Some("test_msg")).unwrap();
    cf.send("agent-1", "Test message").unwrap();
    cf.announce_join("agent-2").unwrap();

    let lines = cf.read_last(10).unwrap();

    assert!(lines.iter().any(|l| l == "agent-1: Test message"));
    assert!(lines.iter().any(|l| l == "[agent-2 joined]"));

    fs::remove_file("test_msg.Chatfile").unwrap();
}

#[test]
fn test_logging() {
    log::init();
    log::debug("TEST", "Debug message");
    log::info("TEST", "Info message");
    log::warn("TEST", "Warning message");
    log::error("TEST", "Error message");
}
