use std::io;
use std::io::{ Read, ErrorKind, BufRead, BufReader };
use std::fs::File;
use std::cmp::Ordering;

struct Question {
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
        let file = match File::open("questions.csv") {
            Ok(f) => f,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) =>  panic!("Tried to create file but there was a problem: {:?}", e),
                },
                other_error => panic!("There was a problem opening the file {:?}", other_error)
            }
        };

        let mut result = Vec::new();

        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let values = line.unwrap().split(";");


            result.push(Question {

            });
        }

        result;
    }
}



impl QuestionRepository for StaticQuestionRepository {
    fn load(&self) -> Vec<Question> {
        vec![
            Question {
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
            },
            Question {
                anwser: 3,
                question: String::from("Question test2"),
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

    let mut right = 0;

    for question in questions {
        println!("{}",question.question);

        for option in question.options {
            println!("{} - {}",option.id, option.text);
        };

        let guess = get_number();

        match guess.cmp(&question.anwser) {
            Ordering::Equal => { 
                println!("You are right!!!");
                right = right + 1;
            },
            _ => println!("You are wrong :(")
        }
    }

    println!("You right {} questions", right);
}


fn get_number() -> i32 {
    loop {
        print!("Choose a answers: ");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid number, try again");
                continue
            }
        };
    }
}