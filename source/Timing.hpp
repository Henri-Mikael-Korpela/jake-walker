#ifndef _JAKE_HEADER_TIMING
#define _JAKE_HEADER_TIMING

inline void delay(I32 delayInMilliseconds){
    std::this_thread::sleep_for(std::chrono::milliseconds(delayInMilliseconds));
}

#endif // _JAKE_HEADER_TIMING