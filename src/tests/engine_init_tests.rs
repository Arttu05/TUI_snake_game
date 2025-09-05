use crate::utils::engine_inits::get_engine_from_file;


#[test]
fn read_ok_settings(){

    let result = get_engine_from_file("./src/tests/ok_settings.json");

    assert_eq!(result.is_ok(), true, "{:?}", result.err());
}

#[test]
fn validate_ok_settings(){

    let result = get_engine_from_file("./src/tests/ok_settings.json");

    let mut engine = result.ok().unwrap();

    let valid_settings = engine.validate_settings();

    assert_eq!(valid_settings.is_ok(), 
        true, 
        "validate_settings() returned following error: {:?}",
        valid_settings.err()
    )
}
