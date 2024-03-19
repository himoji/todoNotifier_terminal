pub fn user_select(string_show: &str) -> u8 {
    let mut select;
    println!("{}", string_show);
    std::io::stdin().read_line(&mut select).expect("TODO: panic message");
    let mut select: u8 = select.parse().unwrap();
    select
    
}
pub fn select_print() {
    let select = user_select("Select:\n1)New work\n2)Edit work\n3)Export Year\n");
}