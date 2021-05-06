Usage: rust_logic_tools [-n {1 | 2 | 3 | 4 | 5}] [-output {html | text}]
At present, this program generates a large number of boolean formulas, calculates their truth tables, and determines the smallest formula for each truth table.  If the output is text mode, it just outputs all the formulas generated in a text file formulalist.txt.  This is more for testing purposes.  If the output is html mode, it generates pretty printed html files named truthtablesX.htm, where X is an integer.  In html mode, it shows the truth tables and displays the formula with the least binary operators, followed by a list of all the formulas with that truth table.
The output is stored in the folder [user]\Documents\Loot Smuggler\Rust Logic Tools\
The defaults are -n 3 and -output text if you don't enter any parameters.
At present, the program is intractable for n >= 4.  I'm planning to make this work for n = 5 somehow.

Parameters:
-n number determines the number of booleans per formula to precompute
-output html causes the output to be output as multiple .html files
-output text causes the output to be output as a .txt file