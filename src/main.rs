use std::error::Error;

use tonic::transport::Channel;

use terminal::MainSelect;
use work::Work;

use crate::proto::db_api_client::DbApiClient;

mod file_work;
mod proto_work;
mod string_work;
mod terminal;
mod time_work;
mod work;

pub mod proto {
    tonic::include_proto!("db_api");
}

const MAX_SIZE: usize = 10;

async fn main_terminal(client: &mut DbApiClient<Channel>) {
    let mut vector: Vec<Work> = Vec::with_capacity(MAX_SIZE);

    'main_loop: loop {
        match terminal::select_print() {
            MainSelect::NewWork => {
                vector.push(
                    Work::from_vec(terminal::input_create_work_params())
                        .expect("From vec is empty"),
                );
            }

            MainSelect::EditWork => {
                println!("There are {} works in the array.", vector.len());

                let selected_work = terminal::user_input::<usize>("Select:") - 1;

                if selected_work < vector.len() {
                    if let Some(work) = vector.get_mut(selected_work).map(|w| w as &mut Work) {
                        work.edit(terminal::input_edit_work_params());
                    }
                }
            }

            MainSelect::ExportWorks => {
                let select = terminal::user_select(vec!["Export all", "Filter and export"]);
                match select {
                    1 => {
                        terminal::export_works(vector.clone());

                        for work in vector.clone() {
                            println!(
                                "{}",
                                proto_work::add_work(client, work)
                                    .await
                                    .expect("Failed to add work")
                            );
                        }
                    }
                    2 => {
                        terminal::export_filter_works(vector.clone());

                        for work in vector.clone() {
                            println!(
                                "{}",
                                proto_work::add_work(client, work)
                                    .await
                                    .expect("Failed to add work")
                            );
                        }
                    }
                    _ => continue 'main_loop,
                }
            }
            MainSelect::PrintReadable => {
                for work in &vector {
                    println!("{}", work);
                }
            }
            MainSelect::ImportFromJSON => {
                let select = terminal::user_select(vec![
                    "From custom path",
                    "Default saved file",
                    //"From the database",
                ]);
                match select {
                    1 => {
                        let path_buf = terminal::user_input_path_buf();
                        let add_works =
                            Work::from_vec_string(file_work::read_file(path_buf.as_path()));
                        vector.extend(add_works);
                    }
                    2 => {
                        let current_dir = file_work::get_current_path_buf();
                        if let Ok(vec) = file_work::dir(current_dir.join("saved_works").as_path()) {
                            for path in vec {
                                if path.file_name().expect("Failed to find any saved files")
                                    == "saved.json"
                                {
                                    terminal::import_from_default(path.as_path(), &mut vector);
                                }
                            }
                        } else {
                            println!("Failed to get list of entries!");
                        }
                    }
                    _ => continue 'main_loop,
                }
            }
            MainSelect::ListFiles => {
                let current_dir = file_work::get_current_path_buf();
                if let Ok(vec) = file_work::dir(current_dir.join("saved_works").as_path()) {
                    let path = vec
                        .get((terminal::user_select(vec.clone()) - 1) as usize)
                        .expect("Failed to get this path");

                    terminal::import_from_default(path.as_path(), &mut vector);
                } else {
                    println!("Failed to get list of entries!");
                }
            }
            MainSelect::Error => {
                println!("Wrong input!");
                continue 'main_loop;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = DbApiClient::connect(url).await?;

    main_terminal(&mut client).await;

    Ok(())
}
