use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rust_xlsxwriter::Workbook;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExcelPayLoad {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

async fn generate_excel(data: web::Json<ExcelPayLoad>) -> impl Responder {
    let mut workbook = Workbook::new();
    let bod = data.into_inner();

    let columns = bod.columns.clone();
    let rows = bod.rows.clone();

    let _worksheet = workbook.add_worksheet();
    let gen_err = "I have an Issue";
    //write column names
    for (index, col_name) in columns.iter().enumerate() {
        _worksheet.write(0, index as u16, col_name).expect(gen_err);
    }

    for x in 0..rows.len() {
        let curr_row = x + 1;
        for y in 0..columns.len() {
            _worksheet
                .write(
                    curr_row as u32,
                    y as u16,
                    rows.get(x).expect(gen_err).get(y).expect(gen_err),
                )
                .expect(gen_err);
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
    HttpResponse::Ok().body("body")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App started!");
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(test))
            .route("/generate-excel", web::post().to(generate_excel))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
