fn main() {
    let mut txt:String = String::from("");
    //////////////////////////////////////////////////////////
    //Preliminary setup
    //////////////////////////////////////////////////////////
    txt += &document_class_tag(11, "a4paper", "sans");
    txt += &modern_cv_style("classic");
    txt += &modern_cv_color("blue");
    txt += &add_user_packages("ModernCV");
    txt += &add_personal_info("firstname", "familyname", "address", "phone", "email@email.com", "http://www.homepage.com", "homepage_text_to_show");
    //////////////////////////////////////////////////////////
    //Beginning Document
    //////////////////////////////////////////////////////////
    txt += "\n\\begin{document}";
    txt += "\n\\maketitle";
    //////////////////////////////////////////////////////////
    //Writing details
    //////////////////////////////////////////////////////////
    txt += &define_section("Education");


    //////////////////////////////////////////////////////////
    //Ending Document
    //////////////////////////////////////////////////////////
    txt += "\n\\end{document}";
    
    println!("{}", txt);
    
}

/** Creates the document class tag at the beginning of the tex file 

* font_sizes:     10, 11 or 12

* paper_type:     a4paper, letterpaper, a5paper, legalpaper, executivepaper or landscape

* font_family:    sans or roman 

* returns: String*/

fn document_class_tag(font_size:u32, paper_type:&str, font_family:&str) -> String {
    
    let txt = format!("{}[{}, {}, {}]", "\\documentclass", font_size, paper_type, font_family);
    return txt;
}

/** Creates the moderncvstyle tag which is used for defining the CV theme

* style:     'casual' (default), 'classic', 'oldstyle' and 'banking'
* returns: String*/

fn modern_cv_style(style:&str) -> String {
    let txt = format!("{}[{}]", "\n\\moderncvstyle", style);
    return txt;
}

/** Creates the moderncvcolor tag which is used for defining the CV color

* color:     'blue' (default), 'orange', 'green', 'red', 'purple', 'grey' and 'black'
* returns: String*/

fn modern_cv_color(color:&str) -> String {
    let txt = format!("{}[{}]", "\n\\moderncvcolor", color);
    return txt;
}

/** Adds the user packages as per the selected theme
 * The themes are: ModernCV
 * returns: String*/

fn add_user_packages(template:&str) -> String {
    let template = template.to_lowercase();
    let mut txt:String = String::from("");
    if template == "moderncv" {
        txt += "\n\\usepackage[scale=.85]{geometry}";
        txt += "\n\\usepackage{hyperref}";
    }
    return txt;
}

/** Adds personal information 
 * first name
 * family name
 * address
 * mobile
 * email
 * homepage
*/

fn add_personal_info(first_name:&str, family_name:&str, address:&str, mobile:&str, email:&str, homepage:&str, url_text_to_show:&str) ->String{
    let mut _txt = String::from("");
    if homepage == ""{
        _txt = format!("\n\\firstname{{{}}}\n\\familyname{{{}}}\n\\address{{{}}}\n\\mobile{{{}}}\n\\email{{{}}}",
        first_name, family_name, address, mobile, email);
    }
    else{
        _txt = format!("\n\\firstname{{{}}}\n\\familyname{{{}}}\n\\address{{{}}}\n\\mobile{{{}}}\n\\email{{{}}}\n\\homepage{{{}}}{{{}}}", 
        first_name, family_name, address, mobile, email, homepage, url_text_to_show);
    }
    return _txt;
}

struct EducationInfo{
    start_date: String,
    end_date: String,
    institution: String,
    city: String,
    country: String,
    qualification: String,
}



fn add_education_info(start_date:&str, end_date:&str, institution:&str, city:&str, country:&str, qualification:&str, education_info:&mut Vec<EducationInfo>){
    let education_item:EducationInfo = EducationInfo{
        start_date:     String::from(start_date),
        end_date:       String::from(end_date),
        institution:    String::from(institution),
        city:           String::from(city),
        country:        String::from(country),
        qualification:  String::from(qualification),
    };
    education_info.push(education_item);
}

fn define_section(title:&str) -> String{
    let mut all_education: Vec<EducationInfo> = Vec::new();
    add_education_info("2008", "2012", "University", "City", "Country", "BS", &mut all_education);
    add_education_info("2014", "2016", "University", "City", "Country", "MS", &mut all_education);
    add_education_info("2018", "2022", "University", "City", "Country", "PhD", &mut all_education);
    let mut txt:String = String::from("\n\\vspace{-1mm}");
    txt+= &format!("\n\\section{{{}}}", title);
    for education_info in all_education.iter() {
        txt += &format!("\n\\cventry{{{}}}--{{{}}}{{{}}}{{{}}}{{{}}}{{{}}}{{}}",
        education_info.start_date, education_info.end_date, education_info.institution, 
        education_info.city, education_info.country, education_info.qualification,);
        //txt += "{}";
    }
    return txt;
}

