use std::ops::Add;

mod StringScanner;
fn main() {

    let mut parser = StringScanner::FileParser::new_from_path("my_input.txt")
        .unwrap();

    let mut line_number : u32 = 0;
    let directions = parser.read_line()
        .unwrap();
    line_number +=1;

    parser.discard_line()
        .unwrap();
    line_number+=1;

    let lines = get_node_lines(&mut parser, &mut line_number);
    let instructions = lines_to_nodes(lines.clone());
    let turn_directions = parse_turn_directions(directions);

    let mut position_info = DirectionsIterResult {
        current_pos: "AAA".to_string(),
        steps: 0
    };

    println!("lines.len() {}", lines.len());
    println!("instructions.len() {}",instructions.len());
    println!("turn_directions.len() {}", turn_directions.len());

    while (position_info.current_pos != "ZZZ") {
        follow_directions(&instructions, &turn_directions, &mut position_info);
    }

    println!("{:?}", position_info);

}

#[derive(Debug)]
struct DirectionsIterResult {
    current_pos: String,
    steps: u32
}

fn follow_directions(instructions: &Vec<NodeInstruction>, turn_directions: &Vec<(TurnDirection, CharacterNumber)>, current_info: &mut DirectionsIterResult) {
    // follow all instructions once
    //let mut i: u32 = 0;
    let mut i = current_info.steps;
    let mut turn_direction_number = 0;
    let mut next_turn: &(TurnDirection, CharacterNumber) = &turn_directions[turn_direction_number];
    //let mut current_pos: &String = &instructions[0].source;
    let mut current_pos = &current_info.current_pos;
    loop {

        // get current turn option
        let next_instruction = instructions.iter()
            .find(|instruction| &instruction.source == current_pos)
            .unwrap();

        // select new destination
        let dest = match next_turn.0 {
            TurnDirection::Left => {
                &next_instruction.left_turn_destination
            }
            TurnDirection::Right => {
                &next_instruction.right_turn_destination
            }
        };

        // update loop counter
        i +=1;

        // update position from turn
        current_pos = dest;

        // update next turn
        turn_direction_number +=1;

        next_turn = match turn_directions.get(turn_direction_number) {
            None => {
                break;
            }
            Some(v) => {v}
        };
    }

    current_info.current_pos = current_pos.to_owned();
    current_info.steps = i;
}

#[derive(Debug)]
struct NodeInstruction {
    source: String,
    left_turn_destination: String,
    right_turn_destination: String,
    line_number : u32,
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct CharacterNumber(usize);

fn parse_turn_directions(line1: String) -> Vec<(TurnDirection, CharacterNumber)> {
    let mut directions = Vec::new();
    for (i, c) in line1.chars().enumerate() {
        let turn_direction = match c {
            'L' =>  TurnDirection::Left,
            'R' => TurnDirection::Right,
            _ => panic!("Invalid character '{}' aka '{}' at position {} in string",c , c as i32 ,i)
        };

        directions.push((turn_direction, CharacterNumber(i)));
    }

    return directions;
}

fn get_node_lines(parser: &mut StringScanner::FileParser<impl std::io::Read>, line_number: &mut u32) -> Vec<(String, u32)> {
    let mut lines = Vec::new();
    loop {
        let line = parser.read_line();

        // break on end of file
        let line = match line {
            Err(StringScanner::ScannerError::EndOfFile) => break,
            Err(e) => panic!("{:?}", e),
            Ok(v) => v
        };

        // break on empty line
        if line.is_empty() {
            break;
        }

        // line was read successfully
        *line_number+=1;


        lines.push((line, *line_number));
    }


    return lines;
}

fn lines_to_nodes(lines: Vec<(String, u32)>) -> Vec<NodeInstruction> {
    let mut nodes = Vec::new();
    for (ln, line_number) in &lines {
        nodes.push(NodeInstruction{
            source: ln [0..3].to_owned(),
            left_turn_destination: ln [7..10].to_owned(),
            right_turn_destination: ln [12..15].to_owned(),
            line_number: *line_number
        });
    }

    return nodes;
}
