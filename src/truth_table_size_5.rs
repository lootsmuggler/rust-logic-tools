/** This file stores truth tables and can format them to be output as html.
    Author: Steven Fletcher
    Created: 01/28/2021
    Last Updated: 05/05/2021
*/
use crate::html_text::*;
use crate::logic::*;

///The truth tables in this file are only for a tables containing at most 5 booleans.  For tables with fewer booleans,
///the beginning of the data will be all zeros.
///
///The truth tables in this file don't even have a struct.  The data is just a u32.  Callers will have to keep track of
///which booleans are actually in the table and what order they're in.
///
///The data is stored as follows for booleans (p, q, r):
///bit 7: 1 if p=T, q=T, r=T is True
///bit 6: 1 if p=T, q=T, r=F is True
///bit 5: 1 if p=T, q=F, r=T is True
///bit 4: 1 if p=T, q=F, r=F is True
///bit 3: 1 if p=F, q=T, r=T is True
///bit 2: 1 if p=F, q=T, r=F is True
///bit 1: 1 if p=F, q=F, r=T is True
///bit 0: 1 if p=F, q=F, r=F is True
///
///Creating SimpleLogicNodes to feed into TruthTableSize5Computer: Please note that the booleans are numbered from
///1 to n.  There is no 0 boolean.  This works fine.

//To get Table1 or Table2, just use bitwise or.  Etc.  Table1 <-> Table2 would require bitwise negation of bitwise xor.

//Table constants.
const TABLE_BORDER_THICKNESS : u8 = 1;
const TABLE_HEADER_NUMBER : u8 = 3;
const TAG_EMPTY_PROPERTIES : &str = "";
const T_TEXT : &str = "T";
const F_TEXT : &str = "F";
//const T_F_TEXT_LIST : [&str; 2] = [T_TEXT, F_TEXT];

///Adds the html for a truth table of size at most 5 to an HtmlGenerator.
///
///html_generator is the object that is generating the Html
///table_conclusion is the conclusion column of the table being generated.  1 is True, 0 is False.  The most
///significant bit is the first row of the conclusion.
///table_title is the title to be displayed at the top of the table
///boolean_name_list is a list of the names of the booleans used in the table.  These will be the headers of each
///column.
///
///Return value: () if the function was successful, an error message if the function failed.
pub fn add_html_for_truth_table_size_5(html_generator : &mut HtmlGenerator, table_conclusion : u32, table_title : &str,
    boolean_name_list : &Vec<String>) -> Result<(),String>
{
    //Add the header.
    html_generator.add_header(table_title, TABLE_HEADER_NUMBER);

    let num_booleans : usize = boolean_name_list.len();
    if num_booleans > 5 {
        return Err(format!(
            "INTERNAL ERROR: Too many booleans {} for table {} in get_html_for_truth_table_size_5",
            num_booleans, table_title));
    } //End if the name list contains the wrong number of booleans

    //Create the table's header.
    html_generator.table_create_with_border(TABLE_BORDER_THICKNESS);
    html_generator.table_add_row(TAG_EMPTY_PROPERTIES);
    for i in 0..num_booleans {
        html_generator.table_add_header(TAG_EMPTY_PROPERTIES, &boolean_name_list[i]);
    }
    html_generator.table_add_header(TAG_EMPTY_PROPERTIES, table_title);

    //The bitmask for accessing the conclusion from the table conclusion.
    //It should be fine to convert from usize to u32 because the number of booleans will be much less than u32.MAX.
    let num_booleans_u32 : u32 = num_booleans as u32;
    let mut bitmask : u32 =
        if num_booleans_u32 == 5 {
            //We can't use 1 << 32 - 1 because 1 << 32 would overflow.
            1 << 31
        }
        else {
            /* 1 << num_booleans is pow(2, num_booleans).  Subtracting 1 from that gives the number of bits to shift
            *  left to get the bit mask.
            */
            1 << ((1 << num_booleans) - 1)
        };

    //This variable stores the current values of each boolean.  The last boolean is stored in the least significant
    //bit, and each other boolean is stored in the direction of the most significant bit, but it won't use the entire
    //32 bits.
    let mut current_boolean_value : u32 = 1;
    for i in 1..num_booleans {
        current_boolean_value = current_boolean_value | (1 << i);
    } //End for each boolean (except the first one)

    loop {
        html_generator.table_add_row(TAG_EMPTY_PROPERTIES);

        //Add the columns identifying the booleans' values.
        for i in (0..num_booleans).rev() {
            //Either output T or F
            let t_or_f_value : &str =
                if current_boolean_value & (1 << i) > 0 {T_TEXT}
                else {F_TEXT};
            html_generator.table_add_data(TAG_EMPTY_PROPERTIES, t_or_f_value);
        } //End for each boolean

        //Add the conclusion column.
        let t_or_f_conclusion =
            if table_conclusion & bitmask > 0 {T_TEXT}
            else {F_TEXT};
        html_generator.table_add_data(TAG_EMPTY_PROPERTIES, t_or_f_conclusion);

        //Update the loop condition.
        if current_boolean_value == 0 {break;}
        //Else
        current_boolean_value = current_boolean_value - 1;

        //Adjust the bitmask to get the next line of the table
        bitmask = bitmask >> 1;
    } //End loop

    //Finish the table.
    html_generator.table_end();
    Ok(())
} //End add_html_for_truth_table_size_5

///This struct is used to compute truth tables with 5 or fewer booleans.
pub struct TruthTableSize5Computer {
    positive_bitmask_vec : Vec<u32>,
    negative_bitmask_vec : Vec<u32>
} //End struct TruthTableSize5Computer

impl TruthTableSize5Computer {
    ///Creates a TruthTableSize5Computer.
    ///num_booleans is the maximum number of booleans this TTS5Computer can compute the truth table of.
    pub fn new(num_booleans : u32) -> TruthTableSize5Computer {
        //If it's the wrong number of booleans, panic.
        if num_booleans > 5 || num_booleans == 0 {
            panic!("Invalid number of booleans {} in (truth_table_size_5.rs) compute_truth_table_size_5", num_booleans);
        }

        //Create the bitmasks for this number of booleans.
        let two_to_n_minus_1 = 1 << (num_booleans - 1);
        let mut positive_bitmask_vec : Vec<u32> = Vec::with_capacity(two_to_n_minus_1);
        if num_booleans == 1 {
            //This is the bitmask for 1 boolean.
            positive_bitmask_vec.push(0b10);
        } //End if there's only 1 boolean
        //Else there's at least 2 booleans
        else {
            //These are the bitmask for 2 booleans.
            //They're in backwards order because they're going to be reversed later.
            positive_bitmask_vec.push(0b1010);
            positive_bitmask_vec.push(0b1100);

            //For more than 2 booleans, copy each previous bitmask to appear twice in the new bitmask.
            //Then add a new bitmask that covers the first half of the truth table (for whatever boolean is being added).
            let mut num_bits = 4;
            for _i in 3..=num_booleans {
                //Duplicate each bitmask within itself.
                let vec_len = positive_bitmask_vec.len();
                for j in 0..vec_len {
                    let bitmask = positive_bitmask_vec[j];
                    positive_bitmask_vec[j] = (bitmask << num_bits) | bitmask;
                } //End for each bitmask in the Vec

                //Add the new bitmask - it must have at least 3 booleans.
                let mut new_bitmask = 0b111;
                for _j in 4..=num_bits {
                    new_bitmask = (new_bitmask << 1) | 1;
                } //End for each 1 to add to the new bitmask

                positive_bitmask_vec.push(new_bitmask << num_bits);

                //Double the number of bits for the next iteration.
                num_bits = num_bits << 1;
            } //End for each boolean after the 2nd

            //Reverse the vector to put the bitmasks in the right order.
            positive_bitmask_vec.reverse();
        } //End else there's at least 2 booleans

        //Create the negative bitmasks.
        let mut negative_bitmask_vec : Vec<u32> = Vec::with_capacity(two_to_n_minus_1);
        if num_booleans == 1 {
            //This is the bitmask for 1 boolean.
            negative_bitmask_vec.push(0b01);
        } //End if there's only 1 boolean
        //Else there's at least 2 booleans
        else {
            //Push the first negative bitmask.
            let first_positive_bitmask = positive_bitmask_vec[0];
            let first_negative_bitmask = first_positive_bitmask >> two_to_n_minus_1;
            negative_bitmask_vec.push(first_negative_bitmask);

            //Create a bitmask encompassing all the possible bits in the bitmasks.
            let total_bitmask = first_positive_bitmask | first_negative_bitmask;
            let num_bitmasks = positive_bitmask_vec.len();
            for i in 1..num_bitmasks {
                negative_bitmask_vec.push(!positive_bitmask_vec[i] & total_bitmask);
            } //End for each bitmask (except the first one)
        } //End else there's at least 2 booleans

        TruthTableSize5Computer {
            positive_bitmask_vec : positive_bitmask_vec,
            negative_bitmask_vec : negative_bitmask_vec
        }
    } //End new

    ///Computes a truth table.
    ///formula is the formula to compute the truth table of.
    pub fn compute_truth_table(&self, formula : &SimpleLogicNode) -> u32 {
        match formula {
            SimpleLogicNode::False => 0,
            SimpleLogicNode::True => self.positive_bitmask_vec[0] | self.negative_bitmask_vec[0],
            SimpleLogicNode::Literal(lit) => {
                //Just get the correct bitmask.
                let variable_index = get_variable_index(*lit) as usize;
                if is_positive_literal(*lit) {
                    self.positive_bitmask_vec[variable_index - 1]  //(-1 because booleans range from 1 to n)
                }
                else {
                    self.negative_bitmask_vec[variable_index - 1]  //(-1 because booleans range from 1 to n)
                }
            },
            SimpleLogicNode::Conjunction(operand_vec) => {
                //Conjunction of each operand.
                let mut truth_table = u32::MAX;
                for operand in operand_vec {
                    truth_table = truth_table & self.compute_truth_table(operand);
                } //End for each operand

                truth_table
            },
            SimpleLogicNode::Disjunction(operand_vec) => {
                //Disjunction of each operand.
                let mut truth_table = 0;
                for operand in operand_vec {
                    truth_table = truth_table | self.compute_truth_table(operand);
                } //End for each operand

                truth_table
            }
        } //End match formula
    } //End compute_truth_table

    ///Prints the bitmasks used by this TTS5Computer for testing purposes.
    pub fn print_bitmasks(&self) {
        print!("Positive: ");
        for bitmask in &self.positive_bitmask_vec {
            print!("{:032b}, ", bitmask);
        } //End for each bitmask
        println!();

        print!("Negative: ");
        for bitmask in &self.negative_bitmask_vec {
            print!("{:032b}, ", bitmask);
        } //End for each bitmask
        println!();
    } //End print_bitmasks
} //End impl TruthTableSize5Computer
