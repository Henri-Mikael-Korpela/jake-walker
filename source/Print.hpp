#ifndef _JAKE_HEADER_PRINT
#define _JAKE_HEADER_PRINT
#include "types.hpp"
#include "Timing.hpp"

namespace Print{
    enum class Color : I32{
        Green
    };

    struct Settings{
        bool const visible;
        I32 const delay;
    };

    namespace Callbacks{
        Print::Settings normal(char const& c){
            switch(c){
                case '.':
                case ':':
                    return { true, 350 };
                default:
                    return { true, 20 };
            }
        }
        Print::Settings paragraph_change(char const& c){
            return { true, 75 };
        }
        Print::Settings question(char const& c){
            return { true, 100 };
        }
        Print::Settings title(char const& c){
            return { true, 200 };
        }
    }
}

inline void print(char const *value){
    printf("%s", value);
}
void print(char const *value, Print::Settings (*callback)(char const& c)){
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
inline void print_disable_color(){
    printf("\033[0m");
}
inline void print_enable_color(Print::Color const&& color){
    switch(color){
        case Print::Color::Green:
            printf("\033[0;32m");
            break;
    }
}

#endif // _JAKE_HEADER_PRINT