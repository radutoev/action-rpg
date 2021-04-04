use gdnative::api::*;
use gdnative::prelude::*;


#[test]
fn normalize_vector() {
    let normalized = Vector2::normalize(Vector2::new(10.0, 6.0));
    assert!(normalized.x <= 1.0);
    assert!(normalized.y <= 1.0); 
}