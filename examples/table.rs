use term_kit::table::Table;
use crossterm::style::Color;

fn main() {
    // Table 1: Basic table with default settings
    let mut table1 = Table::new("Basic Table".to_string(), 3, None, None, None);
    table1.set_cell(0, 0, "A1".to_string());
    table1.set_cell(0, 1, "B1".to_string());
    table1.set_cell(1, 0, "A2: I have a long value for some reason".to_string());
    table1.render().unwrap();

    // Table 2: Table with custom border and title color
    let mut table2 = Table::new(
        "Custom Colors Table".to_string(),
        3,
        Some(Color::Cyan),
        Some(Color::Magenta),
        None,
    );
    table2.set_cell(0, 0, "A1".to_string());
    table2.set_cell(0, 1, "B1".to_string());
    table2.set_cell(1, 0, "A2 with more content".to_string());
    table2.render().unwrap();

    // Table 3: Table with customized column count
    let mut table3 = Table::new("Customized Columns".to_string(), 3, None, None, None);
    table3 = table3.with_column_count_override(4);  // Override the default column count to 4
    table3.set_cell(0, 0, "A1".to_string());
    table3.set_cell(0, 1, "B1".to_string());
    table3.set_cell(0, 2, "C1".to_string());
    table3.set_cell(0, 3, "D1".to_string());
    table3.set_cell(1, 0, "A2".to_string());
    table3.set_cell(1, 1, "B2".to_string());
    table3.set_cell(1, 2, "C2".to_string());
    table3.set_cell(1, 3, "D2".to_string());
    table3.render().unwrap();

    // Table 4: Table with custom data color
    let mut table4 = Table::new(
        "Custom Data Color Table".to_string(),
        3,
        Some(Color::Green),
        Some(Color::Blue),
        None,
    );
    table4.set_cell(0, 0, "A1".to_string());
    table4.set_cell(0, 1, "B1".to_string());
    table4.set_cell(1, 0, "A2".to_string());
    table4.set_cell(1, 1, "B2".to_string());
    table4.render().unwrap();

    // Table 5: Table with varied content lengths to showcase dynamic column width adjustment
    let mut table5 = Table::new("Dynamic Width Table".to_string(), 3, None, None, None);
    table5.set_cell(0, 0, "Short".to_string());
    table5.set_cell(0, 1, "Medium Length".to_string());
    table5.set_cell(1, 0, "A much longer string that will stretch the column width".to_string());
    table5.set_cell(1, 1, "Short again".to_string());
    table5.set_cell(2, 0, "Some short value".to_string());
    table5.render().unwrap();
}
