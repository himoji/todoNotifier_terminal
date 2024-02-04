
mod file_work;
mod week;

mod prelude {
    pub use crate::week::Work;
    pub use crate::week::Week;
    pub use crate::week::Year;
}
use prelude::*;
use crate::file_work::{create_file, delete_file, get_current_path, write_into_file};

fn main() {
    delete_file(&get_current_path().unwrap().join("json.txt"));

    let work = Work::create_work("Homework".to_string(), "todo algebra".to_string(), 600);

    let monday = vec![work.clone(), work.clone()];

    let week = Week{
        week_number: 0,
        week_year: 0,
        monday:    monday.clone(),
        tuesday:   monday.clone(),
        wednesday: monday.clone(),
        thursday:  monday.clone(),
        friday:    monday.clone(),
        saturday:  monday.clone(),
        sunday:    monday.clone(),
    };

    let week_vec = vec![week.clone(), week.clone(), week.clone()];

    let year = Year{ weeks: week_vec };

    let json_string = serde_json::to_string(&year.save()).unwrap();

    println!("{}", json_string);

    let file = create_file(&get_current_path().unwrap(), "json.txt").expect("bombaclat");

    write_into_file(file, serde_json::to_string(&work.humanify_time()).unwrap());
}




