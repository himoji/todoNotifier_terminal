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
                println!("{}", terminal::export_works(&vector));
            }
            MainSelect::PrintReadable => {
                for work in &vector{
                    println!("{work}");
                }
            }
            MainSelect::Error => {
                println!("Something went wrong!");
                continue 'main_loop
            }
        }
    }
}