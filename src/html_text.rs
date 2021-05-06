/** This file generates the text of an Html page.
    Author: Steven Fletcher
    Created: 01/17/2021
    Last Updated: 03/29/2021
*/
use std::fmt;

///This struct is used to generate the text of an Html page.
///A limitation of HtmlGenerator is that it can't nest tables/lists inside other tables/lists.  It's not built for that
///level of versatility.
pub struct HtmlGenerator {
    html_header : String,
    body_tag_properties : String,
    html_body : String,

    current_list_text : String,
    is_current_list_ordered : bool,

    current_table_text : String
} //End struct HtmlGenerator

impl HtmlGenerator {
    /**Constructor.
     */
    pub fn new() -> HtmlGenerator {
        HtmlGenerator {
            html_header : String::from("<html>\n"),
            body_tag_properties : String::from(""),
            html_body : String::from(""),

            current_list_text : String::from(""),
            is_current_list_ordered : false,

            current_table_text : String::from("")
        }
    } //End new

    /**Adds a header to the body of the text.  header_text is the text to be displayed in the header.  header_number is
     * the number of the header tag.  It should be from 1 to 6.  If it's value is wrong, this function will just use it
     * anyways.  The page will be incorrect when displayed.
     */
    pub fn add_header(&mut self, header_text : &str, header_number : u8) {
        let header_number_text = &header_number.to_string();
        self.html_body.push_str("<h");
        self.html_body += header_number_text;
        self.html_body.push_str(">");
        self.html_body.push_str(header_text);
        self.html_body.push_str("</h");
        self.html_body += header_number_text;
        self.html_body.push_str(">\n\n");
    } //End add_header

    /**Adds a paragraph to the body of the text.  The paragraph text is whatever text appears in the paragraph.  The
     * p tags are unnecessary.
     */
    pub fn add_paragraph(&mut self, paragraph_text : &str) {
        self.html_body.push_str("<p>");
        self.html_body.push_str(paragraph_text);
        self.html_body.push_str("</p>\n\n");
    } //End add_paragraph

    ///Adds a row to the current list.
    ///Parameter row_properties is used to set the internals of the row tag
    pub fn list_add_row(&mut self, row_properties : &str, data : &str) {
        self.current_list_text.push_str(&"<li ".to_owned());
        self.current_list_text.push_str(row_properties);
        self.current_list_text.push('>');
        self.current_list_text.push_str(data);
        self.current_list_text.push('\n');
    } //End list_add_row

    ///To work with lists:
    ///1. Call list_create
    ///2. For each row, call list_add_row
    ///3. When done, call list_end
    ///Only one list can exist at a time.  If you create a new list while the old one hasn't been ended, the old
    ///list will be lost.  If you don't end a list, it will never be added to the text.
    ///
    ///Parameter is_ordered should be set to true if the list should be ordered and false if the list should be
    ///unordered.
    ///Parameter list_properties is used to set the internals of the list tag.
    pub fn list_create(&mut self, is_ordered : bool, list_properties : &str) {
        self.is_current_list_ordered = is_ordered;
        if is_ordered {self.current_list_text = "<ol ".to_owned();}
        else {self.current_list_text = "<ul ".to_owned();}

        self.current_list_text.push_str(list_properties);
        self.current_list_text.push_str(">\n");
    } //End list_create

    ///Ends the current list.
    pub fn list_end(&mut self) {
        self.html_body.push_str(&self.current_list_text);

        if self.is_current_list_ordered {self.html_body.push_str("</ol>\n");}
        else {self.html_body.push_str("</ul>\n");}

        self.current_list_text = "".to_owned();
    } //End list_end

    /**Adds a data cell to the current table.
     */
    pub fn table_add_data(&mut self, data_properties : &str, data : &str) {
        self.table_add_data_cell(false, data_properties, data);
    } //End table_add_data

    /**Adds a data header cell to the current table.
     */
    pub fn table_add_header(&mut self, data_properties : &str, data : &str) {
        self.table_add_data_cell(true, data_properties, data);
    } //End table_add_header

    ///Adds a row to the current table.
    ///Parameter row_properties is used to set the internals of the row tag
    pub fn table_add_row(&mut self, row_properties : &str) {
        self.current_table_text.push_str(&"<tr ".to_owned());
        self.current_table_text.push_str(row_properties);
        self.current_table_text.push_str(">\n");
    } //End table_add_row

    ///To work with tables:
    ///1. Call table_create
    ///2. For each row, call table_add_row
    ///3. For each data cell in each row, call table_add_data or table_add_header
    ///4. When done, call table_end
    ///Only one table can exist at a time.  If you create a new table while the old one hasn't been ended, the old
    ///table will be lost.  If you don't end a table, it will never be added to the text.
    ///
    ///Parameter table_properties is used to set the internals of the table tag.
    pub fn table_create(&mut self, table_properties : &str) {
        self.current_table_text = "<table ".to_owned();
        self.current_table_text.push_str(table_properties);
        self.current_table_text.push_str(">\n");
    } //End table_create

    ///See table_create for information about working with tables.
    ///This variant creates a table that has a border with size equal to the parameter.  Any more complicated table tag
    ///will have to use table_create instead.
    pub fn table_create_with_border(&mut self, border : u8) {
        self.table_create(&format!("border=\"{}\"", border).to_owned());
    } //End table_create

    ///Ends the current table.
    pub fn table_end(&mut self) {
        self.html_body.push_str(&self.current_table_text);
        self.html_body.push_str("</table>\n");
        self.current_table_text = "".to_owned();
    } //End table_end

    //PRIVATE
    /**This helper function does the work for table_add_data and table_add_header.
     */
    fn table_add_data_cell(&mut self, is_header : bool, data_properties : &str, data : &str) {
        if is_header {
            self.current_table_text.push_str("<th ");
        }
        else {
            self.current_table_text.push_str("<td ");
        }

        self.current_table_text.push_str(data_properties);
        self.current_table_text.push('>');
        self.current_table_text.push_str(data);
        self.current_table_text.push('\n');
    } //End table_add_data_cell
} //End impl HtmlGenerator

/** Implementation of fmt::Display for HtmlGenerator.
*/
impl fmt::Display for HtmlGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "<html>\n".to_owned();
        if self.html_header.len() > 0 {
            text.push_str("<header>\n");
            text.push_str(&self.html_header);
            text.push_str("\n</header>\n");
        }
        text.push_str("<body ");
        text.push_str(&self.body_tag_properties);
        text.push_str(">\n");
        text.push_str(&self.html_body);
        text.push_str("</html>\n");

        write!(f, "{}", text)
    } //End fmt
} //End impl fmt::Display for HtmlGenerator
