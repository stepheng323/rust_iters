use std::fmt::format;

fn main() {

    fn print_element (elements: &[String]) {
        elements
            .iter()
            .map(|el| format!("{} {}", el, el))
            .for_each(|el| println!("{}", el))
    }


    fn shorten_element (elements: &mut Vec<String>) {
        elements.iter_mut().for_each(|el| el.truncate(1))

    }

    fn to_uppercase (elements: &mut[String]) -> Vec<String> {
        elements.iter().map(|el| el.to_uppercase()).collect()
    }

    fn move_elements (list_a: Vec<String>, list_b: &mut Vec<String>)  {
        list_a.into_iter().for_each(|el| list_b.push(el));
    }

    fn explode (elements: &[String])  -> Vec<Vec<String>>{
        elements
            .iter()
            .map(|el| el.chars().map(|c| c.to_string())
            .collect()
            ).collect()
    }

    fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
        elements
            .iter()
            .find(|el| el.contains(search))
            .map_or(String::from(fallback), |el| el.to_string())
    }

    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green")
    ];

    // print_element(&colors[1..3]);
    // shorten_element(&mut colors);

    // let mut color_iters = colors.iter();

    // println!("{:#?}", color_iters.next());
    // println!("{:#?}", color_iters.next());
    // println!("{:#?}", colors);

    // let uppercased = to_uppercase(& mut colors);

    // println!("{:#?}", uppercased)
    // let mut destination = vec![];
    //
    // move_elements(colors, &mut destination);
    //
    // println!("{:#?}", destination)

    // let exploded = explode(&colors);

    let result = find_color_or(&colors, "bl987", "black");
    println!("{:#?}", result)

}
