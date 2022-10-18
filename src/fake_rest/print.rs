use crate::server::request::Request;


pub fn format_for_print(request: &Request) {
    let mut query_strings = String::new();
    if request.query_strings.is_empty() {
        query_strings.push_str("\n-      -- Empty --");
    }else {
        for query in request.query_strings.iter() {
            let query_style = format!(
                "\n-       -{} = {}", 
                query.0, 
                query.1
            );
            query_strings.push_str(&query_style);
        }
    }
    
    let mut headers = String::new();
    for header in request.headers.iter(){
        let header_style = format!(
            "\n-      -{} : {}", 
            header.0, 
            header.1
        );
        headers.push_str(&header_style);
    }
    println!();
    println!("------------------------ Start Request-------------------------");
    let printable = format!(
        "-- Version: {}\n-- Type: {}\n-- Path: {}\n-- Query Strings:{}\n-- Headers:{}", 
        request.version, 
        request.method, 
        request.uri, 
        query_strings, headers
    );
    println!("{}", printable);
    println!("------------------------ End  Request-------------------------");
}