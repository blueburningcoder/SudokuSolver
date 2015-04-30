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
    FIELD() {}
    sf::RectangleShape Tile;
    FIELD *Neighbours[4] = {};
    NUMBERS num = NOTHING;
    std::vector<bool> possible;
    sf::Font font;
    sf::Text Nums[10];
    Sudoku *sudoku;
    int Index = -1;

    // initiating the field with all necessary information for later reference
    // @param x and y: location of the tile, @param font: previously loaded font,
    // @param Cluster: the num of the cluster, @param sud: pointer to the Sudoku,
    // @param index: unique ID to find different tiles
    void init(int x, int y, sf::Font font, int Cluster, Sudoku *sud,
                int index);

    void draw(sf::RenderWindow *window); // drawing the field
    void update(); // updates the field TODO: multithreading

    void setNum(int number); // setting the @param number
    void removePossible(NUMBERS alreadyNum); // removing the possibility of the @param alreadyNum
    void setPossible(NUMBERS possNum); // setting possNum possible
    NUMBERS getNum(); // @return: the current Num

    void setNeighbour(int dir, FIELD *next); // setting the @param next Neighbour in @param dir
    bool isNeighbour(int dir); // @return if there is Neighbour in @param dir
    FIELD *getNeighbour(int dir); // @return the Neighbour in @param dir

    bool isInside(int x, int y); // @return if the @param x and y coordinates are inside the field

    int getClusterNum(); // @return the number of the cluster

private:
    bool isInit = false; // if the field is initiated yet
    int xPos, yPos;     // the relative coordinates
    int ClusterNum;
};


class Sudoku {
private:
    int x = 50, y = 50; // own coordinates
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;
    sf::Font font;

    void setNeighbours(); // setting the neighbours of the fields
public:

    Sudoku();
    void draw(sf::RenderWindow *window); // drawing all the fields on the @param window
    void setNum(int i, int j, int number); // setting to the field at @param i and j the @param number
    FIELD *getClicked(int x, int y); // @return the field that got clicked (if there is one) or NULL
    void Update(); // updating all the fields
    // FIELD *getNextFromCluster(int index, int Cluster); // @return the next field starting from @param index from the @param Cluster
    // int getLowestIndexInCluster(int Cluster); // @return the lowest index in the @param cluster

};








#endif