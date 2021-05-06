/** This file precomputes logic formulas and maps them to truth tables.
    Author: Steven Fletcher
    Created: 03/26/2021
    Last Updated: 05/05/2021
*/
use crate::html_text::*;
use crate::logic::*;
use std::vec::Vec;
use crate::truth_table_size_5::*;

///This struct stores all the formulas that map to a specific truth table.
///The minimum cnf and dnf are stored separately from the other formulas.
pub struct LogicFormulaBucket {
    minimum_formula : Option<SimpleLogicNode>,
    formula_vector : Vec<SimpleLogicNode>
} //End struct LogicFormulaBucket

impl LogicFormulaBucket {
    //Adds a formula to this bucket.
    //formula is any SimpleLogicNode
    fn add_formula(&mut self, formula : SimpleLogicNode) {
        //Add the formula to the Vector.
        self.formula_vector.push(formula.clone());

        //Check to see if the formula is the minimum formula in the bucket.
        match &(self.minimum_formula) {
            Option::None => {self.minimum_formula = Some(formula.clone());},
            Option::Some(old_formula) => {
                //If the new formula is smaller, make it the minimum formula.
                if formula.count_binary_operators() < old_formula.count_binary_operators() {
                    self.minimum_formula = Some(formula.to_owned());
                }
            } //End Some
        };
    } //End add_formula

    ///Adds the Html for the formula list.
    pub fn add_html_for_formula_list(&self, html_generator : &mut HtmlGenerator, boolean_name_list : &Vec<String>) {
        //Minimum Formula
        let cnf_text =
            match self.minimum_formula.as_ref() {
                Some(formula) => formula.get_as_text(boolean_name_list),
                None                         => NONE_TEXT.to_string()
            };
        html_generator.add_paragraph(&format!("Minimum Formula: {}", cnf_text));

        //List of all formulas.
        for formula in &self.formula_vector {
            html_generator.list_add_row("", &formula.get_as_text(boolean_name_list));
        } //End for each formula

        html_generator.list_end();
    } //End add_html_for_formula_list

    pub fn get_formula_list_as_text(&self, boolean_name_list : &Vec<String>) -> String {
        let mut formula_list_text = "".to_string();

        //List of all formulas.
        for formula in &self.formula_vector {
            formula_list_text.push_str(&formula.get_as_text(boolean_name_list));
            formula_list_text.push('\n');
        } //End for each formula

        formula_list_text
    }  //End get_formula_list_as_text
} //End impl LogicFormulaBucket

///Generates all the truth tables with up to n variables.  Also maps a ton of boolean formulas to those truth tables.
///Trivial subformulas like p & p or p | ~p do not appear.
///tt_computer is the computation struct that computes the truth tables.
///For this function n <= 5 to avoid overflow.  For large n, this function is intractable anyways.  It's O(16^n).
pub fn generate_truth_tables_with_up_to_n_variables(n : u32) -> Vec<LogicFormulaBucket> {
    //Ignore n < 1.
    if n < 1 {
        panic!("Cannot generate truth tables for n < 1.");
    } //End if n < 1

    //This object generates CNF and DNF formulas.
    let two_to_n  = compute_two_to_n(n) as usize;
    let num_truth_tables = compute_two_to_two_to_n(n) as usize;

    //Create a bucket of formulas for each truth table.
    let mut formula_buckets: Vec<LogicFormulaBucket> = Vec::with_capacity(num_truth_tables);
    for _i in 0..num_truth_tables {
        formula_buckets.push(LogicFormulaBucket {
            minimum_formula : None,
            formula_vector : Vec::with_capacity(two_to_n)
        });
    } //End for each bucket to add

    formula_buckets[0].add_formula(SimpleLogicNode::False);
    formula_buckets[num_truth_tables - 1].add_formula(SimpleLogicNode::True);

    //Compute n factorial.
    let mut n_factorial = 1;
    for i in 2..=n {n_factorial = n_factorial * i;}

    //This isn't the correct capacity, but it should be close enough.
    let literal_subarray_capacity = (2 * n + n_factorial * compute_two_to_n(n)) as usize;

    //Store all different possible subarrays of the variables to generate truth tables for.
    let mut literal_subarray_vec : Vec<Vec<u32>> = Vec::with_capacity(literal_subarray_capacity);

    //Create all the possible literals.
    for i in 1..=n {
        literal_subarray_vec.push(vec![i]);
        literal_subarray_vec.push(vec![i | NEGATIVITY_FLAG]);
    } //End for each boolean

    //If n is at least 2, add pairs of literals.
    if n >= 2 {
        //Create each clause with 2 literals.
        for i in 1..=n {
            for j in i+1..=n {
                literal_subarray_vec.push(vec![i, j]);
                literal_subarray_vec.push(vec![i | NEGATIVITY_FLAG, j]);
                literal_subarray_vec.push(vec![i, j | NEGATIVITY_FLAG]);
                literal_subarray_vec.push(vec![i | NEGATIVITY_FLAG, j | NEGATIVITY_FLAG]);
            } //End for each second literal of the pair
        } //End for each first literal of the pair
    } //End if n is at least 2

    //If n is at least 3, add triples of literals.
    if n >= 3 {
        //Create each clause with 3 literals.
        for i in 1..=n {
            for j in i+1..=n {
                for k in j+1..=n {
                    for flagged_boolean_indices_vec in AssignFlagsIterator::new(vec![i, j, k])
                    {
                        literal_subarray_vec.push(flagged_boolean_indices_vec);
                    } //End for each Vec of flagged boolean indices
                } //End for the third literal of the subarray
            } //End for each second literal of the subarray
        } //End for each first literal of the subarray
    } //End if n is at least 3

    //If n is at least 4, add quadruples of literals.
    if n >= 4 {
        //Create each clause with 4 literals.
        for i in 1..=n {
            for j in i+1..=n {
                for k in j+1..=n {
                    for l in k+1..=n {
                        for flagged_boolean_indices_vec in AssignFlagsIterator::new(vec![i, j, k, l])
                        {
                            literal_subarray_vec.push(flagged_boolean_indices_vec);
                        } //End for each Vec of flagged boolean indices
                    } //End for the fourth literal of the subarray
                } //End for the third literal of the subarray
            } //End for each second literal of the subarray
        } //End for each first literal of the subarray
    } //End if n is at least 4

    //If n is at least 5, add quintuples of literals.
    if n >= 5 {
        //Create each clause with 5 literals.
        for i in 1..=n {
            for j in i+1..=n {
                for k in j+1..=n {
                    for l in k+1..=n {
                        for m in l+1..=n {
                            for flagged_boolean_indices_vec in AssignFlagsIterator::new(vec![i, j, k, l])
                            {
                                literal_subarray_vec.push(flagged_boolean_indices_vec);
                            } //End for each Vec of flagged boolean indices
                        } //End for the fifth literal of the subarray
                    } //End for the fourth literal of the subarray
                } //End for the third literal of the subarray
            } //End for each second literal of the subarray
        } //End for each first literal of the subarray
    } //End if n is at least 5

    //Output all subarrays.
/*    for literal_subarray in &literal_subarray_vec {
        let mut literals_vec = Vec::with_capacity(literal_subarray.len());
        for literal_index in literal_subarray {
            literals_vec.push(SimpleLogicNode::Literal(*literal_index));
        }

        println!("{}", SimpleLogicNode::Conjunction(literals_vec).get_as_text(&boolean_name_array))
    } //End for each literal subarray
*/
    //Generate the normal formulas.
    let mut nf_generator = NormalFormulaGenerator::new(formula_buckets, literal_subarray_vec, n);
    nf_generator.generate_all_normal_formulas();
    nf_generator.formula_buckets
} //End generate_truth_tables_with_up_to_5_variables

//PRIVATE//////////////////////////////////////////////////////////////////////////////////////////////////////////////
const BOOLEAN_NAME_ARRAY : [&str;5] = ["p1", "p2", "p3", "p4", "p5"];
const NONE_TEXT : &str = "NONE";

///This struct generates Vecs that have all the assorted combinations of positive or negative flags.
///The results are Vec<SimpleLogicNode>.
struct AssignFlagsIterator {
    boolean_indexes_vec : Vec<u32>,
    configuration : usize,
    num_configurations : usize
} //End struct AssignFlagsIterator

impl AssignFlagsIterator {
    ///Creates a new AssignSignsIterator.
    ///boolean_indexes_vec is the vector of boolean indexes to generate positive/negative flags for
    fn new(boolean_indexes_vec : Vec<u32>) -> AssignFlagsIterator {
        let num_booleans = (&boolean_indexes_vec).len() as u32;
        AssignFlagsIterator {
            boolean_indexes_vec : boolean_indexes_vec,
            configuration : 0,
            num_configurations : (compute_two_to_n(num_booleans)) as usize
        }
    } //End new
} //End impl AssignFlagsIterator

impl Iterator for AssignFlagsIterator {
    type Item = Vec<u32>;

    ///AssignFlagsIterator.next to implement Iterator.next
    fn next(&mut self) -> Option<Self::Item> {
        //If the AssignSignsIterator is out of configurations
        if self.configuration >= self.num_configurations {
            return None;
        } //End if the AssignSignsIterator is out of configurations

        //FIX ALL OF THIS
        //Create a Vec for the configuration of literals.
        let mut boolean_indexes_with_signs_vec :Vec<u32> = Vec::with_capacity(self.boolean_indexes_vec.len());

        //The bitmask is used to check whether a specific boolean index must be positive or negative.
        //0 = negative, 1 = positive
        let mut bitmask :usize = 1;
        for index_index in 0..self.boolean_indexes_vec.len() {
            //Figure out the literal.
            let literal =
                if self.configuration & bitmask == 0 {self.boolean_indexes_vec[index_index] | NEGATIVITY_FLAG}
                else {self.boolean_indexes_vec[index_index]};

            //Add the literal to the vec (as a SimpleLogicNode).
            boolean_indexes_with_signs_vec.push(literal);

            //Update the bitmask
            bitmask = bitmask << 1;
        } //End for each boolean index

        //Increment
        self.configuration = self.configuration + 1;

        Some(boolean_indexes_with_signs_vec)
    } //End next
} //End impl Iterator for AssignFlagsIterator

struct NormalFormulaGenerator {
    formula_buckets : Vec<LogicFormulaBucket>,  //Stores the final results
    literal_configurations: Vec<Vec<u32>>,      //The different possible configurations of literals
    tt_computer : TruthTableSize5Computer,      //Computes the truth tables for the formula buckets
} //End struct NormalFormulaGenerator

impl NormalFormulaGenerator {
    fn new(formula_buckets : Vec<LogicFormulaBucket>, literal_configurations : Vec<Vec<u32>>, n : u32)
        -> NormalFormulaGenerator
    {
        let tt_computer = TruthTableSize5Computer::new(n);
        NormalFormulaGenerator {
            formula_buckets : formula_buckets,
            literal_configurations: literal_configurations,
            tt_computer : tt_computer,
        }
    } //End new

    fn add_formula_to_buckets(&mut self, formula : SimpleLogicNode) {
        let truth_table = self.tt_computer.compute_truth_table(&formula);
        let mut formula_bucket = &mut self.formula_buckets[truth_table as usize];

        formula_bucket.add_formula(formula);
    } //End add_formula_to_buckets

    fn generate_all_normal_formulas(&mut self) {
        //Actually generate the formulas.
        let num_literal_configurations = self.literal_configurations.len();
        for i in 0..num_literal_configurations {
            let mut clause_builder = Vec::new();
            clause_builder.push(self.literal_configurations[i].clone());
            self.generate_all_normal_formulas_with_prefix(&mut Vec::new(), i);
        } //End for each literal configuration
    } //End generate_all_normal_formulas

    fn generate_all_normal_formulas_with_prefix(&mut self, prefix_clauses : &mut Vec<Vec<u32>>,
                                                clause_to_add_index : usize)
    {
        let new_clause = self.literal_configurations[clause_to_add_index].clone();
        let mut current_clauses = prefix_clauses.clone();

        //Check for unit clauses that are the opposite of other unit clauses.
        //Note: If the new clause is a unit, then the current clause will also be a unit because the arrays are
        //created in order of increasing size.
        if new_clause.len() == 1 {
            for current_clause in &current_clauses {
                //If the new clause is the opposite of another clause, don't generate any formulas with this prefix.
                //It would be a tautology or contradiction anyways.
                if current_clause[0] ^ new_clause[0] == NEGATIVITY_FLAG {return; }
            } //End for each prefix clause
        } //End if the new clause is a unit clause and the opposite of the current clause
        //Check whether the new clause is subsumed by one of the existing clauses.
        else {
            for current_clause in &current_clauses {
                //If the new clause is subsumed, don't generate any formulas with this prefix.
                if NormalFormulaGenerator::is_subarray_of(current_clause, &new_clause) { return; }
            } //End for each prefix clause
        } //End else the new clause isn't a unit clause

        //Add the clause to add.
        current_clauses.push(new_clause);

        //Turn all the clauses into conjunctions.
        let mut current_conjunctions: Vec<SimpleLogicNode> = Vec::with_capacity(current_clauses.len());
        for clause_integers_vec in &current_clauses {
            let mut clause_literals_vec = Vec::with_capacity(clause_integers_vec.len());
            for integer in clause_integers_vec {
                clause_literals_vec.push(SimpleLogicNode::Literal(*integer));
            } //End for each integer in the clause

            //If there is only 1 literal in clause, don't even wrap it in a conjunction
            if clause_literals_vec.len() == 1 {
                current_conjunctions.push(clause_literals_vec[0].clone());
            } //End if there is only 1 literal in clause, don't even wrap in a conjunction
            //Else there's multiple literals, so wrap the clause in a conjunction
            else {
                current_conjunctions.push(SimpleLogicNode::Conjunction(clause_literals_vec));
            } //End else there's multiple literals
        } //End for each current clause

        //If there's only 1 clause
        if current_conjunctions.len() == 1 {
            //Get the formula.
            let single_conjunction = current_conjunctions.pop().unwrap();

            //Only add the formula to the buckets if it's a literal.
            //Other single clause conjunctions will be added as the CNF version of a DNF.
            match single_conjunction {
                SimpleLogicNode::Literal(_) => {
                    self.add_formula_to_buckets(single_conjunction);
                },
                _ => ()
            };
        } //End if there's only 1 clause
        //Else there's more than 1 clause
        else {
            //Create the disjunction.
            let dnf_formula = SimpleLogicNode::Disjunction(current_conjunctions);

            //Add CNF and DNF formulas.
            self.add_formula_to_buckets(NormalFormulaGenerator::generate_cnf_from_dnf(&dnf_formula));
            self.add_formula_to_buckets(dnf_formula);
        } //End else there's more than 1 clause

        //Add subsequent clauses.
        let num_literal_configurations = self.literal_configurations.len();
        for i in clause_to_add_index+1..num_literal_configurations {
            self.generate_all_normal_formulas_with_prefix(&mut current_clauses, i);
        } //End for each possible next clause
    } //End generate_all_normal_formulas_with_prefix

    //Generates a CNF formula with the same literals as a DNF formula.  They are not in any way equivalent.
    //dnf_formula is the DNF formula to generate the CNF formula from
    //Returns the CNF formula generated.
    fn generate_cnf_from_dnf(dnf_formula : &SimpleLogicNode) -> SimpleLogicNode {
        match dnf_formula {
            SimpleLogicNode::False => {
                panic!("NormalFormulaGenerator.generate_cnf_from_dnf should never see SimpleLogicNode::False")
            },
            SimpleLogicNode::True => {
                panic!("NormalFormulaGenerator.generate_cnf_from_dnf should never see SimpleLogicNode::True")
            },
            SimpleLogicNode::Literal(_) => dnf_formula.to_owned(),
            SimpleLogicNode::Conjunction(operands) => {
                let mut flipped_operands = Vec::with_capacity(operands.len());
                for operand in operands {
                    flipped_operands.push(NormalFormulaGenerator::generate_cnf_from_dnf(operand));
                }
                SimpleLogicNode::Disjunction(flipped_operands)
            },
            SimpleLogicNode::Disjunction(operands) => {
                let mut flipped_operands = Vec::with_capacity(operands.len());
                for operand in operands {
                    flipped_operands.push(NormalFormulaGenerator::generate_cnf_from_dnf(operand));
                }
                SimpleLogicNode::Conjunction(flipped_operands)
            },
        } //End match dnf_formula
    } //End generate_cnf_from_dnf

    //This method determines whether the old array is a subarray of the new array.  However, it doesn't work on
    //arbitrary arrays.  The arrays must be sorted in increasing order.
    //old_array is the array that might be a subarray
    //new_array is the array that might be a superarray
    //It checks whether old_array is actually a subarray of new_array.
    fn is_subarray_of(old_array: &Vec<u32>, new_array: &Vec<u32>) -> bool {
        let old_array_length = old_array.len();
        let new_array_length = new_array.len();

        //If the arrays are the same size, it won't be a subarray (because all the arrays are different).
        if old_array_length == new_array_length {return false;}

        //The arrays are different sizes, so old_array might be a subarray of new_array.

        //Compare elements.
        //i is the index of old_array
        //j is the index of new_array
        let mut j: usize = 0;
        for i in 0..old_array_length {
            let without_flag = old_array[i] & VARIABLE_INDEX_MASK;
            while without_flag > new_array[j] & VARIABLE_INDEX_MASK {
                j = j + 1;

                //If we're at the end of superarray, it's not subsumed
                if j == new_array_length {return false}
            } //End while the current boolean of vec1 is more than the current boolean of vec2

            //if the booleans aren't identical
            if old_array[i] != new_array[j] {
                //In this case, without_flag < than the current boolean in vec2.
                //This means that subarray is not a subarray of superarray because without_flag isn't in it.
                return false;
            } //End else if it's a completely different boolean
        } //End for each Vector that might be subsumed

        //The old array must be a subarray of the new array if the code is here.
        return true;
    } //End is_subarray_of
} //End impl NormalFormulaGenerator

//Computes 2 to the power of n
//n is the power to raise 2 to
fn compute_two_to_n(n : u32) -> u32 {1 << n}

//Computes 2 to the power of 2 to the n.  This is 2^(2^n), not (2^2)^n.
//n is the power of 2 to raise 2 to
fn compute_two_to_two_to_n(n : u32) -> u32 {1 << (1 << n)}
