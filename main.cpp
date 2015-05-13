#include <iostream>
#include <SFML/Graphics.hpp>
#include <thread>
#include "GraphicsControl.h"
#include "Sudoku.h"

int main() {
    std::cout << "Forward, Sudoku Solver!" << std::endl;

    sf::RenderWindow window(sf::VideoMode(550, 550), "Sudoku Solver");

    // initializing

    GraphicsControl control;
    control.setWindow(&window);
    control.createGui();

    // creating the Sudoku, the FrameCounter and the mousePosition

    Sudoku sud(&control);

    int Frame = 0;

    sf::Vector2i mousePos;

    while (window.isOpen() ) {

        mousePos = sf::Mouse::getPosition(window);

        sf::Event event;

        // when there are events that need to be handled, they are

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
                    } else
                        control.handleKeyCode(event.key.code);
                case sf::Event::MouseButtonPressed:
                    if (sf::Mouse::isButtonPressed(sf::Mouse::Left) )
                        control.select(sud.getClicked(mousePos.x, mousePos.y) );
                    break;
                default:
                    std::cout << "uncovered event" << std::endl;
            }
        }

        // drawing the graphics, i.e. the gui and the Sudoku TODO: GUI

        window.clear();

        control.drawGui();

        sud.draw(&window);

        window.display();
        std::cout << "Frame: " << Frame << std::endl;
        Frame++;

        std::this_thread::sleep_for(std::chrono::milliseconds(100) );
    }

    return 0;
}