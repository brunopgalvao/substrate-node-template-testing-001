use crate::{mock::*, Error};
use frame_support::{assert_ok, assert_err};

#[test]
fn test_add_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::add_value(Origin::signed(1), 10));
		assert_eq!(TemplateModule::total(), Some(10));
    })
}

#[test]
fn test_multiple_add_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::add_value(Origin::signed(1), 10));
		assert_ok!(TemplateModule::add_value(Origin::signed(1), 20));
		assert_eq!(TemplateModule::total(), Some(30));
    })
}

#[test]
fn test_max_value() {
    new_test_ext().execute_with(|| {
        assert_err!(
            TemplateModule::add_value(Origin::signed(1), 51), 
			Error::<Test>::ValueBiggerThanMax
        );
    })
}