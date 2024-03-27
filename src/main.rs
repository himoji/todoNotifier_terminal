mod file_work;
mod work;
mod terminal;

use work::Work;
use crate::terminal::MainSelect;
use crate::work::WorkParams;

fn main() {
    let max_size: usize = 10;
    let mut vector = Vec::with_capacity(max_size);

    'main_loop: loop {

        match terminal::select_print() {
            MainSelect::NewWork => {
                let work_params: Vec<WorkParams> = terminal::input_create_work_params();
                vector.push(Work::from_vec(work_params));
            }
            MainSelect::EditWork => {
                println!("There are {} works in the array.", vector.len());
                let selected_work: usize = terminal::user_input("Select:");
                let work  = vector.get_mut(selected_work).unwrap();
                let change = terminal::input_edit_work_params();
                
                work.edit(change);
            }
            MainSelect::ExportWorks => {
                let path_buf = terminal::user_input_path_buf();
                terminal::export_works(&vector, path_buf);
            }
            MainSelect::PrintReadable => {
                for work in &vector{
                    println!("{work}");
                }
            }
            MainSelect::ImportFromJSON => {
                let path_buf = terminal::user_input_path_buf();
                
                let file = file_work::read_file(path_buf.as_path());
                
                match file {
                    Ok(s) => {
                        vector.append(&mut Work::from_vec_string(s));
                    }
                    Err(e) => { 
                        println!("Failed to import from JSON! {e}");
                        continue 'main_loop
                    }
                }
            }
            MainSelect::Error => {
                println!("Something went wrong!");
                continue 'main_loop
            }
        }
    }
}