use ascii_table::AsciiTable;

fn main() {
    let ascii_table = AsciiTable::default();
    let mut table: Vec<Vec<String>> = vec![vec![
        "Dec".to_string(),
        "Hex".to_string(),
        "Bin".to_string(),
    ]];
    for i in 0..128 {
        let binary = format!("{:08b}", i);
        let hex = format!("{:X}", i);
        let data = vec![i.to_string(), hex, binary];
        table.push(data);
    }
    ascii_table.print(table);
}
