#ifndef SUDOKU_H
#define SUDOKU_H


#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/Text.hpp>
#include <iostream>

#define SOURCES "/usr/resources_coding/"

class Sudoku;
class GraphicsControl;

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
    bool possible[9];
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

    void autoSolve(); // automatically solves itself over time
    void lookForMissing();
    void lookForMust();

    bool isPossible(NUMBERS num); // returns if @param num is possible or not
    void setNum(int number); // setting the @param number
    void removePossible(NUMBERS alreadyNum); // removing the possibility of the @param alreadyNum
    void setPossible(NUMBERS possNum); // setting possNum possible
    NUMBERS getNum(); // @return: the current Num

    void setNeighbour(int dir, FIELD *next); // setting the @param next Neighbour in @param dir
    bool isNeighbour(int dir); // @return if there is Neighbour in @param dir
    FIELD *getNeighbour(int dir); // @return the Neighbour in @param dir

    bool isInside(int x, int y); // @return if the @param x and y coordinates are inside the field

    int getClusterNum(); // @return the number of the cluster

    void reset();       // resetting the field now

private:
    bool isInit = false; // if the field is initiated yet
    int xPos, yPos;     // the relative coordinates
    int ClusterNum;     // the ClusterID
};


class Sudoku {
private:
    int x = 50, y = 50; // own coordinates
    std::vector<std::vector<FIELD> > fields;
    sf::RectangleShape Ground;
    sf::Font font;

    GraphicsControl *control;

    void setNeighbours(); // setting the neighbours of the fields

public:
    Sudoku(GraphicsControl* graphicsControl);
    void draw(sf::RenderWindow *window); // drawing all the fields on the @param window
    void setNum(int i, int j, int number); // setting to the field at @param i and j the @param number
    FIELD *getClicked(int x, int y); // @return the field that got clicked (if there is one) or NULL
    void Update(); // updating all the fields
    FIELD *getNextFromCluster(int index, int Cluster); // @return the next field starting from @param index from the @param Cluster
    int getLowestIndexInCluster(int Cluster); // @return the lowest index in the @param cluster
    bool alreadyInCluster(int Cluster, NUMBERS num); // @return if the number is in the Cluster already
    bool alreadyInColumnOrRow(FIELD* field, NUMBERS num); // @return if the number is a Column or row
    void out(std::string text);      // logging to the console
    void reset();       // resetting the Sudoku to a completely empty one

    bool autosolve = true;
    int wait = 0;
};








#endif
