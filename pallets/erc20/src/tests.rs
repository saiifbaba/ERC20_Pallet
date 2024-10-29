#![cfg(test)]

use crate::{Error, mock::*, Event};
use frame_support::{assert_ok, assert_err};

#[test]
fn test_do_something() {
    new_test_ext().execute_with(|| {
        // Arrange: Set an initial value.
        let initial_value = 42;
        assert_ok!(TemplatePallet::do_something(Origin::signed(1), initial_value));

        // Act: Retrieve the value from storage.
        let stored_value = TemplatePallet::something();

        // Assert: Check that the value is as expected.
        assert_eq!(stored_value, Some(initial_value));

        // Assert: Check that the event was emitted correctly.
        System::assert_last_event(Event::SomethingStored { something: initial_value, who: 1 }.into());
    });
}

#[test]
fn test_cause_error() {
    new_test_ext().execute_with(|| {
        // Arrange: No value is set initially.
        
        // Act & Assert: Call cause_error and expect an error.
        assert_err!(TemplatePallet::cause_error(Origin::signed(1)), Error::<TestRuntime>::NoneValue);
    });
}

#[test]
fn test_increment_value() {
    new_test_ext().execute_with(|| {
        // Arrange: Set an initial value.
        let initial_value = 42;
        assert_ok!(TemplatePallet::do_something(Origin::signed(1), initial_value));

        // Act: Increment the value.
        assert_ok!(TemplatePallet::cause_error(Origin::signed(1)));

        // Assert: Check that the value has been incremented.
        let updated_value = TemplatePallet::something().unwrap();
        assert_eq!(updated_value, initial_value + 1);
    });
}
