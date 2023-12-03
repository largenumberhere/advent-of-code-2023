use std::fmt::{Debug, Formatter};

type ContextualNodes = Vec<Vec<ContextualNode>>;

#[derive(Clone)]
pub enum  BasicNode {
    Digit (u32),
    Empty,
    SpecialSymbol,
}

// When using `println!("{:?}", node)`, , use a minimum padding to make it look it printable as a primitive table
impl Debug for BasicNode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicNode::Digit(d) => {
                f.pad(
                    &format_width(
                        format_args!("Digit({})", d)
                    )
                )?;
            }
            BasicNode::Empty => {
                f.pad(
                    &format_width(format_args!("{}", "Empty"))
                )?;
            }
            BasicNode::SpecialSymbol => {
                f.pad(
                    &format_width(format_args!("{}", "Symbol"))
                )?;
            }

        }

        Ok(())
    }
}

#[derive(Clone)]
pub enum ContextualNode {
    Digit{
        number: u32,
        left_digit: bool,
        digit_right: bool
    },
    SpecialSymbol,
    Empty,
    Unspecified
}


// When using `println!("{:?}", contextual_node)`, use a minimum padding to make it look it printable as a primitive table
impl Debug for ContextualNode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualNode::Digit { left_digit, digit_right, number} => {
                let left_msg = match left_digit {
                    true => {"L"}
                    _=> {""}
                };

                let right_msg = match digit_right {
                    true => {"R"},
                    _=> {""}
                };


                f.write_str( & format_width(format_args!("Digit[{}, {}, {}]", number, left_msg, right_msg)))?
            }
            ContextualNode::Empty => {
                f.write_str(
                    &format_width(format_args!("{}","Empty"))
                )?;
            }
            ContextualNode::SpecialSymbol => {
                f.write_str(
                    &format_width(format_args!("{}", "Symbol"))
                )?;
            }
            ContextualNode::Unspecified => {
                f.write_str(
                    &format_width(format_args!("{}", "UNSPECIFIED"))
                )?;
            }
        }

        Ok(())
    }
}

pub fn cont_node_from_coords(line_and_row: (usize, usize), nodes: &ContextualNodes) -> Option<&ContextualNode> {
    let line = nodes.get(line_and_row.0)?;
    let node = line.get(line_and_row.1)?;
    Some(node)
}

pub fn add_if_valid_contextual_node<'a>(line_and_row_opt: Option<(usize, usize)>, nodes: &'a ContextualNodes, output: &mut Vec<(&'a ContextualNode, (usize, usize))>) {
    //println!("line_and_row_opt: {:?}, nodes: {:?}, output: {:?}", line_and_row_opt, nodes, output);

    if let Some(line_and_row) = line_and_row_opt {
        let node_opt = cont_node_from_coords(line_and_row, nodes);
        match node_opt {
            None => {
                return;
            }
            Some(node) => {
                output.push((node, line_and_row))
            }
        }
    }

}


/// Add a standard formatting to each string.
///  Allows primitive tables to be printed with the columns lined up
pub fn format_width(args: std::fmt::Arguments) -> String {
    // create a string
    let mut result = format!("{}", &args);

    // The fill/alignment format specifier (`{:a>b}`, where a and b are numbers),
    //  seems to be only limited to small numbers.
    //  Perhaps I'm using it wrong or it's a compiler error?
    //  I could not find examples including more than single digit numbers.
    //  It also seems to not support constants which is a bummer.
    //  The following is good enough I suppose...
    while result.len() <= 15 {
        result.push(' ');
    }
    result.truncate(15);
    result
}
