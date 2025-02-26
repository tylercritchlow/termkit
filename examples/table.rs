use term_kit::table::Table;

fn main() {
    let table = Table::new("Table Example".to_string(), 10, 10, None, None);

    table.render().unwrap();
}
