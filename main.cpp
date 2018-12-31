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


    std::cout << "events: " << sf::Event::KeyPressed << std::endl;
    std::cout << "events: " << sf::Event::MouseButtonPressed << std::endl;


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
                    if (sf::Mouse::isButtonPressed(sf::Mouse::Left) ) {
                        control.select(sud.getClicked(mousePos.x, mousePos.y));
                        control.testClicked(mousePos.x, mousePos.y);
                    }
                    break;
                case sf::Event::MouseMoved:
                    break;
                default:
                    std::cout << "uncovered event: " << event.type << " " << event.key.code << std::endl;
            }
        }

        // drawing the graphics, i.e. the gui and the Sudoku TODO: GUI

        window.clear();

        control.drawGui();

        sud.draw(&window);

        window.display();
        if (Frame % 20 == 0)
            std::cout << "Frame: " << Frame << std::endl;
        Frame++;

        std::this_thread::__sleep_for(std::chrono::seconds(0), std::chrono::milliseconds(30) );

        // std::this_thread::sleep_for(std::chrono::milliseconds(30) );
    }

    return 0;
}
