use crate::repl::{CommandResult, ReplState};

#[tokio::test]
async fn test_clear_command() {
    let mut repl = ReplState::new();
    let result = repl.handle_repl_command(".clear").unwrap();
    assert_eq!(result, CommandResult::ClearedScreen);
}
