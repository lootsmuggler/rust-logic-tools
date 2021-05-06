///This file stores various structs for propositional logic.
///Author: Steven Fletcher
///Created: 2020
///Last Updated: 05/05/2021
///Please note that the booleans are numbered from 1 to n.  There is no 0 boolean.  This works fine.
use std::collections::HashMap;
//use std::fmt;

///A TruthValue of a LogicFormula might be unknown (Unrestricted), or it might be known that the value is either true
///(MustBeTrue), false (MustBeFalse), or both (Contradiction).
#[derive(Debug)]
pub enum TruthValue {
    Unrestricted,
    MustBeTrue,
    MustBeFalse//,
//    Contradiction
} //End enum TruthValue

//Constants for processing the literals.
///The highest order bit represents whether a literal is negated or not.
///u32 is used rather than i32 because bitwise operations should be faster than absolute value.
pub const NEGATIVITY_FLAG : u32     = 1 << 31;
pub const VARIABLE_INDEX_MASK : u32 = !NEGATIVITY_FLAG;

pub const CONJUNCTION_SYMBOL : &str = "&";
pub const DISJUNCTION_SYMBOL : &str = "|";
pub const NEGATION_SYMBOL : &str = "~";

pub const FALSE_TEXT : &str = "FALSE";
pub const TRUE_TEXT : &str  = "TRUE";

pub const NULL_TEXT : &str = "null";

pub fn get_variable_index(literal : u32) -> u32 {
    literal & VARIABLE_INDEX_MASK
} //End get_variable_index

pub fn is_positive_literal(literal : u32) -> bool {
    literal & NEGATIVITY_FLAG == 0
} //End is_positive_literal

///This trait is implemented by any boolean formula.  Any boolean formula should be able to be evaluated to produce a
///TruthValue for a given set of truth values.
pub trait LogicFormula {
    fn evaluate(&self, truth_values : &HashMap<u32,bool>) -> TruthValue;
} //End trait LogicFormula

///SimpleLogicNode can be used to entirely populate a multi-branching syntax tree.  It does not include secondary
///operators.
///Literal stores an integer representing the literal (sign and variable index)
///LiteralConjunction and LiteralDisjunction store a Vec containing multiple literals
///NodeConjunction and NodeDisjunction store a Vec containing multiple SimpleLogicNodes.
///The Literal value is used inside of NodeConjunctions/NodeDisjunctions that include both literals and SimpleLogicNodes
#[derive(Clone)]
pub enum SimpleLogicNode {
    False,
    True,
    Literal(u32),
    Conjunction(Vec<SimpleLogicNode>),
    Disjunction(Vec<SimpleLogicNode>)
} //End enum SimpleLogicNode

impl SimpleLogicNode {
    ///Counts the number of binary operators in this SimpleLogicNode and its descendants.
    ///Returns the number of binary operators in this SimpleLogicNode and its descendants.
    pub fn count_binary_operators(&self) -> u32 {
        match self {
            SimpleLogicNode::False => 0,
            SimpleLogicNode::True => 0,
            SimpleLogicNode::Literal(_) => 0,
            SimpleLogicNode::Conjunction(operands) => {
                let mut count : u32 = 1;
                for operand in operands {
                    count = count + operand.count_binary_operators();
                }
                count
            }, //End Conjunction
            SimpleLogicNode::Disjunction(operands) => {
                let mut count : u32 = 1;
                for operand in operands {
                    count = count + operand.count_binary_operators();
                }
                count
            } //End Disjunction
        }
    } //End count_binary_operators

    ///Gets a text representation of this SimpleLogicNode.
    ///boolean_name_list a list of the names of the booleans in this formula might have
    ///Return value: a text representation of this SimpleLogicNode
    pub fn get_as_text(&self, boolean_name_list : &Vec<String>) -> String {
        let mut text : String = "".to_owned();
        self.get_as_text_helper1(&mut text, false, boolean_name_list);
        return text
    } //End get_as_text

    //Gets a text representation of this SimpleLogicNode.
    //This function is mutually recursive with get_as_text_helper2.
    //boolean_name_list a list of the names of the booleans in this formula might have
    //should_parenthesize - whether the children of this node should be parenthesized
    fn get_as_text_helper1(&self, text : &mut String, should_parenthesize : bool, boolean_name_list : &Vec<String>) {
        match self {
            SimpleLogicNode::False => text.push_str(FALSE_TEXT),
            SimpleLogicNode::True => text.push_str(TRUE_TEXT),
            SimpleLogicNode::Literal(literal) => {
                //Check for negation
                if literal & NEGATIVITY_FLAG > 0 {text.push_str(NEGATION_SYMBOL);}

                //Append the name of the boolean.  (-1 because booleans range from 1 to n)
                text.push_str(&boolean_name_list[((literal & VARIABLE_INDEX_MASK) - 1) as usize]);

                //Exit the function.
                return;
            },
            SimpleLogicNode::Conjunction(node_vec) => {
                //If there's no nodes, output null
                if node_vec.len() == 0 {
                    text.push_str(NULL_TEXT);
                    return;
                } //End if there's no nodes

                if should_parenthesize {text.push('(');}
                self.get_as_text_helper2(text, node_vec, CONJUNCTION_SYMBOL, boolean_name_list);
                if should_parenthesize {text.push(')');}
            },
            SimpleLogicNode::Disjunction(node_vec) => {
                //If there's no nodes, output null
                if node_vec.len() == 0 {
                    text.push_str(NULL_TEXT);
                    return;
                } //End if there's no nodes

                if should_parenthesize {text.push('(');}
                self.get_as_text_helper2(text, node_vec, DISJUNCTION_SYMBOL, boolean_name_list);
                if should_parenthesize {text.push(')');}
            },
        }; //End match self
    } //End get_as_text_helper1

    //This function is mutually recursive with get_as_text_helper1.
    fn get_as_text_helper2(&self, text : &mut String, node_vec : &Vec<SimpleLogicNode>, symbol_text : &str,
                          boolean_name_list : &Vec<String>)
    {
        //For each clause in the node vector
        for clause in node_vec.iter() {
            clause.get_as_text_helper1(text, true, boolean_name_list);

            //Add the symbol
            text.push(' ');
            text.push_str(symbol_text);
            text.push(' ');
        } //End for each clause in the node Vector

        //Delete the last symbol at the end.
        let num_symbols_to_delete = 2 + symbol_text.len();
        text.truncate(text.len() - num_symbols_to_delete);
    } //End get_as_text_helper
} //End impl SimpleLogicNode

impl LogicFormula for SimpleLogicNode {
    fn evaluate(&self, truth_values : &HashMap<u32,bool>) -> TruthValue {
        //If one or more of the child nodes is unknown, it may be impossible to get the value of this node.
        let mut contains_unknown_children : bool = false;

        return match self {
            SimpleLogicNode::False => TruthValue::MustBeFalse,
            SimpleLogicNode::True => TruthValue::MustBeTrue,
            SimpleLogicNode::Literal(literal) => evaluate_single_literal(*literal, truth_values),
            SimpleLogicNode::Conjunction(conjunct_vec) => {
                //Check each conjunct
                for conjunct in conjunct_vec.iter() {
                    match conjunct.evaluate(truth_values) {
                        TruthValue::MustBeTrue      => (),
                        TruthValue::MustBeFalse     => (return TruthValue::MustBeFalse),
                        TruthValue::Unrestricted    => {contains_unknown_children = true;},
                    } //End match evaluate
                } //End for each conjunct

                //If there's unknown conjuncts the entire formula is unrestricted.
                if contains_unknown_children {TruthValue::Unrestricted}
                //Each conjunct is true, so the whole thing is true.
                else {TruthValue::MustBeTrue}
            },
            SimpleLogicNode::Disjunction(disjunct_vec) => {
                //Check each disjunct
                for disjunct in disjunct_vec.iter() {
                    match disjunct.evaluate(truth_values) {
                        TruthValue::MustBeTrue      => (return TruthValue::MustBeTrue),
                        TruthValue::MustBeFalse     => (),
                        TruthValue::Unrestricted    => {contains_unknown_children = true;},
                    } //End match evaluate
                } //End for each disjunct

                //If there's unknown disjuncts the entire formula is unrestricted.
                if contains_unknown_children {TruthValue::Unrestricted}
                //Each disjunct is false, so the whole thing is false.
                else {TruthValue::MustBeFalse}
            },
        } //End match self
    } //End evaluate
} //End impl LogicFormula for SimpleLogicNode

//MISCELLANEOUS HELPER FUNCTIONS

///Evaluates a single literal.
///literal - the literal (sign bit and variable index)
///truth_values - the known values that literals have
fn evaluate_single_literal(literal: u32, truth_values : &HashMap<u32,bool>) -> TruthValue {
    let variable_index = literal & VARIABLE_INDEX_MASK;
    match truth_values.get(&variable_index) {
        Some(known_value) => {
            if *known_value == (literal & NEGATIVITY_FLAG == 0) {TruthValue::MustBeTrue}
            else {TruthValue::MustBeFalse}
        },
        None => TruthValue::Unrestricted
    } //End match truth value
} //End evaluate_single_literal
