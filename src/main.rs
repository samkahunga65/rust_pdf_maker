use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rust_xlsxwriter::Workbook;
use serde::Deserialize;

mod db;

#[derive(Deserialize)]
struct ExcelPayLoad2 {
    client: String,
    start_date: String,
    end_date: String,
}

async fn generate_excel(data: web::Json<ExcelPayLoad2>) -> impl Responder {
    let mut workbook = Workbook::new();
    let bod = data.into_inner();

    let client = bod.client.clone();
    let start_date = bod.start_date.clone();
    let end_date = bod.end_date.clone();

    let res = match db::db_conn(client, start_date, end_date) {
        Ok(e) => e,
        Err(e) => panic!("paniking coz of {:?}", e),
    };

    let columns: Vec<String> = vec![
        String::from("Timestamp"),
        String::from("Phone Number"),
        String::from("MNO"),
        String::from("Message"),
        String::from("Status"),
        String::from("Msg Chars"),
        String::from("Units"),
        String::from("Cost Per Unit(KSH)"),
        String::from("Total Cost(KSH)"),
    ];
    let rows = res;
    println!("generating excel for {} records", &rows.len());

    let _worksheet = workbook.add_worksheet();
    let gen_err = "I have an Issue";
    //write column names
    for (index, col_name) in columns.iter().enumerate() {
        _worksheet.write(0, index as u16, col_name).expect(gen_err);
    }

    for x in 0..rows.len() {
        let curr_row = x + 1;
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 0, &curr_row_data.timestamp)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 1, &curr_row_data.phone_number)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 2, &curr_row_data.mno)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 3, &curr_row_data.message)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 4, &curr_row_data.status)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 5, &curr_row_data.msg_chars)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 6, &curr_row_data.cost_per_unit)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 7, &curr_row_data.cost_per_unit)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
        if let Some(curr_row_data) = rows.get(x) {
            _worksheet
                .write(curr_row as u32, 0, &curr_row_data.timestamp)
                .expect(gen_err);
        } else {
            eprintln!("Row at index {} is missing.", x);
        }
    }
    let excel_data = workbook.save_to_buffer().expect("Cannot Build Workbook");
    HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"report.xlsx\"",
        ))
        .body(excel_data)
}
async fn test() -> impl Responder {
    HttpResponse::Ok().body("hello rs")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App started!");

    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(test))
            .route("/generate-excel", web::post().to(generate_excel))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
