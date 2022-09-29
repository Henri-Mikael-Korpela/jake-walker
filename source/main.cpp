#include <chrono>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <string>
#include <thread>

#include "types.hpp"
#include "Timing.hpp"
#include "Print.hpp"

struct AnswerOption{
    char const *text;
    void (*callback)();
};

void fail_game(){
    print("GAME OVER\n\0", &Print::Callbacks::normal);
}
template<I32 Size>
AnswerOption const* const get_answer_by_user_input(std::array<AnswerOption, Size> const& answers){
    I32 answer_number;
    scanf("%i", &answer_number);

    if(answer_number >= 1 && answer_number <= answers.size()){
        return &answers[answer_number - 1];
    }
    else{
        return nullptr;
    }
}
template<I32 Size>
inline void handle_answer(std::array<AnswerOption, Size> const& answers){
    wait_for_answer:
    auto const answer = get_answer_by_user_input<2>(answers);

    if(answer == nullptr){
        print("Invalid answer was given. Please try again: ");
        goto wait_for_answer;
    }
    else{
        print("\n\n\0", &Print::Callbacks::paragraph_change);
        answer->callback();
    }
}
template<I32 Size>
void print_question(char const *question, std::array<AnswerOption, Size> const& answers){
    print_enable_color(Print::Color::Green);

    print(question, &Print::Callbacks::question);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    for(int i = 0; i != answers.size(); ++i){
        print(std::to_string(i + 1).c_str());
        print(". ");
        print(answers[i].text);
        print("\n");
    }

    print_disable_color();

    print("\n");
}

namespace Actions{
    void die_on_beach(){
        print("You stayed on the beach doing nothing. You thought someone would come to rescue you. But no one came.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);

        print("You died of starvation.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);

        fail_game();
    }
    void go_alongside_river(){
        print("Unsure of what these dark objects were, Jake decided not to approach them. He saw a river nearby and decided to walk alongside it.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);
    }
    void go_to_remains_of_plane(){
        print("Intrigued, Jake took more steps towards it. He thought he would find something useful there. He saw remains of a plane.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);
    }
    void start_walking(){
        print("Jake started to walk along the coastline. Jake could feel the cold in his hands. As he was walking, he noticed how cloudy it was in the darkness. Jake wanted to find any sign of civilization: maybe fire, maybe light. There was nothing.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);

        print("A couple of minutes later Jake could see something. He saw some darks objects.\0", &Print::Callbacks::normal);
        print("\n\n\0", &Print::Callbacks::paragraph_change);

        std::array<AnswerOption, 2> what_to_do_answers = {
            AnswerOption{
                "Go to them.\0",
                &Actions::go_to_remains_of_plane
            },
            AnswerOption{
                "Avoid them.\0",
                &Actions::go_alongside_river
            }
        };
        print_question<2>("What do you do?\0", what_to_do_answers);
        handle_answer<2>(what_to_do_answers);
    }
}

int main(int argc, char* argv[]){
    print("JAKE WALKER\0", &Print::Callbacks::title);
    delay(500);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    print("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.\0", &Print::Callbacks::normal);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    print("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.\0", &Print::Callbacks::normal);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    print("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...\0", &Print::Callbacks::normal);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    print("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientiest. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.\0", &Print::Callbacks::normal);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    print("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.\0", &Print::Callbacks::normal);
    print("\n\n\0", &Print::Callbacks::paragraph_change);

    std::array<AnswerOption, 2> what_to_do_answers = {
        AnswerOption{
            "Stand up and start walking.\0",
            &Actions::start_walking
        },
        AnswerOption{
            "Continue sitting on the beach.\0",
            &Actions::die_on_beach
        }
    };
    print_question<2>("What do you do?\0", what_to_do_answers);
    handle_answer<2>(what_to_do_answers);

    return 0;
}