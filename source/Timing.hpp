#ifndef _JAKE_HEADER_TIMING
#define _JAKE_HEADER_TIMING

inline void delay(I32 const& delay_in_milliseconds){
    std::this_thread::sleep_for(std::chrono::milliseconds(delay_in_milliseconds));
}

#endif // _JAKE_HEADER_TIMING