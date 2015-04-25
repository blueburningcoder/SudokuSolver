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
    std::vector<bool> possible;
    sf::Font font;
    sf::Text Nums[10];

    void init(int x, int y, sf::Font font);
    void draw(sf::RenderWindow *window);
    void setNum(NUMBERS num);

private:
    bool isInit = false;
    int xPos, yPos;
};


class Sudoku {
private:
    int x = 50, y = 50;
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;
    sf::Font font;

public:
    Sudoku();

    void draw(sf::RenderWindow *window);

};








#endif