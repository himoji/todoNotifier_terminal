mod file_work;
mod work;
mod terminal;

use work::Work;
use crate::terminal::MainSelect;
use crate::work::WorkParams;

fn main() {
    let max_size: u8 = 10;
    let mut vector = Vec::with_capacity(max_size as usize);

    'main_loop: loop {

        match terminal::select_print() {
            MainSelect::NewWork => {
                let work_params: Vec<WorkParams> = terminal::input_create_work_params();
                vector.push(Work::from_vec(work_params));
            }
            MainSelect::EditWork => {
                //call work_edit terminal 
            }
            MainSelect::ExportWorks => {
                println!("{}", terminal::export_works(&vector));
            }
            MainSelect::Error => {
                println!("Something went wrong!");
                continue 'main_loop
            }
        }
    }
}