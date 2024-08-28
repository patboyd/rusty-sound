use std::io;
use console::Term;

struct Question {
    prompt: &'static str,
    options: Vec<&'static str>,
    correct_index: usize,
}

struct QuestionResponse {
    attempts: i32,
    correct: bool,
}

fn get_question_char( index:usize ) -> char {
    match index {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => '?',        
    }
}

fn get_index_for_char( c :char ) -> usize {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        _ => 100,        
    }
}

fn show_question( question: &Question, question_number :usize ) -> QuestionResponse {
    println!("\n---------------------------------------------------------------");
    println!("{}) {}\n", question_number, question.prompt );

    let mut n=0;
    while n < question.options.len(){
        println!( "   [{}] {}", get_question_char(n), question.options[n]);
        n += 1;
    }
    println!("");
    let mut attempts = 0;
    let mut prompt = true;
    let mut choice :usize = 100;
    while prompt {
        let term = Term::stdout();
        let response = term.read_char();
        let guess = response.unwrap();
        choice = get_index_for_char( guess );
        attempts += 1;
        if choice > question.options.len() {
            println!("Invalid response: {}", guess);
            tone(110.0, 0.2);
        } else {
            prompt = false;
        }
    }
    QuestionResponse {
        attempts,
        correct: choice == question.correct_index,
    }
}

use rand::Rng;
fn random_emoji() -> char{
    '🍔'
}

fn line( value :f32, delay: f32, direction: bool ) {
    print!("{} ", if direction { "🚀" } else { "👽"} );
    print!("{:.8} ", delay);
    let mut i = 0.0;
    while i < value {
        print!("~");
        i += 20.0;
    }
    print!("{}", random_emoji());
    print!("{}hz", value as i32);
    println!("");

    tone(value, delay);
}



fn main(){
    // quiz_main();
    // random_main();
    // main_control();
    synth_main();

}

fn get_tone( c :char ) -> f32 {
    match c {

        '/' => 15000.0,

        '\'' => 9000.0,
        'a' => 212.0,
        's' => 321.0,
        'd' => 120.0,
        'f' => 243.0,
        'g' => 220.0,
        'h' => 350.0,
        'j' => 460.0,
        'k' => 129.0,
        ';' => 278.0,
        'p' => 251.0,
        'o' => 229.0,
        'i' => 329.0,
        'u' => 244.0,
        'y' => 384.0,
        't' => 294.0,
        'r' => 285.0,
        'e' => 364.0,
        'w' => 244.0,
        'q' => 259.0,
        'm' => 382.0,
        'n' => 274.0,
        'b' => 294.0,
        'v' => 267.0,
        'c' => 471.0,
        'x' => 315.0,
        'z' => 320.0,
        
        'A' => 145.0,
        'S' => 155.0,
        'D' => 165.0,
        'F' => 175.0,
        'G' => 185.0,
        'H' => 190.0,
        'J' => 195.0,
        'K' => 200.0,
        'L' => 205.0,
        ':' => 210.0,
        '\"' => 215.0,

        _ => 120.0,
    }
}

fn synth_main() {
    let term = Term::stdout();
    let mut direction = true;
    let mut delay = 0.1;
    let mut key = ' ';
    while key != '+' {
        
        let response = term.read_char();
        key = response.unwrap();
        println!("[{}]", key);

        // let mut rng = rand::thread_rng();
        // let hz = rng.gen::<f32>() * 2400.0;
        
        let hz = get_tone(key);
        line(hz, delay, direction);
    }
}


fn main_control(){
    let mut rng = rand::thread_rng();

    let mut direction = true;
    let mut delay = 0.05;
    let mut run = true;

    let mut hz = 70.0;
    let mut max = 2400.0;
    while run {
        
        if hz > max {
            direction = false;
            if max < 400.0 {
                max = 2400.00;
            } else {
                max -= 200.0;
            }
        }
        if hz <=70.0 {
            direction = true;
        }

        if delay <=0.0 {
            direction = true;
        }
        // if direction {
        //     delay += 0.2;
        // } else {
        //     if delay < 0.05 {
        //         delay -= 0.005;
        //     } else {
        //         delay -= 0.07;
        //     }
        // }
        line(hz, delay, direction);
        
        if direction {
            hz += 20.0;
            // delay += 0.005;
        } else {
            hz -= 100.0;
            // delay -= 0.005;
        }
    }
}

fn random_main(){
    let mut rng = rand::thread_rng();

    let mut direction = true;
    let mut delay = 0.1;
    let run = true;

    while run {
        let hz = rng.gen::<f32>() * 2400.0;
        
        if delay > 1.0 {
            direction = false;
        }
        if delay < 0.0001 {
            direction = true;
        }
        // if direction {
        //     delay += 0.05;
        // } else {
        //     if delay < 0.05 {
        //         delay -= 0.005;
        //     } else {
        //         delay -= 0.07;
        //     }
        // }
        line(hz, delay, direction);
    }
}

fn quiz_main() {
    let questions : Vec<Question> = vec![
        Question{ 
            prompt: "Welcome to the quiz",
            options: vec![
                "Continue",
            ],
            correct_index: 1,
        },
        Question{ 
            prompt: "How many teeth does a full grown adult have ?",
            options: vec![
                "43",
                "25",
                "32",
            ],
            correct_index: 3,
        },
        Question{ 
            prompt: "What is 2+2 ?",
            options: vec![
                "22",
                "4",
                "2",
                "100",
            ],
            correct_index: 2,
        },
        Question{ 
            prompt: "What does a creeper do ?",
            options: vec![
                "Helps you out",
                "Eats your goat",
                "Steals your stuff",
                "Baked you bread",
            ],
            correct_index: 3,
        },
        Question{ 
            prompt: "What is 999 + 1 ?",
            options: vec![
                "9991",
                "100",
                "1",
                "One thousand",
            ],
            correct_index: 4,
        },
    ];

    let mut num_correct = 0;
    let mut i = 0;
    while i < questions.len() {
        let q = &questions[i];
        let resp = show_question( q, i+1 );
        if resp.correct {
            println!(" >>> Correct!");
            tone(880.0, 0.8);
            num_correct += 1;
        } else {
            tone(220.0, 0.8);
            println!(" >>> Incorrect!");
        }
       
        i += 1;
    }
    println!("");
    score_quiz( questions, vec![], num_correct );
    println!("");
}

fn score_quiz( questions: Vec<Question>, _results: Vec<QuestionResponse>, num_correct: i32 ) {
    let num_questions = questions.len() as f32;
    let score: f32 = (num_correct as f32 / num_questions )  * 100.0;
    println!("You scored: ({}/{}) {:.0}%", num_correct, num_questions, score);
}

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn tone(hz :f32, duration: f32){
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(hz).take_duration(Duration::from_secs_f32(duration)).amplify(0.50);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}