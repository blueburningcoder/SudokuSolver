#ifndef SUDOKU_H
#define SUDOKU_H


enum NUMBERS {

    NOTHING,
    ONE,   TWO,   THREE,
    FOUR,  FIVE,  SIX,
    SEVEN, EIGHT, NINE

};

struct FIELD {
    sf::RectangleShape Tile;
    NUMBERS num = NOTHING;
    std::vector<NUMBERS> possible;
};



class Sudoku {
private:
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;

public:
    Sudoku();

    void draw(sf::RenderWindow *window);

};








#endif