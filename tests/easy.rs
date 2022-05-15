use crate::common::{multi_thread_test, single_thread_test};
use common::TestElement;
use std::time::Duration;

mod common;

#[test]
fn add_and_get() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Add": 5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""Result""#),
            TestElement::expect_response(r#"{"Result": 5}"#),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn calculator() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Add": 5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#"{"Add": -5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""Result""#),
            TestElement::expect_response(r#"{"Result": 0}"#),
            TestElement::send_request(r#""Result""#),
            TestElement::expect_response(r#"{"Result": 0}"#),
            TestElement::send_request(r#"{"Add": 10}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#"{"Multiply": 2}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""Result""#),
            TestElement::expect_response(r#"{"Result": 20}"#),
            TestElement::send_request(r#"{"Divide": 2}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""Result""#),
            TestElement::expect_response(r#"{"Result": 10}"#),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn division_by_zero() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Divide": 0}"#),
            TestElement::expect_response(r#"{"Error": "Division by zero"}"#),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn two_threads() {
    multi_thread_test(
        vec![
            vec![
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""Result""#),
                TestElement::expect_response(r#"{"Result": 5}"#),
                TestElement::sleep(Duration::from_millis(100)),
            ],
            vec![
                TestElement::sleep(Duration::from_millis(50)),
                TestElement::send_request(r#"{"Add": 10}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""Result""#),
                TestElement::expect_response(r#"{"Result": 15}"#),
            ],
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn my_history_single_thread() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Add": 5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#"{"Add": 10}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""MyHistory""#),
            TestElement::expect_response(r#"{"MyHistory": [{"Add": 5}, {"Add": 10}]}"#),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn my_history_multiple_threads() {
    multi_thread_test(
        vec![
            vec![
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#"{"Add": 10}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""MyHistory""#),
                TestElement::expect_response(r#"{"MyHistory": [{"Add": 5}, {"Add": 10}]}"#),
            ],
            vec![
                TestElement::send_request(r#"{"Add": 15}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""MyHistory""#),
                TestElement::expect_response(r#"{"MyHistory": [{"Add": 15}]}"#),
            ],
            vec![
                TestElement::send_request(r#"{"Add": 20}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""MyHistory""#),
                TestElement::expect_response(r#"{"MyHistory": [{"Add": 20}]}"#),
            ],
            vec![
                TestElement::send_request(r#"{"Add": 25}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""MyHistory""#),
                TestElement::expect_response(r#"{"MyHistory": [{"Add": 25}]}"#),
            ],
        ],
        Duration::from_secs(1),
    );
}
