mod file_work;
mod work;
mod terminal;

use work::Work;
use crate::terminal::MainSelect;

const MAX_SIZE: usize = 10;

fn main() {
    let mut vector: Vec<Work> = Vec::with_capacity(MAX_SIZE);

    'main_loop: loop {
        match terminal::select_print() {
            MainSelect::NewWork => {
                terminal::input_create_work_params()
                    .and_then(|work_params| {
                        vector.push(Work::from_vec(work_params));
                        Ok(())
                    }).expect("Failed to create the work");
            }
            MainSelect::EditWork => {
                println!("There are {} works in the array.", vector.len());
                
                if let Ok(selected_work) = terminal::user_input::<usize>("Select:") {
                    if selected_work < vector.len() {
                        if let Some(work) = vector.get_mut(selected_work).map(|w| w as &mut Work)
                            {
                            terminal::input_edit_work_params()
                                .and_then(|c| {
                                    work.edit(c);
                                    Ok(())
                                }).expect("TODO: panic message");
                            }
                        }
                }
            }
            MainSelect::ExportWorks => {
                terminal::export_works(&vector);
            }
            MainSelect::PrintReadable => {
                for work in &vector {
                    println!("{}", work);
                }
            }
            MainSelect::ImportFromJSON => {
                let path_buf = terminal::user_input_path_buf();
                if let Ok(add_works) = file_work::read_file(path_buf.as_path()).map(|add_work| Work::from_vec_string(add_work)) {
                    vector.extend(add_works);
                }
                continue 'main_loop
            }
            MainSelect::ListFiles => {
                if let Ok(current_dir) = file_work::get_current_path() {
                    if let Ok(vec) = file_work::dir(current_dir.join("saved_works").as_path()) {
                        for (index, path) in vec.iter().enumerate() {
                            println!("#{}. {}", index, path.display());
                        }
                    } else {
                        println!("Failed to get list of entries!");
                    }
                } else {
                    println!("Failed to get current path!");
                }
            }
            MainSelect::Error => {
                println!("Wrong input!");
            }
        }
    }
}
