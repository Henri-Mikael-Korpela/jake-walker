#include <chrono>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <string>
#include <thread>

typedef int32_t I32;

struct PrintSettings{
    bool const visible;
    I32 const delay;
};

namespace PrintCallbacks{
    PrintSettings normal(char const& c){
        switch(c){
            case '.':
            case ':':
                return { true, 350 };
            default:
                return { true, 75 };
        }
    }
    PrintSettings paragraph_change(char const& c){
        return { true, 75 };
    }
    PrintSettings question(char const& c){
        return { true, 100 };
    }
    PrintSettings title(char const& c){
        return { true, 200 };
    }
}

void delay(I32 delayInMilliseconds){
    std::this_thread::sleep_for(std::chrono::milliseconds(delayInMilliseconds));
}
void print(char const *value){
    printf("%s", value);
}
void print(char const *value, PrintSettings (*callback)(char const& c)){
    auto len = strlen(value);

    for(decltype(len) i = 0; i != len; ++i){
        auto const settings = callback(value[i]);

        if(settings.visible){
            printf("%c", value[i]);
            fflush(stdout);
        }
        if(i + 1 != len){
            delay(settings.delay);
        }
    }

    fflush(stdout);
}
template<I32 Size>
void printQuestion(char const *question, std::array<char const*, Size>&& answers){
    print(question, &PrintCallbacks::question);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    for(int i = 0; i != answers.size(); ++i){
        print(std::to_string(i + 1).c_str());
        print(". ");
        print(answers[i]);
        print("\n");
    }

    print("\n");
}

int main(int argc, char* argv[]){
    print("JAKE WALKER\0", &PrintCallbacks::title);
    delay(500);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    print("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.\0", &PrintCallbacks::normal);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    print("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.\0", &PrintCallbacks::normal);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    print("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...\0", &PrintCallbacks::normal);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    print("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientiest. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.\0", &PrintCallbacks::normal);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    print("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.\0", &PrintCallbacks::normal);
    print("\n\n\0", &PrintCallbacks::paragraph_change);

    printQuestion<2>("What do you do?\0", {
        "Stand up and start walking\0",
        "Continue sitting on the beach\0"
    });

    return 0;
}