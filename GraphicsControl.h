#ifndef GRAPHICSCONTROL_H
#define GRAPHICSCONTROL_H


#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Event.hpp>
#include "Sudoku.h"

#define SOURCES "/usr/resources_coding/"

struct debugText {
private:
    sf::Text debugCon[4];
    sf::Font font;

public:
    debugText();
    void add(std::string text);
    void draw(sf::RenderWindow *window);

};

class GraphicsControl {
private:
    sf::RenderWindow *window;
    sf::Font font;
    sf::Text text;
    FIELD *selected = NULL;
    bool hasSelected = false;
    debugText dbg;

public:
    GraphicsControl(){}
    void setWindow(sf::RenderWindow *window); // setting the @param window to draw on later
    void select(FIELD *field);  // selecting the @param FIELD for changing its valuse
    FIELD *getSelected();       // returns the previously selected FIELD
    bool isSelected();          // returns if there is a FIELD selected
    void handleKeyCode(sf::Keyboard::Key pressed); // handling the numkeys pressed
    void createGui();           // TODO: creates the GUI
    void drawGui();             // drawing the gui
    void out(std::string text); // drawing it to the output-console
};




#endif