#ifndef GRAPHICSCONTROL_H
#define GRAPHICSCONTROL_H


#include <SFML/Graphics/RenderWindow.hpp>

#define SOURCES "/usr/resources_coding/"



class GraphicsControl {
private:
    sf::RenderWindow *window;
    sf::Font font;
    sf::Text text;

public:
    GraphicsControl(){}
    void setWindow(sf::RenderWindow *window);
    void createGui();
    void drawGui();
};




#endif