#include <iostream>
#include <SFML/Graphics.hpp>
#include "GraphicsControl.h"
#include "Sudoku.h"

int main() {
    std::cout << "Forward, Sudoku Solver!" << std::endl;

    sf::RenderWindow window(sf::VideoMode(550, 550), "Sudoku Solver");

    GraphicsControl control;
    control.setWindow(&window);
    control.createGui();

    Sudoku sud;

    int Frame = 0;

    sf::Vector2i mousePos;

    while (window.isOpen() ) {

        mousePos = sf::Mouse::getPosition(window);


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


        window.clear();

        control.drawGui();

        sud.draw(&window);

        window.display();
        std::cout << "Frame: " << Frame << std::endl;
        Frame++;
    }

    return 0;
}