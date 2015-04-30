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
    void setWindow(sf::RenderWindow *window); // setting the @param window to draw on later
    void select(FIELD *field);  // selecting the @param FIELD for changing its valuse
    FIELD *getSelected();       // returns the previously selected FIELD
    bool isSelected();          // returns if there is a FIELD selected
    void handleKeyCode(sf::Keyboard::Key pressed); // handling the numkeys pressed
    void createGui();           // TODO: creates the GUI
    void drawGui();             // drawing the gui
};




#endif