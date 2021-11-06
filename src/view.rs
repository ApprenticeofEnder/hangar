use requestty::{Question, Answers};

pub fn menu(){
    // TODO: Create the overall menu
}

pub fn hangar_create_menu() -> Answers{
    let questions: Vec<Question> = vec![
        Question::input("hangar_name")
            .message("What is your new hangar's name")
            .build(),
        Question::input("hangar_desc")
            .message("What is the description of your new hangar")
            .default("Default Hangar.")
            .build()
    ];

    let answers: Answers = requestty::prompt(questions).unwrap();
    answers
}