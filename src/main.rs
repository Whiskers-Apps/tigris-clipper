use forms::handle_forms;
use results::handle_results;
use tigris_rs::features::api::{
    get_extension_request,
    RequestType::{FormResults, GetResults, RunAction},
};

pub mod db;
pub mod forms;
pub mod icons;
pub mod results;

const EXTENSION_ID: &str = "clipper";

fn main() {
    let request = get_extension_request();

    match request.request_type {
        GetResults => {
            handle_results(request.get_results_request.unwrap());
        }
        RunAction => {}
        FormResults => {
            handle_forms(request.form_results_request.unwrap());
        }
    }
}
