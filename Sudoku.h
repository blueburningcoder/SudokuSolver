#ifndef SUDOKU_H
#define SUDOKU_H


#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/Text.hpp>
#include <iostream>

#define SOURCES "/usr/resources_coding/"

class Sudoku;

enum NUMBERS {
    NOTHING,
    ONE,   TWO,   THREE,
    FOUR,  FIVE,  SIX,
    SEVEN, EIGHT, NINE

};


struct FIELD {
    FIELD(){};
    sf::RectangleShape Tile;
    FIELD *Neighbours[4] = {};
    NUMBERS num = NOTHING;
    std::vector<bool> possible;
    sf::Font font;
    sf::Text Nums[10];
    Sudoku *sudoku;

    void init(int x, int y, sf::Font font, int Cluster, Sudoku *sud);
    void draw(sf::RenderWindow *window);
    void update();

    void setNum(int number);
    void removePossible(NUMBERS alreadyNum);
    void setPossible(NUMBERS possNum);
    NUMBERS getNum();

    void setNeighbour(int dir, FIELD *next);
    bool isNeighbour(int dir);
    FIELD *getNeighbour(int dir);

    bool isInside(int x, int y);

    int getClusterNum();

private:
    bool isInit = false;
    int xPos, yPos;
    int ClusterNum;
};


class Sudoku {
private:
    int x = 50, y = 50;
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;
    sf::Font font;

    void setNeighbours();
public:

    Sudoku();
    void draw(sf::RenderWindow *window);
    void setNum(int i, int j, int number);
    FIELD *getClicked(int x, int y);
    void Update();

};








#endif