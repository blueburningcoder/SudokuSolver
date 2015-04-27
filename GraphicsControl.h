#ifndef GRAPHICSCONTROL_H
#define GRAPHICSCONTROL_H


#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Event.hpp>
#include "Sudoku.h"

#define SOURCES "/usr/resources_coding/"



class GraphicsControl {
private:
    sf::RenderWindow *window;
    sf::Font font;
    sf::Text text;
    FIELD *selected = NULL;
    bool hasSelected = false;

public:
    GraphicsControl(){}
    void setWindow(sf::RenderWindow *window);
    void select(FIELD *field);
    FIELD *getSelected();
    bool isSelected();
    void handleKeyCode(sf::Keyboard::Key pressed);
    void createGui();
    void drawGui();
};




#endif