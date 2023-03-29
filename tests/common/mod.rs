use dotenvy::dotenv;

// Using this as an example of how testing could be done
// This module provides an example of a setup that could be called and run from other tests

pub fn setup() {
    // This function can do setup for any functions that call it
    dotenv().unwrap();
}
