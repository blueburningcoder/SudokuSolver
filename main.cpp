#include <iostream>
#include <TGUI/TGUI.hpp>


int main() {
    std::cout << "Hello, World!" << std::endl;

    sf::RenderWindow window(sf::VideoMode(200, 200), "Sudoku Solver");
    tgui::Gui gui(window);

    int Frame = 0;

    while (window.isOpen() ) {


        sf::Event event;

        while (window.pollEvent(event))
        {
            switch(event.type) {
                case sf::Event::Closed:
                    window.close();
                    break;
                case sf::Event::KeyPressed:
                    // exit the window if 'Escape' is pressed
                    if(event.key.code == sf::Keyboard::Escape) {
                        std::cout << "Escape" << std::endl;
                        window.close();
                    }
                default:
                    std::cout << "uncovered event" << std::endl;
            }

            gui.handleEvent(event);
        }

        tgui::Callback callback;
        while (gui.pollCallback(callback))
        {
            // catches the callback of the TGUI
            switch (callback.id) {
                default:
                    std::cout << "uncought callback: " << std::to_string(callback.id) << std::endl;
                    break;
            }
        }


        window.clear();
        gui.draw();
        window.display();
        std::cout << "Frame: " << Frame << std::endl;
        Frame++;
    }

    return 0;
}