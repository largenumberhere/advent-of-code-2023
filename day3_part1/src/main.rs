use std::fmt::{Debug, Formatter};
use std::fs;

const special_characters: [char; 11] = ['*', '$', '+', '#', '@', '&', '%', '+', '/', '=', '-'];

mod node_models;
use node_models::BasicNode;
use node_models::ContextualNode;
use node_models::cont_node_from_coords;
use crate::node_models::add_if_valid_contextual_node;

fn main() {
    let input_text = fs::read_to_string(
        std::env::args()
            .skip(1)
            .next()
            .expect("2nd argument must be file path")
    )
        .expect("Failed to read file");

    let input_lines = input_text.lines()
        .enumerate();

    let mut nodes: Vec<Vec<BasicNode>> = Vec::new();
    'outer: for (zero_indexed_line_number, line_text) in input_lines {
        let mut line = Vec::new();
        'inner: for c in line_text.chars() {
            if c.is_digit(10) {
                line.push(
                    BasicNode::Digit(c.to_digit(10).unwrap())
                );
                continue;
            }

            if c == '.' {
                line.push(
                    BasicNode::Empty
                );
                continue;
            }

            for special in special_characters {
                //println!("'{}' '{}'", c, special);
                if c.to_ascii_lowercase() == special.to_ascii_lowercase() {
                    line.push(
                        BasicNode::SpecialSymbol
                    );
                    //println!("special found {}=={}", c, special);

                    continue 'inner;
                }
            }

            panic!("Unexpected character found '{}' on line {}", c, zero_indexed_line_number + 1);
        }

        nodes.push(line);
    }


    // debug the nodes
    // for line in nodes.iter() {
    //     println!("{:?}", line);
    // }

    // Create and Write to contextual nodes.
    //  Tags each digit with weather it has another one to its left or right.
    let contextual_nodes = {
        let mut contextual_nodes: Vec<Vec<ContextualNode>> = Vec::new();

        // allocate defaults. todo: make this less bone-headed
        for line in nodes.iter() {
            let mut out_nodes = Vec::new();
            for node in line {
                out_nodes.push(ContextualNode::Unspecified);
            }
            contextual_nodes.push(out_nodes);
        }

        // check it is the correct size.
        assert_eq!(nodes[0].len(), contextual_nodes[0].len());
        assert_eq!(contextual_nodes.len(), contextual_nodes.len());

        // update all the nodes information about neighbour numbers
        let mut index_in_lines = 0;
        let mut index_in_line = 0;
        loop {
            let mut current_has_left = false;
            let mut current_has_right = false;
            match nodes[index_in_lines][index_in_line] {
                // current node to copy from is useful
                BasicNode::Digit(d) => {
                    // check for digit left
                    let prev_node_opt = {
                        let line_index_opt = index_in_line.checked_sub(1);
                        match line_index_opt {
                            None => None,
                            Some(line_index) => {
                                contextual_nodes[index_in_lines].get_mut(line_index)
                            }
                        }
                    };

                    // do stuff if there's a left digit
                    match prev_node_opt {
                        Some(pre_node) => {
                            match pre_node
                            {
                                // there is a digit left, let's record that these 2 have neighbours
                                ContextualNode::Digit { number, left_digit, ref mut digit_right } => {
                                    *digit_right = true;
                                    current_has_left = true;
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }

                    // check digit on the right
                    let next_node_opt = {
                        let line_index_opt = index_in_line.checked_add(1);
                        match line_index_opt {
                            None => None,
                            Some(line_index) => {
                                contextual_nodes[index_in_lines].get_mut(line_index)
                            }
                        }
                    };

                    // do stuff if there's a right digit
                    match next_node_opt {
                        Some(next_node) => {
                            // there is a digit on the right, let's record that these 2 have neighbours
                            match next_node {
                                ContextualNode::Digit { number, digit_right, left_digit } => {
                                    *left_digit = true;
                                    current_has_right = true;
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }

                    // write current digit
                    contextual_nodes[index_in_lines][index_in_line] = ContextualNode::Digit {
                        left_digit: current_has_left,
                        digit_right: current_has_right,
                        number: d
                    }
                }

                // current node is boring
                BasicNode::Empty => {
                    contextual_nodes[index_in_lines][index_in_line] = ContextualNode::Empty
                }
                BasicNode::SpecialSymbol => {
                    contextual_nodes[index_in_lines][index_in_line] = ContextualNode::SpecialSymbol
                }
            }

            // update loop counter
            index_in_line += 1;

            // check we have not gone over the max line width
            match nodes[0].get(index_in_line) {
                Some(_) => {}
                None => {
                    index_in_line = 0;
                    index_in_lines += 1;
                }
            }

            // check we have not gone over max line
            match nodes.get(index_in_lines) {
                Some(_) => {}
                None => {
                    break;
                }
            }
        };

        contextual_nodes
    };

    // println!("contextual nodes:");
    // for line in &contextual_nodes {
    //     println!("{:?}", line);
    // }
    // println!("end contextual node\n\n");

    // note positions of all the special characters
    let special_character_positions = {
        let mut specials = Vec::new();
        for (line_number, line) in contextual_nodes.iter().enumerate() {
            for (node_number, node) in line.iter().enumerate() {
                match node {
                    ContextualNode::SpecialSymbol => {
                        specials.push((line_number, node_number));
                    }
                    _ => {}
                }
            }
        }

        specials
    };

    //println!("special_character position {:?}", special_character_positions);


    // Find and note all of the numbers touching the special characters
    let mut touched = Vec::new();
    for (line_number, node_number) in special_character_positions {
        // check top
        {
            // calculate top index
            let top_index_opt = {
                let a = line_number.checked_sub(1);
                let b = node_number;
                match a {
                    None => None,
                    Some(a) => {
                        Some((a, b))
                    }
                }
            };

            add_if_valid_contextual_node(top_index_opt, &contextual_nodes, &mut touched);
        };

        // check bellow
        {
            // Calculate bottom index
            let bottom_index_opt = {
                let a = line_number.checked_add(1);
                let b = node_number;
                match a {
                    None => None,
                    Some(a) => {
                        Some((a, b))
                    }
                }
            };

            add_if_valid_contextual_node(bottom_index_opt, &contextual_nodes, &mut touched);
        }

        // check left
        {
            let left_index_opt = {
                let a = line_number;
                let b = node_number.checked_sub(1);
                match b {
                    None => None,
                    Some(b) => {
                        Some((a, b))
                    }
                }
            };

            add_if_valid_contextual_node(left_index_opt, &contextual_nodes, &mut touched);
        }

        // check right
        {
            // Get index
            let right_index_opt = {
                let a = line_number;
                let b = node_number.checked_add(1);
                match b {
                    None => None,
                    Some(b) => {
                        Some((a, b))
                    }
                }
            };

            add_if_valid_contextual_node(right_index_opt, &contextual_nodes, &mut touched);
        }

        // check top left
        {
            let top_left_index_opt = {
                let a = line_number.checked_sub(1);
                let b = node_number.checked_sub(1);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        Some((a, b))
                    }
                    _ => {
                        None
                    }
                }
            };

            add_if_valid_contextual_node(top_left_index_opt, &contextual_nodes, &mut touched);
        }

        // check top right
        {
            let top_right_index_opt = {
                let a = line_number.checked_sub(1);
                let b = node_number.checked_add(1);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        Some((a, b))
                    }

                    _ => {
                        None
                    }
                }
            };

            add_if_valid_contextual_node(top_right_index_opt, &contextual_nodes, &mut touched)
        }

        // check bottom left
        {
            let bottom_left_index_opt = {
                let a = line_number.checked_add(1);
                let b = node_number.checked_sub(1);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        Some((a, b))
                    }

                    _ => {
                        None
                    }
                }
            };


            add_if_valid_contextual_node(bottom_left_index_opt, &contextual_nodes, &mut touched)
        }

        // check bottom right
        {
            let bottom_right_index_opt = {
                let a = line_number.checked_add(1);
                let b = node_number.checked_add(1);

                match (a, b) {
                    (Some(a), Some(b)) => {
                        Some((a, b))
                    }
                    _ => {
                        None
                    }
                }
            };

            add_if_valid_contextual_node(bottom_right_index_opt, &contextual_nodes, &mut touched)
        }
    }

    // Remove empty entries
    {
        let mut empties = Vec::new();
        for (index, (node, coords)) in touched.iter().enumerate() {
            match node {
                ContextualNode::Empty => {
                    empties.push(index);
                }
                _ => {}
            }
        }
        for i in empties.into_iter().rev() {
            touched.remove(i);
        }
    }

    // debug
    // println!("Numbers that touch nodes:");
    // for pair in &touched {
    //     println!("{:?}", pair.0)
    // }

    let mut lefts = Vec::new();

    // Find the left-most digit that this number links to
    for (digit_node, digit_coords) in touched.into_iter() {
        let digit = match digit_node {
            ContextualNode::Digit {left_digit, digit_right, number } => {
                (left_digit, digit_right, number)
            }
            _ => {
                panic!("Unexpected non-digit found in touched");
            }
        };

        let mut selected_node = digit;
        let mut selected_coords = digit_coords;

        'nodes: loop {
            // has left node
            if *selected_node.0 == true {
                // move to left node
                selected_coords.1 -=1;
                let left = cont_node_from_coords((selected_coords), &contextual_nodes);
                let left = left.expect("The node lied!! It has no left");

                selected_node = match left {
                    ContextualNode::Digit { left_digit,digit_right, number } => {
                        (left_digit, digit_right, number)
                    }
                    _ => {
                        panic!("The node lied!! Left is not a number '{:?}' at {:?}", left, selected_coords)
                    }
                };

            }
            // has no left node
            else {
                lefts.push((selected_node, selected_coords));
                break 'nodes;
            }
        }
    }


    // Remove duplicates
    lefts.sort();   // dedup requires a sorted vec
    lefts.dedup();

    // Debug
    // for left in &lefts {
    //     println!("{:?}", left)
    // }

    // Parse the numbers from the nodes
    let mut all_numbers = {
        let mut all_numbers = Vec::new();
        let mut pending_number: Vec<u32> = Vec::new();
        for (digit, coords) in lefts {
            pending_number.clear();

            let mut selected_digit = digit;
            let mut selected_coords = coords;
            loop {
                // save digit
                pending_number.push(*selected_digit.2);

                // check if there are more in this number
                if *selected_digit.1 == true {

                    // go to next digit if there is one
                    selected_coords.1 += 1;

                    let next = cont_node_from_coords(selected_coords, &contextual_nodes);
                    let next = next.expect("The node lied!! There is nothing to the right");
                    let next = match next {
                        ContextualNode::Digit { left_digit, digit_right, number } => {
                            (left_digit, digit_right, number)
                        }
                        _ => {
                            panic!("The node lied!! the node to the right is not a digit!")
                        }
                    };

                    selected_digit = next;
                }
                // This was the last digit in a row
                else {
                    all_numbers.push(pending_number.clone());
                    break;
                }
            }
        }
        all_numbers
    };

    // // debug the numbers
    // for number in &all_numbers {
    //     println!("number {:?}", number);
    // }

    // convert digits arraylists to normal numbers
    let mut normal_numbers = Vec::new();
    for digits in all_numbers {
        let mut tally: u32 = 0;
        let length = digits.len();

        //println!("digit ");
        for (position, digit) in digits.into_iter().enumerate() {
            let position_power = (length -1 - position);
            let position_multiple:u32 = 10;
            let position_multiplier = position_multiple.pow(position_power as u32);

            //println!("position multiplier {}", position_multiplier);

            let position_value: u32 = (digit as u32) * position_multiplier;

            tally += position_value;
        }

        normal_numbers.push(tally);
    }

    // Print the list
    //println!("{:?}", normal_numbers);

    // Sum the list
    let sum: u32 = normal_numbers.iter().sum();

    // Print the reuslt
    println!("Sum :{}", sum);
}


