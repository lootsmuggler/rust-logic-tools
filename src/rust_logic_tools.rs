/** This file runs Rust Logic Tools.  This is a command line tool.  Read "USAGE_TEXT" below for details.
    Author: Steven Fletcher
    Created: 2020
    Last Updated: 05/05/2021
*/
mod formula_precomputer;
mod html_text;
mod logic;
mod truth_table_size_5;

use formula_precomputer::*;
use html_text::*;
use std::env;
use std::io::Write;
use std::path::*;
use std::time::SystemTime;
use truth_table_size_5::*;
use std::fs::create_dir_all;

const USAGE_TEXT : &str =
"\nUsage: rust_logic_tools [-n {1 | 2 | 3 | 4 | 5}] [-output {html | text}]\n\
At present, this program generates a large number of boolean formulas, calculates their truth tables, and determines \
the smallest formula for each truth table.  If the output is text mode, it just outputs all the formulas generated in \
a text file formulalist.txt.  This is more for testing purposes.  If the output is html mode, it generates pretty \
printed html files named truthtablesX.htm, where X is an integer.  In html mode, it shows the truth tables and \
displays the formula with the least binary operators, followed by a list of all the formulas with that truth table.\n\
The output is stored in the folder [user]\\Documents\\Loot Smuggler\\Rust Logic Tools\\\n\
The defaults are -n 3 and -output text if you don't enter any parameters.\n\
At present, the program is intractable for n >= 4.  I'm planning to make this work for n = 5 somehow.\n\n\
Parameters:\n\
-n number determines the number of booleans per formula to precompute\n\
-output html causes the output to be output as multiple .html files\n\
-output text causes the output to be output as a .txt file";

fn main() {
    let start_time = SystemTime::now();

    //Parameters
    let mut html_mode                    : bool = false;
    let mut num_booleans_to_precompute   : u32 = 3;

    //Read arguments.
    let mut argument_mode = ArgumentMode::Default;
    let mut env_iterator = env::args();
    env_iterator.next(); //Skip the name of the program

    for argument in env_iterator {
        match argument_mode {
            ArgumentMode::Default => {
                if argument == "-n" {
                    argument_mode = ArgumentMode::N;
                }
                else if argument == "-output" {
                    argument_mode = ArgumentMode::Output;
                }
                else {
                    argument_mode = ArgumentMode::Error;
                    break;
                }
            }, //End ArgumentMode::Default
            ArgumentMode::Error => break,
            ArgumentMode::N => {
                match argument.parse::<u32>() {
                    Ok(number) => {
                        if number == 0 || number > MAX_BOOLEANS_TO_PRECOMPUTE {
                            argument_mode = ArgumentMode::Error;
                            break;
                        }

                        //else
                        num_booleans_to_precompute = number;
                        argument_mode = ArgumentMode::Default;
                    },
                    Err(_) => {
                        argument_mode = ArgumentMode::Error;
                        break;
                    }
                } //End match parse argument
            }, //End ArgumentMode::N
            ArgumentMode::Output => {
                if argument == "html" {
                    argument_mode = ArgumentMode::Default;
                    html_mode = true;
                }
                else if argument == "text" {
                    argument_mode = ArgumentMode::Default;
                    html_mode = false;
                }
                else {
                    argument_mode = ArgumentMode::Error;
                    break;
                }
            } //End ArgumentMode::Output
        } //End match mode
    } //End for each argument

    //If the argument mode isn't the default
    match argument_mode {
        ArgumentMode::Default => {},
        _ => {
            panic!(USAGE_TEXT);
        }
    } //End match argument_mode to make sure it is the default

    //Compute truth tables for all non-trivial CNFs and DNFs with the specified number of booleans.
    let tt_bucket_vec : Vec<LogicFormulaBucket> = generate_truth_tables_with_up_to_n_variables(num_booleans_to_precompute);

    //Create the output directory.
    let output_directory = generate_output_directory();

    //Generate the names of the booleans.
    let mut boolean_name_list = Vec::with_capacity(num_booleans_to_precompute as usize);
    for i in 1..=num_booleans_to_precompute {
        boolean_name_list.push(format!("p{}", i));
    }

    //Write the data to file.
    if html_mode {
        write_formula_list_to_html_files(&output_directory, &tt_bucket_vec, &boolean_name_list);
    }
    else {
        write_formula_list_to_text_file(&output_directory, tt_bucket_vec, &boolean_name_list);
    }

    //End the program.
    let end_time = SystemTime::now();
    println!("Total Execution Time = {:?}", end_time.duration_since(start_time));
} //End main

//CONSTANTS////////////////////////////////////////////////////////////////////////////////////////////////////////////
const MAX_BOOLEANS_TO_PRECOMPUTE : u32 = 5;
const NUM_TRUTH_TABLES_PER_FILE : u32 = 256;

const HTML_FILE_EXTENSION: &str = "htm";
const TRUTH_TABLE_FILE_NAME_PREFIX : &str = "truthtables";
const TRUTH_TABLE_SUBDIRECTORIES : [&str;2] = ["Loot Smuggler", "Rust Logic Tools"];

const FORMULA_LIST_FILE_NAME : &str = "formulalist.txt";

//CLASSES//////////////////////////////////////////////////////////////////////////////////////////////////////////////
///This enum enumerates different ArgumentModes for parsing the command line arguments.
enum ArgumentMode {
    Default,
    N,
    Output,

    Error
} //End enum ArgumentMode

//FUNCTIONS////////////////////////////////////////////////////////////////////////////////////////////////////////////
///Generate the output directory.
///Returns the output directory as a PathBuf
fn generate_output_directory() -> PathBuf {
    //Figure out the directory to write the files to.
    let documents_dir_option = get_documents_directory();
    let mut table_dir_path =
        match documents_dir_option {
            Some(path) => path,
            None => panic!("Document directory not found.")
        };

    for subdirectory in TRUTH_TABLE_SUBDIRECTORIES.iter() {
        table_dir_path.push(subdirectory);
    } //End for each subdirectory

    //Create the directory.
    match create_dir_all(table_dir_path.clone()) {
        Ok(result) => {},
        Err(message) => {
            println!("path = {}", table_dir_path.to_str().unwrap());
            panic!("{}", message);
        }
    };

    return table_dir_path;
} //End generate_output_directory

fn get_documents_directory() -> Option<PathBuf> {dirs_next::document_dir()}

///Writes the list of formulas to html files.
///table_dir_path is the directory to write the files to
///tt_bucket_vec is the Vec of all the truth tables with the formulas mapped to them
fn write_formula_list_to_html_files(table_dir_path : &PathBuf, tt_bucket_vec : &Vec<LogicFormulaBucket>,
                                    boolean_name_list : &Vec<String>)
{
    //Print the truth tables to multiple html files.
    let num_truth_tables = tt_bucket_vec.len() as u32;
    let num_truth_files : u32 =
        if num_truth_tables < NUM_TRUTH_TABLES_PER_FILE {1}
        else {num_truth_tables / NUM_TRUTH_TABLES_PER_FILE};

    //Save all the truth table files.
    let mut truth_table : u32 = 0;
    for file_index in 0..num_truth_files {
        let mut truth_table_html_generator = HtmlGenerator::new();

        let end_point : u32 =
            if file_index + 1 == num_truth_files {num_truth_tables}
            else {truth_table + NUM_TRUTH_TABLES_PER_FILE};

        //For each truth table in this file
        while truth_table < end_point {
            let html_result : Result<(),String> = add_html_for_truth_table_size_5(&mut truth_table_html_generator,
                                                                                  truth_table, &truth_table.to_string(), &boolean_name_list);
            match html_result {
                Ok(()) => (),
                Err(error_message) => println!("{}", error_message),
            };

            tt_bucket_vec[truth_table as usize].add_html_for_formula_list(&mut truth_table_html_generator,
                                                                          &boolean_name_list);

            //Increment the counter.
            truth_table = truth_table + 1;
        } //End for each truth table in this file

        //Generate the html for the truth tables.
        let truth_table_html = format!("{}", truth_table_html_generator);

        //Determine the html filepath.
        let mut html_filepath = table_dir_path.clone();
        let html_filename : String = format!("{}{}.{}", TRUTH_TABLE_FILE_NAME_PREFIX, file_index,
                                             HTML_FILE_EXTENSION);
        html_filepath.push(html_filename);

        //Write the html file.
        let mut tt_html_file = std::fs::File::create(html_filepath).expect("create failed");
        tt_html_file.write_all(truth_table_html.as_bytes()).expect("write failed");
        println!("Truth table data written to file {}", file_index);
    } //End for each truth table file
} //End write_formula_list_to_html_files

///Writes the list of formulas to html files.
///table_dir_path is the directory to write the files to
///tt_bucket_vec is the Vec of all the truth tables with the formulas mapped to them
fn write_formula_list_to_text_file(table_dir_path : &PathBuf, tt_bucket_vec : Vec<LogicFormulaBucket>,
                                   boolean_name_list : &Vec<String>)
{
    //Determine the formula list filepath.
    let mut formula_list_filepath = table_dir_path.clone();
    formula_list_filepath.push(FORMULA_LIST_FILE_NAME);

    //Write the formula list file.
    let mut formula_list_file = std::fs::File::create(&formula_list_filepath).expect("create failed");
    for bucket in tt_bucket_vec {
        formula_list_file.write_all(bucket.get_formula_list_as_text(&boolean_name_list).as_bytes()).expect("write failed");
    }

    println!("Formula list written to file {}", formula_list_filepath.to_str().unwrap());
} //End write_formula_list_to_text_file