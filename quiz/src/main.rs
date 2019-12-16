struct Question {
    id: i32,
    question: String,
    options: Vec<QuestionOption>,
    anwser: i32
}

struct QuestionOption {
    id: i32,
    text: String
}

trait QuestionRepository {
    fn load(&self) -> Vec<Question>;
}


enum QuestionRepositoryType {
    File,
    Static
}

// trait QuestionRepositoryFactory {
//     fn create(repositoryType: QuestionRepositoryType) -> Box<dyn QuestionRepository> {
//         match repositoryType {
//             QuestionRepositoryType::Static => StaticQuestionRepository,
//             QuestionRepositoryType::File => FileQuestionRepository,
//         }
//     }
// }


struct FileQuestionRepository { }

struct StaticQuestionRepository { }


impl QuestionRepository for FileQuestionRepository {
    fn load(&self) -> Vec<Question> {
        panic!("NOT IMPLEMENT")
    }
}



impl QuestionRepository for StaticQuestionRepository {
    fn load(&self) -> Vec<Question> {
        vec![
            Question {
                id: 1,
                anwser: 2,
                question: String::from("Question test"),
                options: vec![QuestionOption {
                    id: 1,
                    text: String::from("test 1")
                }, 
                QuestionOption {
                    id: 2,
                    text: String::from("test 2")
                },
                QuestionOption {
                    id: 3,
                    text: String::from("test 3")
                },
                QuestionOption {
                    id: 4,
                    text: String::from("test 4")
                }]
            }
        ]
    }
}



fn main() {

    let repository = StaticQuestionRepository { };
    let questions = repository.load();

    for question in questions {
        println!("{}",question.question);

        for option in question.options {
            println!("{} - {}",option.id, option.text);
        }
    }
}
