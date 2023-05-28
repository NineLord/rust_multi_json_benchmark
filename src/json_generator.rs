/* #region Imports */
// Standard
use std::error::Error;

// 3rd Party
use serde_json::{ Value, Map };

// Project
use crate::utils::{ json_type, randomizer };
/* #endregion */

#[derive(Debug)]
pub struct Generator {
    charachters_poll: Vec<char>,
    number_of_letters: usize,
    depth: u8,
    number_of_children: u8,
}

impl Generator {
    fn new(charachters_poll: &str, number_of_letters: u8, depth: u8, number_of_children: u8) -> Generator {
        Generator {
            charachters_poll: charachters_poll.chars().collect(),
            number_of_letters: number_of_letters.into(),
            depth,
            number_of_children
        }
    }

    pub fn generate_json(charachters_poll: &str, number_of_letters: u8, depth: u8, number_of_children: u8) -> Result<Value, Box<dyn Error>> {
        let generator = Generator::new(charachters_poll, number_of_letters, depth, number_of_children);

        generator.generate_full_tree()
    }

    fn generate_full_tree(&self) -> Result<Value, Box<dyn Error>> {
        let mut root = Map::new();
        
        /* #region Edge Cases */
        if self.depth == 0 {
            return Ok(Value::Object(root));
        } else if self.depth == 1 {
            self.add_leaf_children_to_map(&mut root);
            return Ok(Value::Object(root));
        }
        /* #endregion */

        /* #region First Level */
        self.add_none_leaf_children_to_map(&mut root);
        let mut result =  Value::Object(root);
        /* #endregion */

        /* #region Middle Levels */
        let mut current_nodes: Vec<&mut Value> = vec![&mut result];
        let mut next_level_nodes: Vec<&mut Value> = vec!();
        let last_level = self.depth - 1;

        for _level in 1..last_level {
            while let Some(current_node) = current_nodes.pop() {
                match current_node {
                    Value::Array(current_node_array) =>
                        self.add_none_leaf_children(current_node_array.iter_mut(), &mut next_level_nodes)?,
                    Value::Object(current_node_object) =>
                        self.add_none_leaf_children(current_node_object.values_mut(), &mut next_level_nodes)?,
                    _ => return Err(Box::from(format!("Invalid current node type: {}", current_node)))
                }
            }

            current_nodes = next_level_nodes;
            next_level_nodes = vec!();
        }
        /* #endregion */

        /* #region Last Level */
        while let Some(current_node) = current_nodes.pop() {
            match current_node {
                Value::Array(current_node_array) =>
                    self.add_leaf_children(current_node_array.iter_mut())?,
                Value::Object(current_node_object) =>
                    self.add_leaf_children(current_node_object.values_mut())?,
                _ => return Err(Box::from(format!("Invalid current node type: {}", current_node)))
            }
        }
        /* #endregion */

        Ok(result)
    }

    /* #region Helper methods */
    fn get_random_node_character(&self) -> &char {
        randomizer::get_random_value_from_array(&self.charachters_poll)
    }

    fn get_random_node_name(&self) -> String {
        let mut string_builder = String::with_capacity(self.number_of_letters);
        for _count in 0..self.number_of_letters {
            string_builder.push(*self.get_random_node_character());
        }
        string_builder
    }

    fn add_none_leaf_children_to_array(&self, array: &mut Vec<Value>) {
        for _node_count in 0..self.number_of_children {
            let child_node = json_type::get_random_none_leaf_json();
            array.push(child_node);
        }
    }
    
    fn add_none_leaf_children_to_map(&self, object: &mut Map<String, Value>) {
        for _node_count in 0..self.number_of_children {
            let child_node_name = self.get_random_node_name();
            let child_node = json_type::get_random_none_leaf_json();
            object.insert(child_node_name, child_node);
        }
    }

    fn add_none_leaf_children<'a, 'b, I>(&self, iterator: I, next_level_nodes: &'b mut Vec<&'a mut Value>) -> Result<(), Box<dyn Error>>
    where I: Iterator<Item = &'a mut Value> {
        for next_level_node in iterator {
            match next_level_node {
                Value::Array(array) => self.add_none_leaf_children_to_array(array),
                Value::Object(map) => self.add_none_leaf_children_to_map(map),
                _ => return Err(Box::from(format!("Invalid child node type: {}", next_level_node)))
            }
            next_level_nodes.push(next_level_node);
        }

        Ok(())
    }

    fn add_leaf_children_to_array(&self, array: &mut Vec<Value>) {
        for _node_count in 0..self.number_of_children {
            let child_node = json_type::get_random_leaf_json();
            array.push(child_node);
        }
    }

    fn add_leaf_children_to_map(&self, object: &mut Map<String, Value>) {
        for _node_count in 0..self.number_of_children {
            let child_node_name = self.get_random_node_name();
            let child_node = json_type::get_random_leaf_json();
            object.insert(child_node_name, child_node);
        }
    }

    fn add_leaf_children<'a, I>(&self, iterator: I) -> Result<(), Box<dyn Error>>
    where I: Iterator<Item = &'a mut Value> {
        for next_level_node in iterator {
            match next_level_node {
                Value::Array(array) => self.add_leaf_children_to_array(array),
                Value::Object(map) => self.add_leaf_children_to_map(map),
                _ => return Err(Box::from(format!("Invalid child node type: {}", next_level_node)))
            }
        }

        Ok(())
    }
    /* #endregion */
}