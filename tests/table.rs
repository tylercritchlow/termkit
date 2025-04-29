#[cfg(test)]
mod tests {
    use term_kit::table::Table;

    #[test]
    fn test_table_creation() {
        let table = Table::new("Test Table".to_string(), 3);
        assert_eq!(table.title, "Test Table");
        assert_eq!(table.rows, 3);
        assert_eq!(table.columns, 2);
        assert_eq!(table.data.borrow().len(), 3);
        assert_eq!(table.data.borrow()[0].len(), 2);
    }

    #[test]
    fn test_table_with_custom_data() {
        let data = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        let table = Table::new("Data Table".to_string(), 2).with_data(data.clone());
        assert_eq!(table.data, data.into());
    }

    #[test]
    fn test_column_override() {
        let table = Table::new("Wide Table".to_string(), 2).with_column_count_override(4);
        assert_eq!(table.columns, 4);
        assert_eq!(table.data.borrow()[0].len(), 4);
    }

    #[test]
    fn test_set_cell() {
        let table = Table::new("Cell Test".to_string(), 3);
        table.set_cell(1, 1, "Value".to_string());
        assert_eq!(table.data.borrow()[1][1], "Value");
    }

    #[test]
    fn test_calculate_column_widths() {
        let table = Table::new("Width Test".to_string(), 2);
        table.set_cell(0, 0, "Short".to_string());
        table.set_cell(1, 0, "LongerValue".to_string());
        table.set_cell(0, 1, "X".to_string());

        let widths = table.calculate_column_widths();
        assert_eq!(widths[0], 11); // "LongerValue" length
        assert_eq!(widths[1], 1); // "X" length
    }

    #[test]
    fn test_custom_writer() {
        let buffer = Vec::new();
        let table = Table::new("Writer Test".to_string(), 1).with_custom_writer(buffer);

        // We're mainly testing that it compiles correctly here
        assert_eq!(table.rows, 1);
    }

    #[test]
    fn test_render_output() {
        let buffer = Vec::new();
        let table = Table::new("Render Test".to_string(), 2).with_custom_writer(buffer);

        table.set_cell(0, 0, "A1".to_string());
        table.set_cell(0, 1, "B1".to_string());
        table.set_cell(1, 0, "A2".to_string());
        table.set_cell(1, 1, "B2".to_string());

        let result = table.render();
        assert!(result.is_ok());

        // We could check the exact output, but it would be complex due to ANSI codes
        // This test primarily ensures rendering doesn't panic
    }

    #[test]
    fn test_out_of_bounds_set_cell() {
        let table = Table::new("Bounds Test".to_string(), 2);
        // These should silently fail and not panic
        table.set_cell(-1, 0, "Invalid".to_string());
        table.set_cell(0, -1, "Invalid".to_string());
        table.set_cell(10, 0, "Invalid".to_string());
        table.set_cell(0, 10, "Invalid".to_string());

        // Check table is still in a valid state
        assert_eq!(table.data.borrow().len(), 2);
        assert_eq!(table.data.borrow()[0].len(), 2);
    }
}
