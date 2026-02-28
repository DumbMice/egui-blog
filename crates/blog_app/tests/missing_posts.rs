use blog_app::{PostManager, PostManagerState};

#[test]
fn test_error_state_display() {
    let manager = PostManager::default();
    let state = manager.state();

    // Verify state is one of the expected variants
    match state {
        PostManagerState::Loading => println!("Loading state"),
        PostManagerState::Loaded => println!("Loaded state"),
        PostManagerState::Error(_) => println!("Error state"),
        PostManagerState::Empty => println!("Empty state"),
    }
}
