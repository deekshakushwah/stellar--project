#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Question {
    pub question_id: u64,
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_option: u32,
}

#[contracttype]
pub enum QuizKey {
    Question(u64),
    Count,
}

#[contracttype]
pub enum RewardKey {
    Reward(u64),
    Balance(Address),
}

#[contract]
pub struct QuizAndRewardSystem;

#[contractimpl]
impl QuizAndRewardSystem {
    // Add a new question to the quiz
    pub fn add_question(env: Env, question_text: String, options: Vec<String>, correct_option: u32) -> u64 {
        let mut count = env.storage().instance().get(&QuizKey::Count).unwrap_or(0);
        count += 1;

        let question = Question {
            question_id: count,
            question_text,
            options,
            correct_option,
        };

        env.storage().instance().set(&QuizKey::Question(count), &question);
        env.storage().instance().set(&QuizKey::Count, &count);

        count
    }

    // Answer a question and reward if correct
    pub fn answer_question(env: Env, question_id: u64, user: Address, selected_option: u32) -> bool {
        let question: Question = env.storage().instance().get(&QuizKey::Question(question_id)).expect("Question not found");

        if question.correct_option == selected_option {
            // Reward the user
            let mut balance: i128 = env.storage().instance().get(&RewardKey::Balance(user.clone())).unwrap_or(0);
            balance += 10; // Reward amount

            env.storage().instance().set(&RewardKey::Balance(user.clone()), &balance);

            true // Correct answer, reward given
        } else {
            false // Incorrect answer, no reward
        }
    }

    // View the balance of a user
    pub fn view_balance(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&RewardKey::Balance(user)).unwrap_or(0)
    }

    // View all questions in the quiz
    pub fn view_all_questions(env: Env) -> Vec<Question> {
        let mut questions = Vec::new(&env);
        let mut count = env.storage().instance().get(&QuizKey::Count).unwrap_or(0);

        while count > 0 {
            if let Some(question) = env.storage().instance().get(&QuizKey::Question(count)) {
                questions.push_back(question);
            }
            count -= 1;
        }

        questions
    }
}
