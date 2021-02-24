#include <iostream>
#include <windows.h>

char window_name[] = "Grand Theft Auto V";

bool check_window(){
    HWND foreground = GetForegroundWindow();
    if (foreground)
    {
        char window_title[256];
        GetWindowText(foreground, window_title, 256);

        return strcmp(window_title, window_name) == 0;
    }

    return false;
}

void do_activity() {
    if (check_window()) {
        INPUT mouseScrollInput;
        mouseScrollInput.type = INPUT_MOUSE;
        mouseScrollInput.mi.dx = 0;
        mouseScrollInput.mi.dy = 0;
        mouseScrollInput.mi.dwFlags = MOUSEEVENTF_WHEEL;
        mouseScrollInput.mi.time = 0;
        mouseScrollInput.mi.dwExtraInfo = 0;
        mouseScrollInput.mi.mouseData = WHEEL_DELTA * 5 * -1;

        SendInput(1, &mouseScrollInput, sizeof(mouseScrollInput));
    }
}

int main() {
    std::cout << "This program will work only when GTA V is in focus." << std::endl;
    std::cout << "To turn it off, simply close this window." << std::endl;

    while (true) {
        do_activity();
        Sleep(10000);
    }
}
