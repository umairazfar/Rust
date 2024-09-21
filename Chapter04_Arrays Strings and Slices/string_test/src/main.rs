fn main() {
    let mut txt:String = String::from("");
    //////////////////////////////////////////////////////////
    //Preliminary setup
    //////////////////////////////////////////////////////////
    txt += &document_class_tag(11, "a4paper", "sans");
    txt += &modern_cv_style("classic");
    txt += &modern_cv_color("blue");
    txt += &add_user_packages("ModernCV");
    txt += &add_personal_info("firstname", "familyname", "address", "phone", "email@email.com", "http://www.homepage.com"); //be wary of mathematical symblos
    //////////////////////////////////////////////////////////
    //Beginning Document
    //////////////////////////////////////////////////////////
    txt += "\n\n\\begin{document}";
    txt += "\n\n\\maketitle";
    //////////////////////////////////////////////////////////
    //Writing details
    //////////////////////////////////////////////////////////
    txt += &define_education_section();
    txt += &define_work_experience_section();
    txt += &define_publications_section();


    //////////////////////////////////////////////////////////
    //Ending Document
    //////////////////////////////////////////////////////////
    txt += "\n\n\\end{document}";
    
    println!("{}", txt);
    
}

/** Creates the document class tag at the beginning of the tex file 

* font_sizes:     10, 11 or 12

* paper_type:     a4paper, letterpaper, a5paper, legalpaper, executivepaper or landscape

* font_family:    sans or roman 

* returns: String*/

fn document_class_tag(font_size:u32, paper_type:&str, font_family:&str) -> String {
    
    let txt = format!("{}[{}, {}, {}]{}", "\\documentclass", font_size, paper_type, font_family, "{moderncv}");
    return txt;
}

/** Creates the moderncvstyle tag which is used for defining the CV theme

* style:     'casual' (default), 'classic', 'oldstyle' and 'banking'
* returns: String*/

fn modern_cv_style(style:&str) -> String {
    let txt = format!("{}{{{}}}", "\n\n\\moderncvstyle", style);
    return txt;
}

/** Creates the moderncvcolor tag which is used for defining the CV color

* color:     'blue' (default), 'orange', 'green', 'red', 'purple', 'grey' and 'black'
* returns: String*/

fn modern_cv_color(color:&str) -> String {
    let txt = format!("{}{{{}}}", "\n\\moderncvcolor", color);
    return txt;
}

/** Adds the user packages as per the selected theme
 * The themes are: ModernCV
 * returns: String*/

fn add_user_packages(template:&str) -> String {
    let template = template.to_lowercase();
    let mut txt:String = String::from("");
    if template == "moderncv" {
        txt += "\n\n\\usepackage[scale=.85]{geometry}";
        //txt += "\n\\usepackage{hyperref}";//we need to get it to work
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

fn add_personal_info(first_name:&str, family_name:&str, address:&str, mobile:&str, email:&str, homepage:&str) ->String{
    let mut _txt = String::from("");
    if homepage == ""{
        _txt = format!("\n\n\\firstname{{{}}}\n\\familyname{{{}}}\n\\address{{{}}}\n\\mobile{{{}}}\n\\email{{{}}}",
        first_name, family_name, address, mobile, email);
    }
    else{
        _txt = format!("\n\n\\firstname{{{}}}\n\\familyname{{{}}}\n\\address{{{}}}\n\\mobile{{{}}}\n\\email{{{}}}\n\\homepage{{{}}}", 
        first_name, family_name, address, mobile, email, homepage);
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
    subject: String,
}

/** Adds individual education information to a vector*/

fn add_education_info(start_date:&str, end_date:&str, institution:&str, city:&str, country:&str, qualification:&str, subject:&str, education_info:&mut Vec<EducationInfo>){
    let education_item:EducationInfo = EducationInfo{
        start_date:     String::from(start_date),
        end_date:       String::from(end_date),
        institution:    String::from(institution),
        city:           String::from(city),
        country:        String::from(country),
        qualification:  String::from(qualification),
        subject:        String::from(subject),
    };
    education_info.push(education_item);
}

/** Defines the education section by using the vector values*/

fn define_education_section() -> String{
    let mut all_education: Vec<EducationInfo> = Vec::new();//This vector will be passed as argument

    //temperory data to be removed later
    //**********************************
    add_education_info("2008", "2012", "University", "City", "Country", "BS", "BS Subject", &mut all_education);
    add_education_info("2014", "2016", "University", "City", "Country", "MS", "MS Subject", &mut all_education);
    add_education_info("2018", "2022", "University", "City", "Country", "PhD", "PhD Subject", &mut all_education);
    //**********************************
    let mut txt:String = String::from("\n\\vspace{-1mm}");
    txt+= &format!("\n\n\\section{{{}}}", "Education");
    for education_info in all_education.iter() {
        txt += &format!("\n\\cventry{{{}--{}}}{{{}, {}, {}}}{{{}}}{{}}{{}}{{}}",
        education_info.start_date, education_info.end_date, education_info.institution, 
        education_info.city, education_info.country, education_info.qualification,);
        txt += "\n{\\textit{";
        txt += &format!("{{{}}}", education_info.subject);
        txt += "}{}";
    }
    return txt;
}

struct WorkInfo{
    start_date: String,
    end_date: String,
    position: String,
    workplace: String,
    city: String,
    country: String,    
    job_info: String,
    tasks_done: Vec<String>,
}

/** Adds individual work information to tasks_done vector*/
fn add_work_tasks(tasks_done:&mut Vec<String>, task:String){
    tasks_done.push(task);
}

/** Adds individual work information that will go into the Experience section */
fn add_work_info(start_date:&str, end_date:&str, position:&str, workplace:&str, city:&str, country:&str, job_info:&str, tasks_done:&mut Vec<String>, work_info:&mut Vec<WorkInfo>){
    //temperory data to be removed later, this data would already be in the tasks_done vector
    //**********************************
    add_work_tasks(tasks_done, String::from("Task 1"));
    add_work_tasks(tasks_done, String::from("Task 2"));
    add_work_tasks(tasks_done, String::from("Task 3"));
    //**********************************
    
    let work_item:WorkInfo = WorkInfo{
        start_date: String::from(start_date),
        end_date:   String::from(end_date),
        position:   String::from(position),
        workplace:  String::from(workplace),
        city:       String::from(city),
        country:    String::from(country),        
        job_info:   String::from(job_info),
        tasks_done: tasks_done.to_vec(),
    };
    work_info.push(work_item);
}

/** Defines the work experience section in LaTeX by using the vector values*/

fn define_work_experience_section() -> String{
    let mut all_experience: Vec<WorkInfo> = Vec::new();//This vector will be passed as argument
    let mut tasks_done: Vec<String> = Vec::new();//only present temporarily for fake data, remove it later
    //temperory data to be removed later this data will already be in the all_experience vector
    //**********************************
    add_work_info("2020", "2021", "Position1", "Workplace1", "City", "Country", "Job Info", &mut tasks_done, &mut all_experience);
    add_work_info("2021", "2022", "Position2", "Workplace2", "City", "Country", "Job Info", &mut tasks_done, &mut all_experience);
    //temperory data to be removed later
    //**********************************
    let mut txt:String = String::from("\n\\vspace{-3mm}");
    txt+= &format!("\n\n\\section{{{}}}", "Work Experience");
    for work_info in all_experience.iter() {
        txt += &format!("\n\\cventry{{{}--{}}}{{{}}}{}{{{}}}{}{{{}, {}}}\n{}{}{}",
        work_info.start_date, work_info.end_date, work_info.position, "\\textsc",
        work_info.workplace,"}{}", work_info.city, work_info.country, "{\\textnormal{",work_info.job_info,":");
        txt += "\n\\begin{itemize}";
        for task in work_info.tasks_done.iter() {
            txt += &format!("\n\t{}{}, ", "\\item ",task);
        }
        txt += "\n\\end{itemize}}\n}";
    }
    return txt;
}

/** Adds publication entry*/
fn add_publication(publications_done:&mut Vec<String>, task:&str, url:&str){
    let mut txt:String = String::from(task);
    if url != ""{
        txt += &format!("{}", "\\url{"); 
        txt += &format!("{}", url);
        txt += &format!("{}", "}");
    }
    txt += &format!("{}", "}}{}{}{}{}");
    publications_done.push(txt);
}

fn define_publications_section() -> String{
    let mut publications_done: Vec<String> = Vec::new();

    //temperory data to be removed later, this data would already be in the publications_done vector
    add_publication(&mut publications_done, "Publication 1", "https://www.google.com");
    add_publication(&mut publications_done, "Publication 2", "https://www.cnn.com");
    add_publication(&mut publications_done, "Publication 3", "https://www.bbc.com");

    let mut txt:String = String::from("\n\\vspace{-3mm}");
    txt+= &format!("\n\n\\section{{{}}}", "Publications");

    let mut i:i32 = 1;

    for publication in publications_done.iter() {
        txt += &format!("{}", "\n\\vspace{1mm}");
        txt += &format!("{}{}.{}{}", "\n\\cventry{", &i.to_string(), "}{\\textnormal{", publication);
        i+=1;
    }
    return txt;
}