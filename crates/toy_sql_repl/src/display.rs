use tabled::builder::Builder;
use toy_sql_execution::ExecResponse;

pub fn display_response(res: ExecResponse) {
    match res {
        ExecResponse::Select(table_iter) => {
            let mut builder = Builder::default();

            // let row = rows.get(0).expect("For now assuming we get data back");

            let columns: Vec<String> = table_iter
                .columns
                .iter()
                .map(|col| col.name.to_string())
                .collect();

            builder.set_header(&columns);
            for row in table_iter {
                builder.push_record(columns.iter().map(|col| row.get(col)));
            }
            println!("{}", builder.build())
        }
        _ => println!("{res}"),
    }
}
