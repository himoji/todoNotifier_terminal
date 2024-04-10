mod file_work;
mod work;
mod terminal;
mod db;

use std::thread;
use work::Work;
use terminal::MainSelect;

const MAX_SIZE: usize = 10;

fn main_terminal() {
let mut vector: Vec<Work> = Vec::with_capacity(MAX_SIZE);

'main_loop: loop {
match terminal::select_print() {
MainSelect::NewWork => {
vector.push(Work::from_vec(terminal::input_create_work_params()));
},

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
terminal::export_works(&vector);
}
MainSelect::PrintReadable => {
for work in &vector {
println!("{}", work);
}
}
MainSelect::ImportFromJSON => {
let select = terminal::user_select("1)From custom path\n2)Default saved file");
match select {
1=>{
let path_buf = terminal::user_input_path_buf();
let add_works = Work::from_vec_string(file_work::read_file(path_buf.as_path()));
vector.extend(add_works);
},
2 => {
let current_dir = file_work::get_current_path_buf();
if let Ok(vec) = file_work::dir(current_dir.join("../../saved_works").as_path()) {
for path in vec{
if path.file_name().expect("Failed to find any saved files") == "saved.json" {
let add_works = Work::from_vec_string(file_work::read_file(path.as_path()));
vector.extend(add_works);
}
}
} else {
println!("Failed to get list of entries!");
}
}
_ => {continue 'main_loop}
}

}
MainSelect::ListFiles => {
let current_dir = file_work::get_current_path_buf();
if let Ok(vec) = file_work::dir(current_dir.join("../../saved_works").as_path()) {
for (index, path) in vec.iter().enumerate() {
println!("#{}. {}", index+1, path.display());
}
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

fn main_db() {
    let _ = db::main();
}
fn main() {
    // thread::spawn(|| {
    //     main_terminal();
    // });
    thread::spawn(|| {
            main_db();
        });
    
}