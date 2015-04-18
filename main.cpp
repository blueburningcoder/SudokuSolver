#include <iostream>
#include <SFML/Graphics.hpp>
#include "GraphicsControl.h"
#include "Sudoku.h"

int main() {
    std::cout << "Hello, World!" << std::endl;

    sf::RenderWindow window(sf::VideoMode(550, 550), "Sudoku Solver");

    GraphicsControl control;
    control.setWindow(&window);
    control.createGui();

    Sudoku sud;

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
        }


        window.clear();

        // TODO: draw stuff

        control.drawGui();

        sud.draw(&window);

        window.display();
        std::cout << "Frame: " << Frame << std::endl;
        Frame++;
    }

    return 0;
}