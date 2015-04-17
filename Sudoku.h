#ifndef SUDOKU_H
#define SUDOKU_H


#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/Text.hpp>
#include <iostream>

#define SOURCES "/usr/resources_coding/"


enum NUMBERS {
    NOTHING,
    ONE,   TWO,   THREE,
    FOUR,  FIVE,  SIX,
    SEVEN, EIGHT, NINE

};

struct FIELD {
    FIELD(){};
    sf::RectangleShape Tile;
    NUMBERS num = NOTHING;
    std::vector<NUMBERS> possible;
    sf::Text text;

    void init(int x, int y, sf::Font font);
    void draw(sf::RenderWindow *window);
};

class Sudoku {
private:
    int x = 50, y = 50;
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;

public:
    Sudoku();

    void draw(sf::RenderWindow *window);

};








#endif