/* #region Imports */
// Standard
use std::{error::Error, sync::Arc};

// 3rd Party
use tokio::task::{self, JoinHandle};
use serde_json::Value;

// Project
use crate::{json_generator, search_tree::{breadth_first_search, depth_first_search}};
use super::{reporter::Report, measurement_types::MeasurementType};
/* #endregion */

static CHARACTER_POLL: &str = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!@#$%&";

pub struct RunTestLoop {
    test_count: u8,
    value_to_search: Arc<Value>,
}

impl RunTestLoop {

    pub fn new(test_count: u8, value_to_search: Value) -> RunTestLoop {
        RunTestLoop {
            test_count,
            value_to_search: Arc::new(value_to_search),
        }
    }

    pub async fn run_test(&self, json_name: Arc<String>, number_of_letters: u8, depth: u8, number_of_children: u8, raw_json: Arc<String>)
    -> Result<(), Box<dyn Error + Send + Sync>> {
        for counter in 1..=self.test_count {
            let test_name = format!("Test {}", counter);
            self.run_single_test(
                test_name,
                Arc::clone(&json_name),
                number_of_letters,
                depth,
                number_of_children,
                Arc::clone(&raw_json)
            ).await?;
        }

        Ok(())
    }

    async fn run_single_test(&self, test_count: String, json_name: Arc<String>, number_of_letters: u8, depth: u8, number_of_children: u8, raw_json: Arc<String>)
    -> Result<(), Box<dyn Error + Send + Sync>> {
        Report::async_measure(
            test_count.clone(),
            Arc::clone(&json_name),
            MeasurementType::TotalIncludeContextSwitch,
            self.run_single_test_without_total_measure(
                test_count, json_name, number_of_letters, depth, number_of_children, raw_json
            )
        ).await
    }

    async fn run_single_test_without_total_measure(&self, test_count: String, json_name: Arc<String>, number_of_letters: u8, depth: u8, number_of_children: u8, raw_json: Arc<String>)
    -> Result<(), Box<dyn Error + Send + Sync>> {
        RunTestLoop::test_generate_json(test_count.clone(), Arc::clone(&json_name), number_of_letters, depth, number_of_children).await??;
        let json = RunTestLoop::test_deserialize_json(test_count.clone(), Arc::clone(&json_name), raw_json).await?;
        let json = Arc::new(json);
        self.test_iterate_iteratively(test_count.clone(), Arc::clone(&json_name), Arc::clone(&json)).await?;
        self.test_iterate_recursively(test_count.clone(), Arc::clone(&json_name), Arc::clone(&json)).await?;
        RunTestLoop::test_serialize_json(test_count, json_name, json).await??;
        Ok(())
    }

    fn test_generate_json(test_count: String, json_name: Arc<String>, number_of_letters: u8, depth: u8, number_of_children: u8)
    -> JoinHandle<Result<Value, Box<dyn Error + Send + Sync>>> {
        task::spawn_blocking(move || {
            Report::measure(test_count, json_name, MeasurementType::GenerateJson, move ||
                json_generator::Generator::generate_json(CHARACTER_POLL, number_of_letters, depth, number_of_children)
            )
        })
    }

    fn test_deserialize_json(test_count: String, json_name: Arc<String>, raw_json: Arc<String>) -> JoinHandle<Value> {
        task::spawn_blocking(move || {
            Report::measure(test_count, json_name, MeasurementType::DeserializeJson, move ||
                serde_json::from_str::<Value>(&raw_json).expect("Couldn't parse the input JSON")
            )
        })
    }

    fn test_iterate_iteratively(&self, test_count: String, json_name: Arc<String>, json: Arc<Value>) -> JoinHandle<()> {
        let value_to_search = Arc::clone(&self.value_to_search);
        task::spawn_blocking(move || {
            Report::measure(test_count, json_name, MeasurementType::IterateIteratively, move || {
                assert!(!breadth_first_search::run(&json, &value_to_search), "BFS the tree found value that shouldn't be in it: {}", value_to_search)
            })
        })
    }

    fn test_iterate_recursively(&self, test_count: String, json_name: Arc<String>, json: Arc<Value>) -> JoinHandle<()> {
        let value_to_search = Arc::clone(&self.value_to_search);
        task::spawn_blocking(move || {
            Report::measure(test_count, json_name, MeasurementType::IterateRecursively, move ||
                assert!(!depth_first_search::run(&json, &value_to_search), "DFS the tree found value that shouldn't be in it: {}", value_to_search)
            )
        })
    }

    fn test_serialize_json(test_count: String, json_name: Arc<String>, json: Arc<Value>) -> JoinHandle<Result<String, serde_json::Error>> {
        task::spawn_blocking(move || {
            Report::measure(test_count, json_name, MeasurementType::SerializeJson, move ||
                serde_json::to_string::<Value>(&json)
            )
        })
    }
}
