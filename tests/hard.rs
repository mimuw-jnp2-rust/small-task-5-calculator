use crate::common::{multi_thread_test, single_thread_test};
use common::TestElement;
use std::time::Duration;

mod common;

#[test]
fn history_single_thread() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Add": 5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#"{"Add": -5}"#),
            TestElement::expect_response(r#""Ok""#),
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
            TestElement::send_request(r#""History""#),
            TestElement::expect_response(
                r#"{"History": [[0, {"Add": 5}], [0, {"Add": -5}], [0, {"Add": 10}], [0, {"Multiply": 2}],  [0, {"Divide": 2}]]}"#,
            ),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn history_multiple_threads() {
    multi_thread_test(
        vec![
            vec![
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::sleep(Duration::from_millis(100)),
                TestElement::send_request(r#"{"Add": -5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::sleep(Duration::from_millis(100)),
                TestElement::send_request(r#""History""#),
                TestElement::expect_response(
                    r#"{"History": [[0, {"Add": 5}], [1, {"Add": 5}], [0, {"Add": -5}]]}"#,
                ),
            ],
            vec![
                TestElement::sleep(Duration::from_millis(50)),
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::send_request(r#""History""#),
                TestElement::expect_response(r#"{"History": [[0, {"Add": 5}], [1, {"Add": 5}]]}"#),
            ],
        ],
        Duration::from_secs(1),
    )
}

#[test]
fn disconnect_single_thread() {
    single_thread_test(
        vec![
            TestElement::send_request(r#"{"Add": 5}"#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::send_request(r#""Exit""#),
            TestElement::expect_response(r#""Ok""#),
            TestElement::expect_disconnect(),
        ],
        Duration::from_secs(1),
    );
}

#[test]
fn disconnect_multiple_threads() {
    multi_thread_test(
        vec![
            vec![
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::sleep(Duration::from_millis(100)),
                TestElement::send_request(r#""Exit""#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::expect_disconnect(),
            ],
            vec![
                TestElement::sleep(Duration::from_millis(50)),
                TestElement::send_request(r#"{"Add": 5}"#),
                TestElement::expect_response(r#""Ok""#),
                TestElement::sleep(Duration::from_millis(100)),
                TestElement::expect_disconnect(),
            ],
        ],
        Duration::from_secs(1),
    )
}
