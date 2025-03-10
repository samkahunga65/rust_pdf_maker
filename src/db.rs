use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ExcelRow {
    pub timestamp: String,
    pub phone_number: String,
    pub mno: String,
    pub message: String,
    pub status: String,
    pub msg_chars: String,
    pub cost_per_unit: String,
    pub total_cost: String,
}

pub fn db_conn(
    client_id: String,
    start_date: String,
    end_date: String,
) -> std::result::Result<Vec<ExcelRow>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = dotenv!("DATABASE_URL_PROD");

    let pool = match Pool::new(url) {
        Ok(e) => e,
        Err(e) => panic!("paniking coz of {:?}", e),
    };
    let mut conn = match pool.get_conn() {
        Ok(e) => e,
        Err(e) => panic!("paniking coz of {:?}", e),
    };

    let sql_query = format!("select date_format(dateCreated, '%Y-%M-%d') as dateCreated ,msisdn,mno,message, status, msgChars, totalCost, costPerMsg, totalCost  from SmsOut where  client = '{}'  and dateCreated >= '{}' and dateCreated <= '{}'", client_id, start_date, end_date);

    let selected_payments = match conn.query_map(sql_query, |row: mysql::Row| {
        let date_created: String = row.get(0).unwrap_or_default();
        let msisdn: String = row.get(1).unwrap_or_default();
        let mno: String = row.get(2).unwrap_or_default();
        let message: String = row.get(3).unwrap_or_default();
        let status: String = row.get(4).unwrap_or_default();
        let msg_chars: String = row.get(5).unwrap_or_default();
        let cost_per_msg: String = row.get(6).unwrap_or_default();
        let total_cost: String = row.get(7).unwrap_or_default();

        ExcelRow {
            timestamp: date_created,
            phone_number: msisdn,
            mno,
            message,
            status,
            msg_chars,
            cost_per_unit: cost_per_msg,
            total_cost,
        }
    }) {
        Ok(e) => e,
        Err(e) => panic!("paniking coz of {:?}", e),
    };

    Ok(selected_payments)
}
