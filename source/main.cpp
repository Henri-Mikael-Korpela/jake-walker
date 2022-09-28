#include <chrono>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <thread>

typedef int32_t I32;

void delay(I32 delayInMilliseconds){
    std::this_thread::sleep_for(std::chrono::milliseconds(delayInMilliseconds));
}
void print(char const *value){
    printf("%s", value);
}
void print(char const *value, I32 delayInMilliseconds){
    auto len = strlen(value);

    for(decltype(len) i = 0; i != len; ++i){
        printf("%c", value[i]);
        
        if(i + 1 != len){
            fflush(stdout);
            delay(delayInMilliseconds);
        }
    }

    fflush(stdout);
}

int main(int argc, char* argv[]){
    print("JAKE WALKER\0", 200);
    delay(500);
    print("\n\n\0", 1000);

    print("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.\0", 75);
    print("\n\n\0", 1000);

    print("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.\0", 75);
    print("\n\n\0", 1000);

    print("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...\0", 75);
    print("\n\n\0", 1000);

    print("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientiest. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.\0", 75);
    print("\n\n\0", 1000);

    print("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.\0", 75);
    print("\n\n\0", 1000);

    print("What do you do?", 100);
    print("\n\n\0", 1000);

    print("1. Stand up and start walking\n");
    print("2. Continue sitting on the beach\n");
    print("\n");

    return 0;
}