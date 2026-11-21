use sigmaos_replaces::Component;

#[test]
fn test_component_new_aaa() {
    // Arrange + Act
    let c = Component::new();
    // Assert
    assert!(!c.is_ready());
}

#[test]
fn test_component_init_aaa() {
    // Arrange
    let mut c = Component::new();
    // Act
    let res = c.init();
    // Assert
    assert!(res.is_ok());
}

#[test]
fn test_component_run_aaa() {
    // Arrange
    let mut c = Component::new();
    c.init().unwrap();
    // Act
    let res = c.run();
    // Assert
    assert!(res.is_ok());
}
